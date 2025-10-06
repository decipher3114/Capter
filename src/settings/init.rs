use crate::{
    config::Config,
    settings::Settings,
};

impl Settings {
    pub fn init(config: &Config) -> Self {
        Self {
            folder_path: config.truncate_folder_path(),
        }
    }
}
