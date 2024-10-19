use iced::{
    alignment::Horizontal::Left,
    widget::{button, column, container, horizontal_space, row, text},
    window::Id,
    Alignment::{self, Center},
    Length::Fill,
    Task,
};
use iced_anim::{Animation, Spring, SpringEvent};

use crate::{
    app::AppEvent,
    consts::{BOLD, FOLDER_ICON, ICON},
    theme::{button::ButtonClass, Element, Theme},
};

pub mod config;

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

impl ConfigureWindow {
    pub fn update(&mut self, id: Id, message: ConfigEvent) -> Task<AppEvent> {
        match message {
            ConfigEvent::UpdateFolderPath => Task::done(AppEvent::UpdateDirectory(id)),
            ConfigEvent::OpenFolder => Task::done(AppEvent::OpenDirectory),
            ConfigEvent::UpdateTheme(event) => {
                self.theme.update(event);
                Task::none()
            }
            ConfigEvent::RequestExit => Task::done(AppEvent::ExitApp),
        }
    }

    pub fn view(&self) -> Element<ConfigEvent> {
        let header = row![
            text("Capter").size(60).font(BOLD),
            horizontal_space(),
            button(text("Exit").align_x(Center).align_y(Center).size(20))
                .on_press(ConfigEvent::RequestExit)
                .height(40)
                .width(80)
                .class(ButtonClass::Danger)
        ]
        .align_y(Center);

        let body = column![
            container(
                row![
                    text("Screenshots Directory")
                        .align_x(Left)
                        .size(22)
                        .font(BOLD),
                    horizontal_space().width(Fill),
                    button(text(FOLDER_ICON).font(ICON).size(20).center())
                        .height(40)
                        .width(40)
                        .on_press(ConfigEvent::OpenFolder),
                    horizontal_space().width(10),
                    button(text(&self.path).size(20).center())
                        .height(40)
                        .width(250)
                        .on_press(ConfigEvent::UpdateFolderPath)
                ]
                .align_y(Alignment::Center)
                .width(Fill)
                .height(Fill)
                .padding(10)
            )
            .height(80),
            container(
                row![
                    text("App Theme").align_x(Left).size(22).font(BOLD),
                    horizontal_space().width(Fill),
                    button(text(self.theme.target().to_string()).size(20).center())
                        .height(40)
                        .width(160)
                        .on_press(ConfigEvent::UpdateTheme(
                            self.theme.target().toggle().into()
                        ))
                ]
                .align_y(Alignment::Center)
                .width(Fill)
                .height(Fill)
                .padding(10)
            )
            .height(80)
        ]
        .spacing(10);

        let content = column![header, body].spacing(10).padding(15);

        Animation::new(&self.theme, content)
            .on_update(ConfigEvent::UpdateTheme)
            .into()
    }
}
