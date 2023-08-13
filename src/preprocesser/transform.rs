use std::{io::Write, process::Stdio};
use uniaxe::uniaxe;

pub struct TextTransformFactory;
impl TextTransformFactory {
    pub fn parse(arg: &str) -> Result<Box<dyn TextTransform>, &str> {
        let mut arg = arg.split_whitespace();
        Ok(match arg.next().unwrap() {
            "c" => Box::new(CollapseWhitespace),
            "s" => Box::new(StripWhitespace),
            "u" => Box::new(External("uwuify")),
            "q" => Box::new(Quote),
            "x" => Box::new(UnicodeStrip),
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

pub struct Quote;
impl TextTransform for Quote {
    fn process(&self, text: &mut String) {
        *text = format!("\"{}\"", text.trim());
    }
}

pub struct UnicodeStrip;
impl TextTransform for UnicodeStrip {
    fn process(&self, text: &mut String) {
        *text = text
            .chars()
            .map(|c| {
                if c == '\u{2018}' || c == '\u{2019}' {
                    '\''
                } else if c == '\u{201C}' || c == '\u{201D}' {
                    '\"'
                } else {
                    c
                }
            })
            .collect();
        let table = uniaxe::lookup::generate_table();
        *text = uniaxe(text, &table);
    }
}
