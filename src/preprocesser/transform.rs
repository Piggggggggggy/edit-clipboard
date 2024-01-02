use clap::ValueEnum;
use std::{io::Write, process::Stdio};
use symspell::{self, SymSpell, UnicodeiStringStrategy};
use uniaxe::uniaxe;
use uwuifier;

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum Transformation {
    /// Crunch Whitespace, alias c
    #[value(alias("c"))]
    CollapseWhitespace,
    /// Strip Whitespace, alias s
    #[value(alias("s"))]
    StripWhitespace,
    /// UWUIFY some text, alias u
    #[value(alias("u"))]
    Uwuify,
    /// Add quotes, alias q
    #[value(alias("q"))]
    Quote,
    /// Unicode to Ascii, alias x
    #[value(alias("x"))]
    UnicodeStrip,
    /// Spellcheck, transforms sentence to best guess, alias e
    #[value(alias("e"))]
    SpellCheck,
}
pub struct TextTransformFactory;
impl TextTransformFactory {
    pub fn parse(arg: Transformation) -> Result<Box<dyn TextTransform>, String> {
        Ok(match arg {
            Transformation::CollapseWhitespace => Box::new(CollapseWhitespace),
            Transformation::StripWhitespace => Box::new(StripWhitespace),
            Transformation::Uwuify => Box::new(Uwuify),
            Transformation::Quote => Box::new(Quote),
            Transformation::UnicodeStrip => Box::new(UnicodeStrip),
            Transformation::SpellCheck => Box::new(SpellCheck),
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

pub struct Uwuify;
impl TextTransform for Uwuify {
    fn process(&self, text: &mut String) {
        *text = uwuifier::uwuify_str_sse(text);
    }
}

pub struct SpellCheck;
impl TextTransform for SpellCheck {
    fn process(&self, text: &mut String) {
        let mut symspell: SymSpell<UnicodeiStringStrategy> = SymSpell::default();
        const DICTIONARY: &str = include_str!(r"../../data/frequency_dictionary_en_82_765.txt");
        const BIGRAM: &str = include_str!(r"../../data/frequency_bigramdictionary_en_243_342.txt");
        for line in DICTIONARY.lines() {
            symspell.load_dictionary_line(line, 0, 1, " ");
        }
        for line in BIGRAM.lines() {
            symspell.load_bigram_dictionary_line(line, 0, 2, " ");
        }

        let sugestions = symspell.lookup_compound(text, 2);
        *text = sugestions.first().map(|f| &f.term).unwrap_or(text).clone();
    }
}
