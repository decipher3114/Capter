use iced::{
    alignment::Horizontal::Left,
    widget::{
        button, column, container, horizontal_space, row, svg, svg::Handle, text, vertical_space,
    },
    window::Id,
    Alignment::{self, Center},
    ContentFit,
    Length::Fill,
    Task,
};
use iced_anim::SpringEvent;

use crate::{
    assets::{BOLD, SVG_FOLDER_OPEN},
    entities::{
        config::{ConfigEvent, ConfigureWindow},
        style::ButtonClass,
    },
    style::Element,
    AppEvent,
};

impl ConfigureWindow {
    pub fn update(&mut self, id: Id, message: ConfigEvent) -> Task<AppEvent> {
        match message {
            ConfigEvent::UpdateFolderPath => {
                self.update_directory();
                Task::done(AppEvent::UpdateConfig(id))
            }
            ConfigEvent::OpenFolder => {
                self.open_directory();
                Task::none()
            }
            ConfigEvent::UpdateTheme(event) => {
                if let SpringEvent::Target(_) = event {
                    self.config.theme.update(event);
                    Task::done(AppEvent::UpdateConfig(id))
                } else {
                    self.config.theme.update(event);
                    Task::none()
                }
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
                    button(
                        svg(Handle::from_memory(SVG_FOLDER_OPEN))
                            .height(0.1)
                            .content_fit(ContentFit::ScaleDown)
                    )
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
                    button(
                        text(self.config.theme.target().to_string())
                            .size(20)
                            .center()
                    )
                    .height(40)
                    .width(160)
                    .on_press(ConfigEvent::UpdateTheme(
                        self.config.theme.target().toggle().into()
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

        let footer_row = |key_bind: &'static str, task: &'static str| {
            let text_size = 18;
            row![
                text(key_bind)
                    .font(BOLD)
                    .size(text_size)
                    .align_y(Center)
                    .width(120),
                horizontal_space().width(5),
                text(":").size(text_size).align_y(Center).width(20),
                horizontal_space().width(Fill),
                text(task).size(text_size).align_y(Center).width(320)
            ]
        };

        let footer = container(
            column![
                text("Keybindings:").size(22).font(BOLD),
                vertical_space().height(10),
                footer_row("Alt+Shift+S", "Capture and Crop Screenshot"),
                footer_row("Alt+Shift+F", "Captures Full Screenshot"),
                footer_row("Alt+Shift+W", "Captures Focused Window"),
            ]
            .width(Fill)
            .padding(10),
        );

        column![header, body, footer].spacing(10).padding(15).into()
    }
}
