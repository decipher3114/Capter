use std::fmt::Display;

use iced::{
    Border, Color,
    border::Radius,
    color,
    theme::{Base, Palette, Style, palette::Extended},
};
use iced_anim::Animate;
use serde::{Deserialize, Serialize};

pub mod button;
pub mod container;
pub mod slider;
pub mod text;
pub mod text_input;
pub mod toggler;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub enum Theme {
    #[default]
    Light,
    Dark,
    #[serde(skip)]
    Custom(Palette),
}

pub const LIGHT_PALETTE: Palette = Palette {
    background: color!(0xdcdcdc),
    text: color!(0x323232),
    primary: color!(0x6464ff),
    success: color!(0x4caf50),
    warning: color!(0xffa500),
    danger: color!(0xff6464),
};

pub const DARK_PALETTE: Palette = Palette {
    background: color!(0x3c3c3c),
    text: color!(0xd2d2d2),
    primary: color!(0x4343e4),
    success: color!(0x5da65d),
    warning: color!(0xe4a343),
    danger: color!(0xe44343),
};

pub type Element<'a, Message> = iced::Element<'a, Message, Theme>;

impl Theme {
    pub fn palette(&self) -> Palette {
        match self {
            Theme::Light => LIGHT_PALETTE,
            Theme::Dark => DARK_PALETTE,
            Theme::Custom(palette) => *palette,
        }
    }

    pub fn extended_palette(&self) -> Extended {
        match self {
            Theme::Light => Extended::generate(LIGHT_PALETTE),
            Theme::Dark => Extended::generate(DARK_PALETTE),
            Theme::Custom(palette) => Extended::generate(*palette),
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

impl Base for Theme {
    fn base(&self) -> Style {
        Style {
            background_color: self.extended_palette().background.weakest.color,
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

    fn lerp(&mut self, start: &Self, end: &Self, progress: f32) {
        let start = start.palette();
        let end = end.palette();
        let mut palette = start;
        palette.lerp(&start, &end, progress);
        *self = Theme::Custom(palette);
    }
}

pub fn border(extended_palette: Extended) -> Border {
    Border {
        color: extended_palette.background.strongest.color,
        width: 0.5,
        radius: Radius::new(8),
    }
}
