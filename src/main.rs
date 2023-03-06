use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use inquire::Confirm;
use std::process::Stdio;

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
    // Creates a helix process to edit the file
    let mut helix = std::process::Command::new("hx")
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .env_clear()
        .arg(tempfile.path())
        .spawn()
        .unwrap();
    // Wait for helix to exit -> i.e. editing is done
    helix.wait().expect("helix should not fail");
    // Sets clipboard contents to file
    ctx.set_contents(std::fs::read_to_string(tempfile.path()).unwrap())
        .unwrap();
}
