use iced::{
    Background,
    widget::slider::{Catalog, Handle, HandleShape, Rail, Status, Style},
};

use super::{Theme, border};

pub enum SliderClass {
    Default,
}

impl Catalog for Theme {
    type Class<'a> = SliderClass;

    fn default<'a>() -> Self::Class<'a> {
        SliderClass::Default
    }

    fn style(&self, _class: &Self::Class<'_>, status: Status) -> Style {
        let extended_palette = self.extended_palette();

        Style {
            rail: Rail {
                backgrounds: (
                    Background::Color(extended_palette.background.base.color),
                    Background::Color(extended_palette.background.base.color),
                ),
                width: 10.0,
                border: border(extended_palette),
            },
            handle: Handle {
                shape: HandleShape::Circle { radius: 10.0 },
                background: match status {
                    Status::Active => extended_palette.primary.base.color,
                    Status::Hovered | Status::Dragged => extended_palette.primary.strong.color,
                }
                .into(),
                border_width: 0.5,
                border_color: extended_palette.background.strongest.color,
            },
        }
    }
}
