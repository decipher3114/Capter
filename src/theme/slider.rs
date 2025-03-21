use iced::widget::slider::{Catalog, Handle, HandleShape, Rail, Status, Style};

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
        let palette = self.palette();

        Style {
            rail: Rail {
                backgrounds: (palette.primary.into(), palette.primary.into()),
                width: 10.0,
                border: border(palette),
            },
            handle: Handle {
                shape: HandleShape::Circle { radius: 10.0 },
                background: match status {
                    Status::Active => palette.active_secondary,
                    Status::Hovered => palette.active_primary,
                    Status::Dragged => palette.active_primary,
                }
                .into(),
                border_width: 0.5,
                border_color: palette.secondary,
            },
        }
    }
}
