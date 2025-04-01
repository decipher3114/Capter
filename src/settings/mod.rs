use crate::theme::Theme;

mod init;
mod update;
mod view;

#[derive(Debug)]
pub struct Settings {
    pub theme: Theme,
    notifications: bool,
    screenshot_dir: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    UpdateFolderPath,
    OpenFolder,
    ToggleTheme,
    UpdateNotifications(bool),
    RequestExit,
}

pub enum Request {
    Exit,
}
