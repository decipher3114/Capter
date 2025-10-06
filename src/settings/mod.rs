mod init;
mod update;
mod view;

use crate::{
    organize_type::OrgranizeMode,
    theme::Theme,
};

#[derive(Debug)]
pub struct Settings {
    folder_path: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    UpdateFolderPath,
    OpenFolder,
    UpdateTheme(Theme),
    ToggleShowNotification(bool),
    UpdateOrganizeMode(OrgranizeMode),
    RequestExit,
}

pub enum Request {
    Exit,
}
