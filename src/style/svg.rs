use iced::{
    widget::svg::{self, Status, Style},
    Color,
};

use crate::entities::{style::SvgClass, theme::Theme};

impl svg::Catalog for Theme {
    type Class<'a> = SvgClass;

    fn default<'a>() -> Self::Class<'a> {
        SvgClass
    }

    fn style(&self, _class: &Self::Class<'_>, _status: Status) -> svg::Style {
        Style {
            color: Some(match self {
                Theme::Light => Color::from_rgb8(50, 50, 50),
                Theme::Dark => Color::from_rgb8(210, 210, 210),
            }),
        }
    }
}
