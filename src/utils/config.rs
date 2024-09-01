use std::{
    env::var_os,
    fs::{DirBuilder, File},
    io::{Read, Write},
    path::Path,
    process::Command,
};

use rfd::FileDialog;

use crate::entities::{config::Config, theme::Theme};

impl Default for Config {
    fn default() -> Self {
        #[cfg(target_os = "windows")]
        let dir = format!(
            "{}{}\\Pictures\\Capter",
            var_os("HOMEDRIVE").unwrap().to_string_lossy(),
            var_os("HOMEPATH").unwrap().to_string_lossy()
        );

        #[cfg(not(target_os = "windows"))]
        let dir = format!("{}/Pictures/Capter", var_os("HOME").unwrap().to_string_lossy());

        Self {
            theme: Theme::default(),
            dir: dir.clone(),
            shortened_path: Self::shorten_path(&dir),
        }
    }
}

impl Config {
    pub fn new() -> (Self, bool) {
        match Self::get_config_file() {
            Ok(mut file) => {
                let mut file_content = String::new();
                let _ = file.read_to_string(&mut file_content).unwrap();
                match toml::from_str::<Config>(&file_content) {
                    Ok(config) => (config, false),
                    Err(_) => {
                        let config = Self::default();
                        Self::write_config(&config, true);
                        (config, true)
                    }
                }
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

    pub fn write_config(&self, truncate: bool) {
        match Self::get_config_file() {
            Ok(mut file) => {
                if truncate {
                    file.set_len(0).unwrap()
                };
                let contents = toml::to_string_pretty(self).unwrap();
                file.write_all(contents.as_bytes()).unwrap();
            }
            Err(_) => println!("Config can't be updated"),
        }
    }

    pub fn toggle_theme(&mut self) {
        self.theme = match self.theme {
            Theme::Light => Theme::Dark,
            Theme::Dark => Theme::Light,
        }
    }

    pub fn update_dir(&mut self) {
        if let Some(path) = FileDialog::new()
            .set_directory(self.dir.clone())
            .pick_folder()
        {
            self.dir = path.to_str().unwrap().to_string();
            self.shortened_path = Self::shorten_path(&self.dir);
        }
    }

    pub fn open_dir(&mut self) {
        #[cfg(target_os = "windows")]
        let cmd = "explorer";
        #[cfg(target_os = "linux")]
        let cmd = "xdg-open";
        #[cfg(target_os = "macos")]
        let cmd = "open";
        Command::new(cmd).arg(&self.dir).spawn().unwrap();
    }

    fn shorten_path(path: &str) -> String {

        #[cfg(target_os = "windows")]
        let home_path = format!(
                "{}{}",
                var_os("HOMEDRIVE").unwrap().to_string_lossy(),
                var_os("HOMEPATH").unwrap().to_string_lossy()
            );

        #[cfg(not(target_os = "windows"))]
        let home_path = format!("{}", var_os("HOME").unwrap().to_string_lossy());

        // Replace the full home path with ~
        let replaced_path = path.replace(&home_path, "~");

        // Shorten the path if it's longer than 20 characters
        if replaced_path.len() > 20 {
            format!("...{}", &replaced_path[replaced_path.len() - 17..])
        } else {
            replaced_path
        }
    }
}
