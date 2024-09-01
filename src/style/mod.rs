use std::fmt::Display;

mod button;
mod container;
mod svg;
mod text;

use iced::{
    daemon::{self, DefaultStyle},
    Color,
};

use crate::entities::theme::Theme;

pub type Element<'a, Message> = iced::Element<'a, Message, Theme>;

impl Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Light => write!(f, "Light"),
            Self::Dark => write!(f, "Dark"),
        }
    }
}

impl DefaultStyle for Theme {
    fn default_style(&self) -> daemon::Appearance {
        match self {
            Self::Light => daemon::Appearance {
                background_color: Color::from_rgb8(220, 220, 220),
                text_color: Color::default(),
            },
            Self::Dark => daemon::Appearance {
                background_color: Color::from_rgb8(40, 40, 40),
                text_color: Color::default(),
            },
        }
    }
}
