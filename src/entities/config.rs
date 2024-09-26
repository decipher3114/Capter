use serde::{Deserialize, Serialize};

use crate::entities::theme::Theme;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
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
    ToggleTheme,
    RequestExit,
}
