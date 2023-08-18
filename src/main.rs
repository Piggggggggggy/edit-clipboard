// #![windows_subsystem = "windows"]
mod args;
mod config;
mod preprocesser;

use clap::Parser;
use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use inquire::Confirm;
use preprocesser::processor;
use preprocesser::transform::TextTransformFactory;
use std::process::exit;
use std::process::Stdio;

fn main() {
    let editor_config = config::EditorConfig::new(
        simple_home_dir::expand_tilde("~/.config/edit_clipboard.toml").unwrap(),
    );

    let mut ctx: ClipboardContext = ClipboardProvider::new().expect("could not get provider");
    // Create a temp file with clipboard contents prompting if it is non-text or undefined.
    let confirm_overwrite =
        Confirm::new("clipboard is undefined or non-text, do you want to overwrite it?");
    let mut text = if let Ok(clipboard_contents) = ctx.get_contents() {
        clipboard_contents
    } else if confirm_overwrite.prompt().unwrap() {
        String::from("")
    } else {
        std::process::exit(0)
    };

    // apply preprocesser(s)
    let mut processor = processor::Processor::new();

    let args = args::Args::parse();

    // parse arguments
    for filter_flag in args.filter.unwrap_or_default().chars() {
        processor.add_op(
            TextTransformFactory::parse(&filter_flag.to_string()).unwrap_or_else(|e| {
                eprintln!("flag {e} is not an option");
                exit(1);
            }),
        );
    }
    processor.apply(&mut text);

    // Create tempfile for editor
    let tempfile = temp_file::with_contents(text.as_bytes());

    let launch_terminal: bool = editor_config.terminal.is_some();

    // Creates a process to edit the file
    let mut editor_process = if launch_terminal {
        let terminal = editor_config.terminal.unwrap();
        std::process::Command::new(terminal.proccess)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .args(terminal.args)
            .args([
                editor_config.editor,
                tempfile.path().to_str().unwrap().to_string(),
            ])
            .spawn()
            .unwrap()
    } else {
        std::process::Command::new(editor_config.editor)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .args([tempfile.path().to_str().unwrap()])
            .spawn()
            .unwrap()
    };
    // Wait for editor to exit -> i.e. editing is done
    editor_process.wait().expect("Editor Crashed");
    // Sets clipboard contents to file
    if Confirm::new("Do you want to save to clipboard?")
        .with_default(true)
        .prompt_skippable()
        .unwrap_or_else(|_| exit(0))
        .unwrap_or(false)
    {
        let mut clip = std::fs::read_to_string(tempfile.path()).unwrap();

        if editor_config.trim {
            clip = clip.trim().to_string();
        }

        ctx.set_contents(clip)
            .expect("could not set clipboard contents");
    }
}
