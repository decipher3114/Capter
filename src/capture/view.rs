use iced::{
    Alignment::{self},
    Length,
    widget::{
        Button, Canvas, Column, Container, Image, Row, Slider, Stack, Text, TextInput, Tooltip,
        image::Handle, opaque, tooltip::Position,
    },
};

use crate::{
    consts::{ICON_FONT, MEDIUM_FONT, MOVE_ICON},
    theme::{Element, button::ButtonClass, container::ContainerClass, text::TextClass},
};

use super::{
    Capture, Message, Mode,
    crop::CropState,
    draw::{Tool, ToolColor},
};

const PADDING: f32 = 10.0;
const SPACING: f32 = 10.0;
const TEXT_SIZE: f32 = 18.0;
const BUTTON_SIZE: f32 = 30.0;
const CONTAINER_WIDTH: f32 = 420.0;

impl Capture {
    pub fn view(&self) -> Element<Message> {
        let mut stack = Stack::new().height(Length::Fill).width(Length::Fill);

        stack = stack.push(
            Image::new(Handle::from_rgba(
                self.image.width(),
                self.image.height(),
                self.image.clone().into_raw(),
            ))
            .height(Length::Fill)
            .width(Length::Fill),
        );

        let canvas_with_tooltip = |description| {
            Tooltip::new(
                Canvas::new(self).width(Length::Fill).height(Length::Fill),
                Container::new(
                    Container::new(Text::new(description).size(TEXT_SIZE).center())
                        .padding(PADDING),
                )
                .padding(PADDING)
                .class(ContainerClass::Transparent),
                Position::FollowCursor,
            )
            .class(ContainerClass::Transparent)
        };

        match &self.mode {
            Mode::Crop { size, status, .. } => {
                let description = match status {
                    CropState::FullScreen => String::from("Fullscreen"),
                    CropState::Window(window) => window.name.clone(),
                    CropState::Area | CropState::InProgress { .. } => {
                        format!("{} x {}", size.width as u32, size.height as u32)
                    }
                    CropState::None => "Exiting".to_string(),
                };

                stack = stack.push(canvas_with_tooltip(description));

                if status.is_idle() {
                    stack = stack.push(
                        self.toolbar(
                            Row::new()
                                .push(
                                    Row::from_iter(Tool::ALL.into_iter().map(|tool| {
                                        toolbar_icon(
                                            tool.icon(),
                                            TextClass::Default,
                                            false,
                                            Message::ChangeTool(tool),
                                        )
                                    }))
                                    .spacing(SPACING),
                                )
                                .push(icon_button(
                                    MOVE_ICON.to_string(),
                                    TextClass::Default,
                                    Message::MoveToolBar,
                                    ButtonClass::Selected,
                                ))
                                .spacing(SPACING),
                        ),
                    );
                };
            }
            Mode::Draw {
                element: shape,
                status,
            } => {
                stack = stack.push(canvas_with_tooltip(format!(
                    "{} x {}",
                    self.cursor_position.x as u32, self.cursor_position.y as u32
                )));

                if status.is_idle() || shape.tool.is_text_tool() {
                    let mut toolbar_column = Column::new()
                        .push(
                            Row::new()
                                .push(
                                    Row::from_iter(Tool::ALL.into_iter().map(|tool| {
                                        toolbar_icon(
                                            tool.icon(),
                                            TextClass::Default,
                                            shape.tool == tool,
                                            Message::ChangeTool(tool),
                                        )
                                    }))
                                    .spacing(SPACING),
                                )
                                .push(icon_button(
                                    MOVE_ICON.to_string(),
                                    TextClass::Default,
                                    Message::MoveToolBar,
                                    ButtonClass::Selected,
                                ))
                                .spacing(SPACING),
                        )
                        .push(
                            Row::new()
                                .push(
                                    Slider::new(1..=5, shape.size, Message::ChangeSize)
                                        .height(BUTTON_SIZE)
                                        .width(Length::Fill),
                                )
                                .push(
                                    Row::from_iter(ToolColor::ALL.into_iter().map(|color| {
                                        toolbar_icon(
                                            color.icon(),
                                            TextClass::Custom(color.into()),
                                            shape.color == color,
                                            Message::ChangeColor(color),
                                        )
                                    }))
                                    .spacing(SPACING),
                                )
                                .spacing(SPACING),
                        )
                        .align_x(Alignment::Center)
                        .spacing(SPACING);

                    if status.is_waiting_for_input() {
                        if let Tool::Text { text, .. } = &shape.tool {
                            toolbar_column = toolbar_column.push(
                                TextInput::new("Enter Text", text)
                                    .width(Length::Fill)
                                    .font(MEDIUM_FONT)
                                    .size(TEXT_SIZE)
                                    .on_input(Message::UpdateText)
                                    .id("text_input"),
                            );
                        }
                    }

                    stack = stack.push(self.toolbar(toolbar_column))
                };
            }
        }

        stack.into()
    }

    fn toolbar<'a>(&self, content: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
        Container::new(opaque(
            Container::new(content)
                .center_x(CONTAINER_WIDTH)
                .padding(PADDING),
        ))
        .center_x(Length::Fill)
        .height(Length::Fill)
        .align_y(match self.toolbar_at_top {
            true => Alignment::Start,
            false => Alignment::End,
        })
        .padding(PADDING)
        .class(ContainerClass::Transparent)
        .into()
    }
}

fn toolbar_icon<'a>(
    icon: String,
    text_class: TextClass,
    selected: bool,
    message: Message,
) -> Element<'a, Message> {
    let button_class = match selected {
        true => ButtonClass::Selected,
        false => ButtonClass::Default,
    };

    icon_button(icon, text_class, message, button_class)
}

fn icon_button<'a>(
    text: impl ToString,
    text_class: TextClass,
    message: Message,
    button_class: ButtonClass,
) -> Element<'a, Message> {
    Button::new(
        Text::new(text.to_string())
            .font(ICON_FONT)
            .size(TEXT_SIZE)
            .center()
            .class(text_class),
    )
    .on_press(message)
    .height(BUTTON_SIZE)
    .width(BUTTON_SIZE)
    .class(button_class)
    .into()
}
