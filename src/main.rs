// #![windows_subsystem = "windows"]
mod preprocesser;

use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use inquire::Confirm;
use preprocesser::processor;
use preprocesser::transform::TextTransformFactory;
use std::process::exit;
use std::process::Stdio;

// todo make this read in from a config file
struct EditorConfig<'a> {
    terminal_proccess: &'a str,
    terminal_proccess_args: Vec<&'a str>,
    editor_process: &'a str,
}
fn main() {
    const LAUNCH_TERMINAL: bool = false;
    let editor_config = EditorConfig {
        terminal_proccess: "alacritty",
        terminal_proccess_args: vec!["-e"],
        editor_process: "nvim",
    };

    let mut ctx: ClipboardContext = ClipboardProvider::new().expect("could not get provider");
    // Create a temp file with clipboard contents prompting if it is non-text or undefined.
    let confirm_overwrite =
        Confirm::new("clipboard is undefined or non-text, do you want to overwrite it?");
    let mut text = {
        if let Ok(clipboard_contents) = ctx.get_contents() {
            clipboard_contents
        } else if confirm_overwrite.prompt().unwrap() {
            String::from("")
        } else {
            std::process::exit(0)
        }
    };

    // apply preprocesser(s)
    let mut processor = processor::Processor::new();
    // parse arguments
    let args = std::env::args().skip(1).collect::<String>();
    for arg in args.chars() {
        processor.add_op(
            TextTransformFactory::parse(&arg.to_string()).unwrap_or_else(|e| {
                eprintln!("flag {e} is not an option");
                exit(1);
            }),
        );
    }
    processor.apply(&mut text);

    // Create tempfile for editor
    let tempfile = temp_file::with_contents(text.as_bytes());

    // Creates a process to edit the file
    let mut editor_process = if LAUNCH_TERMINAL {
        std::process::Command::new(editor_config.terminal_proccess)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .args(editor_config.terminal_proccess_args)
            .args([
                editor_config.editor_process,
                tempfile.path().to_str().unwrap(),
            ])
            .spawn()
            .unwrap()
    } else {
        std::process::Command::new(editor_config.editor_process)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .args([tempfile.path().to_str().unwrap()])
            .spawn()
            .unwrap()
    };
    // Wait for helix to exit -> i.e. editing is done
    editor_process.wait().expect("Editor Crashed");
    // Sets clipboard contents to file
    if Confirm::new("Do you want to save to clipboard?")
        .with_default(true)
        .prompt_skippable()
        .unwrap_or_else(|_| exit(0))
        .unwrap_or(false)
    {
        ctx.set_contents(std::fs::read_to_string(tempfile.path()).unwrap())
            .unwrap();
    }
}
