#![windows_subsystem = "windows"]
mod preprocesser;

use crate::preprocesser::Preprocesser;
use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use inquire::Confirm;
use preprocesser::Processor;
use std::process::Stdio;

// todo make this read in from a config file
const EDITOR_COMMAND: &str = "nvim";

fn main() {
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
    let processor = Processor::new();
    processor.apply(&mut text);

    // Create tempfile for editor
    let tempfile = temp_file::with_contents(text.as_bytes());
    // Creates a helix process to edit the file
    let mut editor_process = std::process::Command::new("alacritty")
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .args(["-e", EDITOR_COMMAND, tempfile.path().to_str().unwrap()])
        .spawn()
        .unwrap();
    // Wait for helix to exit -> i.e. editing is done
    editor_process.wait().expect("Editor Crashed");
    // Sets clipboard contents to file
    ctx.set_contents(std::fs::read_to_string(tempfile.path()).unwrap())
        .unwrap();
}
