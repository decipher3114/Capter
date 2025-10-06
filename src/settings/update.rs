use rfd::FileDialog;

use crate::{
    action::Action,
    config::Config,
    settings::{
        Message,
        Request,
        Settings,
    },
};

impl Settings {
    pub fn update(&mut self, message: Message, config: &mut Config) -> Action<Message, Request> {
        match message {
            Message::UpdateFolderPath => {
                if let Some(path) = FileDialog::new()
                    .set_directory(config.folder_path.clone())
                    .pick_folder()
                {
                    config.folder_path = path;

                    self.folder_path = config.truncate_folder_path();
                }
            }
            Message::OpenFolder => {
                config.open_screenshot_folder();
            }
            Message::UpdateTheme(theme) => {
                config.theme = theme;
            }
            Message::ToggleShowNotification(show_notification) => {
                config.show_notification = show_notification;
            }
            Message::UpdateOrganizeMode(organize_type) => {
                config.organize_mode = organize_type;
            }
            Message::RequestExit => {
                return Action::requests([Request::Exit]);
            }
        }
        Action::none()
    }
}
