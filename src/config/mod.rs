use std::{
    fs::{DirBuilder, File},
    io::{Read, Write},
    path::PathBuf,
};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::theme::Theme;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub screenshot_dir: PathBuf,
    pub notifications: bool,
    pub theme: Theme,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            theme: Theme::Light,
            notifications: true,
            screenshot_dir: Self::default_screenshot_dir(),
        }
    }
}

impl Config {
    /// Loads config from file or creates a new one.
    pub fn load() -> Result<(Self, bool)> {
        let (mut config_file, is_newly_created) = Self::get_config_file()?;
        let mut file_content = String::new();

        let _ = config_file.read_to_string(&mut file_content);
        let config = toml::from_str::<Config>(&file_content).unwrap_or_else(|_| {
            let default_config = Self::default();
            default_config
                .save()
                .expect("Failed to save default config");
            default_config
        });

        Ok((config, is_newly_created))
    }

    /// Saves the config to file.
    pub fn save(&self) -> Result<()> {
        let (mut file, _) = Self::get_config_file()?;
        file.set_len(0)?; // Clear existing content
        let contents = toml::to_string(self)?;
        file.write_all(contents.as_bytes())?;
        Ok(())
    }

    /// Gets the config file path and ensures the directory exists.
    fn get_config_file() -> Result<(File, bool)> {
        let config_dir = dirs::config_dir()
            .with_context(|| "Failed to get config directory")?
            .join("Capter");
        let config_path = config_dir.join("capter.toml");

        if !config_dir.exists() {
            DirBuilder::new().recursive(true).create(&config_dir)?;
        }

        let is_newly_created = !config_path.exists();
        let file = File::options()
            .create(true)
            .truncate(false)
            .read(true)
            .write(true)
            .open(&config_path)?;

        Ok((file, is_newly_created))
    }

    /// Returns a shortened version of the screenshot directory for UI.
    pub fn display_screenshot_dir(&self) -> String {
        let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from(""));

        let mut display_path = self.screenshot_dir.clone();
        if let Ok(stripped) = self.screenshot_dir.strip_prefix(&home_dir) {
            display_path = PathBuf::from("~").join(stripped);
        }

        let display_str = display_path.to_string_lossy();
        if display_str.len() > 20 {
            format!("...{}", &display_str[display_str.len() - 17..])
        } else {
            display_str.into_owned()
        }
    }

    /// Provides the default screenshot directory.
    fn default_screenshot_dir() -> PathBuf {
        let screenshot_dir = dirs::picture_dir()
            .unwrap_or_else(|| PathBuf::from("Pictures"))
            .join("Capter");

        DirBuilder::new()
            .recursive(true)
            .create(&screenshot_dir)
            .expect("Failed to create screenshot directory");

        screenshot_dir
    }

    pub fn open_screenshot_folder(&self) {
        opener::open(&self.screenshot_dir).expect("Failed to open screenshot folder");
    }
}
