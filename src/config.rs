use std::{
    fs::{
        DirBuilder,
        File,
    },
    io::{
        Read,
        Write,
    },
    path::PathBuf,
};

use anyhow::{
    Context,
    Result,
};
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    organize_type::OrgranizeMode,
    theme::Theme,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub folder_path: PathBuf,
    pub organize_mode: OrgranizeMode,
    pub show_notification: bool,
    pub theme: Theme,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            folder_path: Self::default_screenshot_dir(),
            organize_mode: Default::default(),
            show_notification: true,
            theme: Default::default(),
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

    /// Gets the config file path and ensures the folder exists.
    fn get_config_file() -> Result<(File, bool)> {
        let config_folder = dirs::config_dir()
            .with_context(|| "Failed to get config folder path")?
            .join("Capter");
        let config_path = config_folder.join("capter.toml");

        if !config_folder.exists() {
            DirBuilder::new().recursive(true).create(&config_folder)?;
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

    /// Returns a truncated version of the screenshot folder path for UI.
    pub fn truncate_folder_path(&self) -> String {
        let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from(""));

        let mut display_path = self.folder_path.clone();
        if let Ok(stripped) = self.folder_path.strip_prefix(&home_dir) {
            display_path = PathBuf::from("~").join(stripped);
        }

        let display_str = display_path.to_string_lossy();
        if display_str.len() > 20 {
            format!("...{}", &display_str[display_str.len() - 17..])
        } else {
            display_str.into_owned()
        }
    }

    /// Provides the default screenshots folder.
    fn default_screenshot_dir() -> PathBuf {
        let screenshot_dir = dirs::picture_dir()
            .unwrap_or_else(|| PathBuf::from("Pictures"))
            .join("Capter");

        DirBuilder::new()
            .recursive(true)
            .create(&screenshot_dir)
            .expect("Failed to create screenshots folder");

        screenshot_dir
    }

    pub fn open_screenshot_folder(&self) {
        opener::open(&self.folder_path).expect("Failed to open screenshot folder");
    }
}
