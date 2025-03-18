use std::io::{Read, Write};

use serde::{Deserialize, Serialize};
use utils::{default_path, open_config};

pub mod utils;

use crate::theme::Theme;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub theme: Theme,
    pub directory: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            theme: Theme::Light,
            directory: default_path(),
        }
    }
}

impl Config {
    pub fn new() -> (Self, bool) {
        match open_config() {
            Ok(mut file) => {
                let mut file_content = String::new();
                let _ = file.read_to_string(&mut file_content).unwrap();
                let bool = file_content.is_empty();
                let config: Config = match toml::from_str::<Config>(&file_content) {
                    Ok(config) => config,
                    Err(_) => {
                        let config = Self::default();
                        Self::update_config(&config);
                        config
                    }
                };
                (config, bool)
            }
            Err(_) => (Self::default(), true),
        }
    }

    pub fn update_config(&self) {
        if let Ok(mut file) = open_config() {
            file.set_len(0).unwrap();
            let contents = toml::to_string(self).unwrap();
            file.write_all(contents.as_bytes()).unwrap();
        }
    }
}
