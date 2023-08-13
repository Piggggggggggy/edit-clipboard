use std::{io::Write, process::Stdio};

pub struct TextTransformFactory;
impl TextTransformFactory {
    pub fn parse(arg: &str) -> Result<Box<dyn TextTransform>, &str> {
        let mut arg = arg.split_whitespace();
        Ok(match arg.next().unwrap() {
            "c" => Box::new(CollapseWhitespace),
            "s" => Box::new(StripWhitespace),
            "u" => Box::new(External("uwuify")),
            a => return Err(a),
        })
    }
}
pub trait TextTransform {
    fn process(&self, text: &mut String);
}

pub struct CollapseWhitespace;
impl TextTransform for CollapseWhitespace {
    fn process(&self, text: &mut String) {
        *text = text.as_mut_str().split_whitespace().collect()
    }
}

pub struct StripWhitespace;
impl TextTransform for StripWhitespace {
    fn process(&self, text: &mut String) {
        *text = text
            .split_whitespace()
            .map(|sub| sub.to_owned() + " ")
            .collect::<String>()
            .trim()
            .to_owned()
    }
}

// string to executable
pub struct External(&'static str);
impl TextTransform for External {
    fn process(&self, text: &mut String) {
        let mut proc = std::process::Command::new(self.0)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap_or_else(|_| panic!("Could not find {} binary", self.0));
        proc.stdin
            .as_mut()
            .unwrap()
            .write_all(text.as_bytes())
            .unwrap();

        *text = String::from_utf8_lossy(&proc.wait_with_output().unwrap().stdout).to_string()
    }
}
