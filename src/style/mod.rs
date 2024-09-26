use std::fmt::Display;

use iced::{
    color,
    daemon::{Appearance, DefaultStyle},
    Color,
};

use crate::entities::theme::{Palette, Theme};

mod button;
mod container;
mod svg;
mod text;

pub type Element<'a, Message> = iced::Element<'a, Message, Theme>;

pub const LIGHT: Palette = Palette {
    background: color!(220, 220, 220),
    surface: color!(210, 210, 210),
    text: color!(50, 50, 50),
    primary: color!(190, 190, 190),
    secondary: color!(170, 170, 170),
    danger_primary: color!(255, 100, 100),
    danger_secondary: color!(245, 90, 90),
};

pub const DARK: Palette = Palette {
    background: color!(40, 40, 40),
    surface: color!(50, 50, 50),
    text: color!(210, 210, 210),
    primary: color!(70, 70, 70),
    secondary: color!(90, 90, 90),
    danger_primary: color!(228, 67, 67),
    danger_secondary: color!(238, 77, 77),
};

impl Theme {
    pub fn palette(&self) -> Palette {
        match self {
            Theme::Light => LIGHT,
            Theme::Dark => DARK,
        }
    }
}

impl Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Light => write!(f, "Light"),
            Self::Dark => write!(f, "Dark"),
        }
    }
}

impl DefaultStyle for Theme {
    fn default_style(&self) -> Appearance {
        Appearance {
            background_color: self.palette().background,
            text_color: Color::default(),
        }
    }
}
