use iced::{
    Alignment::Center,
    Length,
    widget::{
        Button,
        Column,
        Container,
        PickList,
        Row,
        Scrollable,
        Space,
        Text,
        Toggler,
    },
};

use crate::{
    config::Config,
    consts::{
        APPNAME,
        BOLD_FONT,
        FOLDER_ICON_ICON,
        ICON_FONT,
    },
    organize_type::OrgranizeMode,
    settings::{
        Message,
        Settings,
    },
    theme::{
        Element,
        Theme,
        button::ButtonClass,
    },
};

const TEXT_SIZE: u32 = 20;

impl Settings {
    pub fn view<'a>(&'a self, config: &'a Config) -> Element<'a, Message> {
        let header = Row::new()
            .push(Text::new(APPNAME).size(60).font(BOLD_FONT))
            .push(Space::new().width(Length::Fill))
            .push(
                Button::new(Text::new("Exit").center().size(TEXT_SIZE))
                    .on_press(Message::RequestExit)
                    .height(40)
                    .class(ButtonClass::Danger),
            )
            .align_y(Center);

        let body = Scrollable::new(
            Column::new()
                .push(list_item(
                    "Theme",
                    PickList::new(&Theme::ALL[..], Some(&config.theme), Message::UpdateTheme)
                        .text_size(TEXT_SIZE)
                        .into(),
                ))
                .push(list_item(
                    "Show Notification",
                    Toggler::new(config.show_notification)
                        .size(22)
                        .on_toggle(Message::ToggleShowNotification)
                        .into(),
                ))
                .push(list_item(
                    "Screenshots Folder",
                    Row::new()
                        .push(
                            Button::new(
                                Text::new(FOLDER_ICON_ICON)
                                    .font(ICON_FONT)
                                    .size(TEXT_SIZE)
                                    .center(),
                            )
                            .on_press(Message::OpenFolder),
                        )
                        .push(Space::new().width(10))
                        .push(
                            Button::new(
                                Text::new(self.folder_path.as_str())
                                    .size(TEXT_SIZE)
                                    .center(),
                            )
                            .on_press(Message::UpdateFolderPath),
                        )
                        .into(),
                ))
                .push(list_item(
                    "Organize Mode",
                    PickList::new(
                        &OrgranizeMode::ALL[..],
                        Some(&config.organize_mode),
                        Message::UpdateOrganizeMode,
                    )
                    .text_size(TEXT_SIZE)
                    .into(),
                ))
                .spacing(10),
        )
        .spacing(10);

        Column::new()
            .push(header)
            .push(body)
            .spacing(10)
            .padding(10)
            .into()
    }
}

fn list_item<'a>(label: &'a str, item: Element<'a, Message>) -> Element<'a, Message> {
    Container::new(
        Row::new()
            .push(Text::new(label).size(22).font(BOLD_FONT))
            .push(Space::new().width(Length::Fill))
            .push(item)
            .align_y(Center)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10),
    )
    .height(80)
    .into()
}
