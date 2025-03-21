use iced::Task;
use rfd::FileDialog;

use crate::{action::Action, config::Config};

use super::{Message, Request, Settings};

impl Settings {
    pub fn update(&mut self, message: Message, config: &mut Config) -> Action<Message, Request> {
        match message {
            Message::UpdateFolderPath => {
                if let Some(path) = FileDialog::new()
                    .set_directory(config.screenshot_dir.clone())
                    .pick_folder()
                {
                    config.screenshot_dir = path;

                    self.screenshot_dir = config.display_screenshot_dir();
                }
            }
            Message::OpenFolder => {
                config.open_screenshot_folder();
            }
            Message::AnimateTheme(event) => {
                self.theme.update(event);
            }
            Message::ToggleTheme => {
                config.theme = config.theme.toggle();
                return Task::done(Message::AnimateTheme(config.theme.clone().into())).into();
            }
            Message::UpdateNotifications(notifications) => {
                self.notifications = notifications;
                config.notifications = notifications;
            }
            Message::RequestExit => {
                return Action::requests([Request::Exit]);
            }
        }
        Action::none()
    }
}
