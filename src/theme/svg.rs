use iced::widget::svg::{Catalog, Status, Style};

use crate::entities::{style::SvgClass, theme::Theme};

impl Catalog for Theme {
    type Class<'a> = SvgClass;

    fn default<'a>() -> Self::Class<'a> {
        SvgClass
    }

    fn style(&self, _class: &Self::Class<'_>, _status: Status) -> Style {
        Style {
            color: Some(self.palette().text),
        }
    }
}
