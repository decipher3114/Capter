use super::Settings;
use crate::config::Config;

impl Settings {
    pub fn init(config: &Config) -> Self {
        Self {
            folder_path: config.truncate_folder_path(),
        }
    }
}
