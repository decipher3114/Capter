use iced::{
    theme::{Base, Style},
    window::Id,
};

use crate::{
    App, Message,
    theme::{Element, Theme},
    window::AppWindow,
};

impl App {
    pub fn title(&self, _id: Id) -> String {
        String::from("Capter")
    }

    pub fn theme(&self, _id: Id) -> Theme {
        self.config.theme
    }

    pub fn style(&self, theme: &Theme) -> Style {
        theme.base()
    }

    pub fn view(&self, id: Id) -> Element<Message> {
        match &self.windows.get(&id) {
            Some(AppWindow::Settings(settings)) => settings
                .view(&self.config)
                .map(move |message| Message::Settings(id, message)),
            Some(AppWindow::Capture(capture)) => capture
                .view()
                .map(move |message| Message::Capture(id, message)),
            None => unreachable!(),
        }
    }
}
