use iced::Color;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub enum Theme {
    #[default]
    Light,
    Dark,
}

pub struct Palette {
    pub background: Color,
    pub surface: Color,
    pub text: Color,
    pub primary: Color,
    pub secondary: Color,
    pub danger_primary: Color,
    pub danger_secondary: Color,
}
