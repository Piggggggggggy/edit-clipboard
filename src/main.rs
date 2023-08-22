// #![windows_subsystem = "windows"]
mod args;
mod config;
mod preprocesser;

use args::Args;
use clap::Parser;
use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use inquire::Confirm;
use preprocesser::processor::Processor;
use preprocesser::transform::TextTransformFactory;
use std::io;
use std::io::Read;
use std::process::exit;
use std::process::Stdio;

fn main() {
    // parse args
    let args = Args::parse();

    let config_path = if let Ok(path) = std::env::var("EDIT_CLIPBOARD_CONFIG") {
        path.into()
    } else {
        simple_home_dir::expand_tilde("~/.config/edit_clipboard.toml").unwrap()
    };

    if args.config {
        println!("{}", config_path.display());
        exit(0);
    }

    let editor_config = config::EditorConfig::new(config_path);

    let mut clipboard: ClipboardContext =
        ClipboardProvider::new().expect("could not get clipboard provider");

    // get clipboard contents using a new string if it fails or using stdin if provided
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    stdin.read_to_string(&mut buffer).unwrap();

    let mut text = if !buffer.is_empty() {
        buffer
    } else {
        clipboard.get_contents().unwrap_or_default()
    };

    // apply preprocesser(s)
    let mut processor = Processor::new();

    // parse arguments
    for filter_flag in args.filter.unwrap_or_default().into_iter() {
        processor.add_op(
            TextTransformFactory::parse(filter_flag).unwrap_or_else(|e| {
                eprintln!("Error: {e}");
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

        clipboard
            .set_contents(clip)
            .expect("could not set clipboard contents");
    }
}
