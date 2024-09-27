use iced_anim::{Spring, SpringEvent};
use serde::{Deserialize, Serialize};

use crate::entities::theme::Theme;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub theme: Theme,
    pub directory: String,
}

#[derive(Debug)]
pub struct ConfigureWindow {
    pub theme: Spring<Theme>,
    pub path: String,
}

#[derive(Debug, Clone)]
pub enum ConfigEvent {
    UpdateFolderPath,
    OpenFolder,
    UpdateTheme(SpringEvent<Theme>),
    RequestExit,
}
