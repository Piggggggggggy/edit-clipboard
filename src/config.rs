use std::{fs, path};

#[derive(serde::Deserialize)]
pub struct EditorConfig {
    pub terminal: Option<Terminal>,
    pub editor: String,
    pub trim: bool,
}

#[derive(serde::Deserialize, Clone)]
pub struct Terminal {
    pub proccess: String,
    pub args: Vec<String>,
}
impl EditorConfig {
    pub fn default() -> Self {
        let terminal = Some(Terminal {
            proccess: "alacritty".to_string(),
            args: vec!["-e".to_string()],
        });
        EditorConfig {
            terminal,
            editor: "hx".to_string(),
            trim: false,
        }
    }

    pub fn new(path: path::PathBuf) -> Self {
        if let Ok(file) = fs::read_to_string(&path) {
            toml::from_str(&file).expect("Failed to parse toml")
        } else {
            eprintln!("Failed to read config file: {}", &path.display());
            Self::default()
        }
    }
}
