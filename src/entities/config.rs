use iced_anim::{Spring, SpringEvent};
use serde::{Deserialize, Serialize};

use crate::entities::theme::Theme;

#[derive(Debug, Clone)]
pub struct Config {
    pub theme: Spring<Theme>,
    pub directory: String,
}

/// The configuration that gets serialized to disk.
/// This is distinct to avoid serializing animated values.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredConfig {
    #[serde(default)]
    pub theme: Theme,
    #[serde(default = "Config::default_path")]
    pub directory: String,
}

#[derive(Debug)]
pub struct ConfigureWindow {
    pub config: Config,
    pub path: String,
}

#[derive(Debug, Clone)]
pub enum ConfigEvent {
    UpdateFolderPath,
    OpenFolder,
    UpdateTheme(SpringEvent<Theme>),
    RequestExit,
}
