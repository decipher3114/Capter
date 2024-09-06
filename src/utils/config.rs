use std::{
    env::var_os,
    fs::{DirBuilder, File},
    io::{Read, Write},
    path::Path,
    process::Command,
};

use rfd::FileDialog;

use crate::entities::{config::{Config, ConfigureWindow}, theme::Theme};

use super::shorten_path;

impl Default for Config {
    fn default() -> Self {
        Self {
            theme: Theme::default(),
            directory: Config::default_path(),
        }
    }
}

impl Config {
    pub fn new() -> (Self, bool) {
        match Self::get_config_file() {
            Ok(mut file) => {
                let mut file_content = String::new();
                let _ = file.read_to_string(&mut file_content).unwrap();
                let bool = file_content.is_empty();
                let config = match toml::from_str::<Config>(&file_content) {
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

    pub fn get_config_file() -> Result<File, std::io::Error> {
        #[cfg(target_os = "windows")]
        let path = format!(
                "{}{}\\.config\\capter.toml",
                var_os("HOMEDRIVE").unwrap().to_string_lossy(),
                var_os("HOMEPATH").unwrap().to_string_lossy()
            );

        #[cfg(not(target_os = "windows"))]
        let path = format!("{:?}/.config/capter.toml", var_os("HOME").unwrap().to_string_lossy());

        if !Path::new(&path).exists() {
            DirBuilder::new()
                .recursive(true)
                .create(Path::new(&path).parent().unwrap())
                .unwrap();
        }
        return File::options()
            .read(true)
            .create(true)
            .write(true)
            .open(path);
    }

    pub fn update_config(&self) {
        match Self::get_config_file() {
            Ok(mut file) => {
                file.set_len(0).unwrap();
                let contents = toml::to_string(self).unwrap();
                file.write_all(contents.as_bytes()).unwrap();
            }
            Err(_) => println!("Config can't be updated"),
        }
    }


    pub fn default_path() -> String {
        #[cfg(target_os = "windows")]
        let path = format!(
            "{}{}\\Pictures\\Capter",
            var_os("HOMEDRIVE").unwrap().to_string_lossy(),
            var_os("HOMEPATH").unwrap().to_string_lossy()
        );

        #[cfg(not(target_os = "windows"))]
        let path = format!("{}/Pictures/Capter", var_os("HOME").unwrap().to_string_lossy());

        let _ = DirBuilder::new()
            .recursive(true)
            .create(Path::new(&path))
            .unwrap();

        path
    }
}

impl ConfigureWindow {
    pub fn new(config: &Config) -> Self {
        Self {
            config: config.clone(),
            path: shorten_path(config.directory.clone())
        }
    }
    pub fn toggle_theme(&mut self) {
        self.config.theme = match self.config.theme {
            Theme::Light => Theme::Dark,
            Theme::Dark => Theme::Light,
        }
    }

    pub fn update_directory(&mut self) {
        if let Some(path) = FileDialog::new()
            .set_directory(self.config.directory.clone())
            .pick_folder()
        {
            self.config.directory = path.into_os_string().into_string().unwrap();
            self.path = shorten_path(self.config.directory.clone());
        }
    }

    pub fn open_directory(&self) {
        #[cfg(target_os = "windows")]
        let cmd = "explorer";
        #[cfg(target_os = "linux")]
        let cmd = "xdg-open";
        #[cfg(target_os = "macos")]
        let cmd = "open";
        Command::new(cmd).arg(&self.config.directory).spawn().unwrap();
    }

}
