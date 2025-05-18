pub mod button;
pub mod container;
pub mod slider;
pub mod text;
pub mod text_input;
pub mod toggler;

use std::{fmt::Display, sync::LazyLock};

use iced::{
    Border, Color,
    border::Radius,
    color,
    theme::{Base, Palette, Style, palette::Extended},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub enum Theme {
    #[default]
    System,
    Light,
    Dark,
}

pub const LIGHT_PALETTE: Palette = Palette {
    background: color!(0xdcdcdc),
    text: color!(0x323232),
    primary: color!(0x6464ff),
    success: color!(0x4caf50),
    warning: color!(0xffa500),
    danger: color!(0xff6464),
};

pub static EXTENDED_LIGHT_PALETTE: LazyLock<Extended> =
    LazyLock::new(|| Extended::generate(LIGHT_PALETTE));

pub const DARK_PALETTE: Palette = Palette {
    background: color!(0x3c3c3c),
    text: color!(0xd2d2d2),
    primary: color!(0x4343e4),
    success: color!(0x5da65d),
    warning: color!(0xe4a343),
    danger: color!(0xe44343),
};

pub static EXTENDED_DARK_PALETTE: LazyLock<Extended> =
    LazyLock::new(|| Extended::generate(DARK_PALETTE));

pub type Element<'a, Message> = iced::Element<'a, Message, Theme>;

impl Theme {
    pub fn palette(&self) -> Palette {
        match self {
            Self::System => {
                dark_light::detect().map_or(LIGHT_PALETTE, |theme| {
                    match theme {
                        dark_light::Mode::Dark => DARK_PALETTE,
                        dark_light::Mode::Light => LIGHT_PALETTE,
                        dark_light::Mode::Unspecified => LIGHT_PALETTE,
                    }
                })
            }
            Self::Light => LIGHT_PALETTE,
            Self::Dark => DARK_PALETTE,
        }
    }

    pub fn extended_palette(&self) -> Extended {
        match self {
            Self::System => {
                dark_light::detect().map_or(*EXTENDED_LIGHT_PALETTE, |theme| {
                    match theme {
                        dark_light::Mode::Dark => *EXTENDED_DARK_PALETTE,
                        dark_light::Mode::Light => *EXTENDED_LIGHT_PALETTE,
                        dark_light::Mode::Unspecified => *EXTENDED_LIGHT_PALETTE,
                    }
                })
            }
            Self::Light => *EXTENDED_LIGHT_PALETTE,
            Self::Dark => *EXTENDED_DARK_PALETTE,
        }
    }

    pub fn toggle(&mut self) {
        match self {
            Self::System => *self = Self::Light,
            Self::Light => *self = Self::Dark,
            Self::Dark => *self = Self::System,
        }
    }
}

impl Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::System => write!(f, "System"),
            Self::Light => write!(f, "Light"),
            Self::Dark => write!(f, "Dark"),
        }
    }
}

impl Base for Theme {
    fn base(&self) -> Style {
        Style {
            background_color: self.extended_palette().background.weakest.color,
            text_color: Color::default(),
        }
    }
}

pub fn border(extended_palette: Extended) -> Border {
    Border {
        color: extended_palette.background.strongest.color,
        width: 0.5,
        radius: Radius::new(8),
    }
}
