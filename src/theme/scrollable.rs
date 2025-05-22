use iced::{
    Background,
    widget::{
        container,
        scrollable::{Catalog, Rail, Scroller, Status, Style},
    },
};

use crate::theme::{Theme, border};

pub enum ScrollableClass {
    Default,
}

impl Catalog for Theme {
    type Class<'a> = ScrollableClass;

    fn default<'a>() -> Self::Class<'a> {
        ScrollableClass::Default
    }

    fn style(&self, _class: &Self::Class<'_>, _status: Status) -> Style {
        let extended_palette = self.extended_palette();

        let rail = Rail {
            background: Some(Background::Color(extended_palette.background.weak.color)),
            border: border(extended_palette),
            scroller: Scroller {
                color: extended_palette.background.strong.color,
                border: border(extended_palette),
            },
        };

        Style {
            container: container::Style::default(),
            vertical_rail: rail,
            horizontal_rail: rail,
            gap: None,
        }
    }
}
