use std::fmt::Display;

use iced::{
    color,
    daemon::{Appearance, DefaultStyle},
    Color,
};
use iced_anim::Animate;

use crate::entities::theme::{Palette, Theme};

mod button;
mod container;
mod text;

pub type Element<'a, Message> = iced::Element<'a, Message, Theme>;

pub const LIGHT: Palette = Palette {
    background: color!(0xdcdcdc),
    surface: color!(0xd2d2d2),
    text: color!(0x323232),
    primary: color!(0xbebebe),
    secondary: color!(0xaaaaaa),
    active_primary: color!(0x6464ff),
    active_secondary: color!(0x5a5af5),
    danger_primary: color!(0xff6464),
    danger_secondary: color!(0xf55a5a),
};

pub const DARK: Palette = Palette {
    background: color!(0x282828),
    surface: color!(0x323232),
    text: color!(0xd2d2d2),
    primary: color!(0x464646),
    secondary: color!(0x5a5a5a),
    active_primary: color!(0x4343e4),
    active_secondary: color!(0x4d4dee),
    danger_primary: color!(0xe44343),
    danger_secondary: color!(0xee4d4d),
};

impl Theme {
    pub fn palette(&self) -> Palette {
        match self {
            Theme::Light => LIGHT,
            Theme::Dark => DARK,
            Theme::Custom(palette) => *palette,
        }
    }

    pub fn toggle(&self) -> Self {
        match self {
            Theme::Light => Theme::Dark,
            _ => Theme::Light,
        }
    }
}

impl Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Light => write!(f, "Light"),
            Self::Dark => write!(f, "Dark"),
            Self::Custom(_) => write!(f, "Custom"),
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

impl Animate for Theme {
    fn components() -> usize {
        Palette::components()
    }

    fn update(&mut self, components: &mut impl Iterator<Item = f32>) {
        let mut palette = self.palette();
        palette.update(components);
        *self = Theme::Custom(palette);
    }

    fn distance_to(&self, end: &Self) -> Vec<f32> {
        self.palette().distance_to(&end.palette())
    }
}
