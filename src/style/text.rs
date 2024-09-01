use iced::{
    widget::text::{Catalog, Style},
    Color,
};

use crate::entities::{style::TextClass, theme::Theme};

impl Catalog for Theme {
    type Class<'a> = TextClass;

    fn default<'a>() -> Self::Class<'a> {
        TextClass
    }

    fn style(&self, _item: &Self::Class<'_>) -> Style {
        Style {
            color: Some(match self {
                Theme::Light => Color::from_rgb8(50, 50, 50),
                Theme::Dark => Color::from_rgb8(210, 210, 210),
            }),
        }
    }
}
