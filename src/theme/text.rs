use iced::{
    Color,
    widget::text::{Catalog, Style},
};

use crate::theme::Theme;

pub enum TextClass {
    Default,
    Custom(Color),
}

impl Catalog for Theme {
    type Class<'a> = TextClass;

    fn default<'a>() -> Self::Class<'a> {
        TextClass::Default
    }

    fn style(&self, item: &Self::Class<'_>) -> Style {
        Style {
            color: match item {
                TextClass::Default => Some(self.palette().text),
                TextClass::Custom(color) => Some(*color),
            },
        }
    }
}
