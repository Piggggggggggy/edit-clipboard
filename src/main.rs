#![windows_subsystem = "windows"]
use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use inquire::Confirm;
use std::process::Stdio;
const EDITOR_COMMAND: &'static str = "nvim";
fn main() {
    let mut ctx: ClipboardContext = ClipboardProvider::new().expect("could not get provider");
    // Create a temp file with clipboard contents prompting if it is non-text or undefined.
    let tempfile = temp_file::with_contents({
        if let Ok(str) = ctx.get_contents() {
            str
        } else if Confirm::new("clipboard is undefined or non-text, do you want to writeover it?")
            .prompt()
            .expect("prompting failed")
        {
            String::from("")
        } else {
            std::process::exit(0)
        }
        .as_bytes()
    });
    // Creates a helix process to edit the file in alacritty
    let mut helix = std::process::Command::new("alacritty")
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .args(["-e", EDITOR_COMMAND, tempfile.path().to_str().unwrap()])
        .spawn()
        .unwrap();
    // Wait for helix to exit -> i.e. editing is done
    helix.wait().expect("Editor Crashed");
    // Sets clipboard contents to file
    ctx.set_contents(std::fs::read_to_string(tempfile.path()).unwrap())
        .unwrap();
}
