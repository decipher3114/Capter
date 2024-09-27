use iced::{
    border::Radius,
    widget::container::{self, Style},
    Background, Border,
};

use crate::entities::{style::ContainerClass, theme::Theme};

impl container::Catalog for Theme {
    type Class<'a> = ContainerClass;

    fn default<'a>() -> Self::Class<'a> {
        ContainerClass
    }

    fn style(&self, _class: &Self::Class<'_>) -> Style {
        let palette = self.palette();
        Style {
            background: Some(Background::Color(palette.surface)),
            border: Border {
                color: palette.secondary,
                width: 0.5,
                radius: Radius::new(8),
            },
            ..Default::default()
        }
    }
}
