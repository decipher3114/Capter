use serde::{Deserialize, Serialize};

use crate::entities::theme::Theme;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub theme: Theme,
    pub dir: String,
    pub shortened_path: String,
}

#[derive(Debug, Clone)]
pub enum ConfigEvent {
    UpdateFolderPath,
    OpenFolder,
    ToggleTheme,
    RequestExit,
}
