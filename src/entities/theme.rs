use iced::Color;
use iced_anim::Animate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub enum Theme {
    #[default]
    Light,
    Dark,
    #[serde(skip)]
    Custom(Palette),
}

#[derive(Debug, Clone, Copy, PartialEq, Animate)]
pub struct Palette {
    pub background: Color,
    pub surface: Color,
    pub text: Color,
    pub primary: Color,
    pub secondary: Color,
    pub danger_primary: Color,
    pub danger_secondary: Color,
}
