use iced::{
    border::Radius,
    widget::button::{Catalog, Status, Style},
    Background, Border, Color,
};

use crate::entities::{style::ButtonClass, theme::Theme};

impl Catalog for Theme {
    type Class<'a> = ButtonClass;

    fn default<'a>() -> Self::Class<'a> {
        ButtonClass::Default
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        Style {
            background: {
                match status {
                    Status::Active => match self {
                        Self::Light => match class {
                            ButtonClass::Default => {
                                Some(Background::Color(Color::from_rgb8(190, 190, 190)))
                            }
                            ButtonClass::Danger => {
                                Some(Background::Color(Color::from_rgb8(255, 100, 100)))
                            }
                        },
                        Self::Dark => match class {
                            ButtonClass::Default => {
                                Some(Background::Color(Color::from_rgb8(70, 70, 70)))
                            }
                            ButtonClass::Danger => {
                                Some(Background::Color(Color::from_rgb8(228, 67, 67)))
                            }
                        },
                    },
                    Status::Hovered | Status::Pressed => match self {
                        Self::Light => match class {
                            ButtonClass::Default => {
                                Some(Background::Color(Color::from_rgb8(180, 180, 180)))
                            }
                            ButtonClass::Danger => {
                                Some(Background::Color(Color::from_rgb8(245, 90, 90)))
                            }
                        },
                        Self::Dark => match class {
                            ButtonClass::Default => {
                                Some(Background::Color(Color::from_rgb8(80, 80, 80)))
                            }
                            ButtonClass::Danger => {
                                Some(Background::Color(Color::from_rgb8(238, 77, 77)))
                            }
                        },
                    },
                    _ => Some(Background::Color(Color::default())),
                }
            },
            border: Border {
                color: match self {
                    Theme::Light => Color::from_rgb8(180, 180, 180),
                    Theme::Dark => Color::from_rgb8(80, 80, 80),
                },
                width: 0.5,
                radius: Radius::new(8),
            },
            ..Default::default()
        }
    }
}
