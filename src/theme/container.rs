use iced::{
    Background, Color,
    widget::container::{self, Style},
};

use super::{Theme, border};

pub enum ContainerClass {
    Default,
    Transparent,
}

impl container::Catalog for Theme {
    type Class<'a> = ContainerClass;

    fn default<'a>() -> Self::Class<'a> {
        ContainerClass::Default
    }

    fn style(&self, class: &Self::Class<'_>) -> Style {
        let palette = self.palette();
        match class {
            ContainerClass::Default => Style {
                background: Some(Background::Color(palette.surface)),
                border: border(palette),
                ..Default::default()
            },
            ContainerClass::Transparent => Style {
                background: Some(Background::Color(Color::TRANSPARENT)),
                ..Default::default()
            },
        }
    }
}
