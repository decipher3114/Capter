use iced::{
    Alignment::Center,
    Length,
    widget::{Button, Column, Container, Row, Space, Text, Toggler},
};
use iced_anim::Animation;

use crate::{
    consts::{BOLD_FONT, FOLDER_ICON_ICON, ICON_FONT},
    theme::{Element, button::ButtonClass},
};

use super::{Message, Settings};

impl Settings {
    pub fn view(&self) -> Element<Message> {
        let header = Row::new()
            .push(Text::new("Capter").size(60).font(BOLD_FONT))
            .push(Space::with_width(Length::Fill))
            .push(
                Button::new(Text::new("Exit").center().size(20))
                    .on_press(Message::RequestExit)
                    .height(40)
                    .width(80)
                    .class(ButtonClass::Danger),
            )
            .align_y(Center);

        let body = Column::new()
            .push(
                Container::new(
                    Row::new()
                        .push(Text::new("Screenshots Directory").size(22).font(BOLD_FONT))
                        .push(Space::with_width(Length::Fill))
                        .push(
                            Button::new(
                                Text::new(FOLDER_ICON_ICON)
                                    .font(ICON_FONT)
                                    .size(20)
                                    .center(),
                            )
                            .height(40)
                            .width(40)
                            .on_press(Message::OpenFolder),
                        )
                        .push(Space::with_width(10))
                        .push(
                            Button::new(Text::new(self.screenshot_dir.as_str()).size(20).center())
                                .height(40)
                                .width(250)
                                .on_press(Message::UpdateFolderPath),
                        )
                        .align_y(Center)
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .padding(10),
                )
                .height(80),
            )
            .push(
                Container::new(
                    Row::new()
                        .push(Text::new("App Theme").size(22).font(BOLD_FONT))
                        .push(Space::with_width(Length::Fill))
                        .push(
                            Button::new(
                                Text::new(self.theme.target().to_string()).size(20).center(),
                            )
                            .height(40)
                            .width(160)
                            .on_press(Message::ToggleTheme),
                        )
                        .align_y(Center)
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .padding(10),
                )
                .height(80),
            )
            .push(
                Container::new(
                    Row::new()
                        .push(Text::new("Notifications").size(22).font(BOLD_FONT))
                        .push(Space::with_width(Length::Fill))
                        .push(
                            Toggler::new(self.notifications)
                                .size(22)
                                .on_toggle(Message::UpdateNotifications),
                        )
                        .align_y(Center)
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .padding(10),
                )
                .height(80),
            )
            .spacing(10);

        let content = Column::new()
            .push(header)
            .push(body)
            .spacing(10)
            .padding(15);

        Animation::new(&self.theme, content)
            .on_update(Message::AnimateTheme)
            .into()
    }
}
