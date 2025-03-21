use iced::{
    Length,
    theme::{Base, Style},
    widget::Space,
    window::Id,
};

use crate::{
    App, Message,
    theme::{Element, Theme},
    window::AppWindow,
};

impl App {
    pub fn title(&self, id: Id) -> String {
        match self.windows.get(&id) {
            Some(AppWindow::Settings(_)) => String::from("Capter"),
            Some(AppWindow::Capture(_)) => String::from("Capter: Capture"),
            None => String::new(),
        }
    }

    pub fn theme(&self, id: Id) -> Theme {
        match self.windows.get(&id) {
            Some(AppWindow::Settings(config_window)) => config_window.theme.value().clone(),
            _ => self.config.theme.clone(),
        }
    }

    pub fn style(&self, theme: &Theme) -> Style {
        theme.base()
    }

    pub fn view(&self, id: Id) -> Element<Message> {
        match &self.windows.get(&id) {
            Some(AppWindow::Settings(config_window)) => config_window
                .view()
                .map(move |message| Message::Settings(id, message)),
            Some(AppWindow::Capture(capture_window)) => capture_window
                .view()
                .map(move |message| Message::Capture(id, message)),
            None => Space::new(Length::Shrink, Length::Shrink).into(),
        }
    }
}
