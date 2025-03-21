use iced_anim::{Animated, Event};

use crate::theme::Theme;

mod init;
mod update;
mod view;

#[derive(Debug)]
pub struct Settings {
    pub theme: Animated<Theme>,
    notifications: bool,
    screenshot_dir: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    UpdateFolderPath,
    OpenFolder,
    AnimateTheme(Event<Theme>),
    ToggleTheme,
    UpdateNotifications(bool),
    RequestExit,
}

pub enum Request {
    Exit,
}
