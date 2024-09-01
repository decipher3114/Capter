use iced::{border::Radius, widget::container, Background, Border, Color};

use crate::entities::{style::ContainerClass, theme::Theme};

impl container::Catalog for Theme {
    type Class<'a> = ContainerClass;

    fn default<'a>() -> Self::Class<'a> {
        ContainerClass
    }

    fn style(&self, _class: &Self::Class<'_>) -> container::Style {
        match self {
            Self::Light => container::Style {
                background: Some(Background::Color(Color::from_rgb8(210, 210, 210))),
                border: Border {
                    color: Color::from_rgb8(190, 190, 190),
                    width: 0.5,
                    radius: Radius::new(8),
                },
                ..Default::default()
            },
            Self::Dark => container::Style {
                background: Some(Background::Color(Color::from_rgb8(50, 50, 50))),
                border: Border {
                    color: Color::from_rgb8(80, 80, 80),
                    width: 0.5,
                    radius: Radius::new(8),
                },
                ..Default::default()
            },
        }
    }
}
