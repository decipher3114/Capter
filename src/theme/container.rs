use iced::{
    Background, Color, Shadow,
    widget::container::{Catalog, Style},
};

use super::{Theme, border};

pub enum ContainerClass {
    Default,
    Transparent,
}

impl Catalog for Theme {
    type Class<'a> = ContainerClass;

    fn default<'a>() -> Self::Class<'a> {
        ContainerClass::Default
    }

    fn style(&self, class: &Self::Class<'_>) -> Style {
        let palette = self.palette();
        let extended_palette = self.extended_palette();
        match class {
            ContainerClass::Default => Style {
                background: Some(Background::Color(extended_palette.background.weak.color)),
                border: border(extended_palette),
                text_color: Some(palette.text),
                shadow: Shadow::default(),
            },
            ContainerClass::Transparent => Style {
                background: Some(Background::Color(Color::TRANSPARENT)),
                ..Default::default()
            },
        }
    }
}
