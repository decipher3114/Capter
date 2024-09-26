use iced::widget::text::{Catalog, Style};

use crate::entities::{style::TextClass, theme::Theme};

impl Catalog for Theme {
    type Class<'a> = TextClass;

    fn default<'a>() -> Self::Class<'a> {
        TextClass
    }

    fn style(&self, _item: &Self::Class<'_>) -> Style {
        Style {
            color: Some(self.palette().text),
        }
    }
}
