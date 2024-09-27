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
            Theme::Custom(palette) => *palette,
        }
    }

    /// Toggles the theme between light and dark, defaulting to `Light` if using a custom palette.
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

impl iced_anim::Animate for Theme {
    fn components() -> usize {
        Palette::components()
    }

    fn distance_to(&self, end: &Self) -> Vec<f32> {
        self.palette().distance_to(&end.palette())
    }

    fn update(&mut self, components: &mut impl Iterator<Item = f32>) {
        let mut palette = self.palette();
        palette.update(components);
        *self = Theme::Custom(palette);
    }
}
