use iced::{
    border::Radius,
    widget::button::{Catalog, Status, Style},
    Background, Border, Color,
};

use crate::entities::{style::ButtonClass, theme::Theme};

impl Catalog for Theme {
    type Class<'a> = ButtonClass;

    fn default<'a>() -> Self::Class<'a> {
        ButtonClass::Default
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        let palette = self.palette();
        Style {
            background: {
                match status {
                    Status::Active => match class {
                        ButtonClass::Default => Some(Background::Color(palette.primary)),
                        ButtonClass::Danger => Some(Background::Color(palette.danger_primary)),
                    },
                    Status::Hovered | Status::Pressed => match class {
                        ButtonClass::Default => Some(Background::Color(palette.secondary)),
                        ButtonClass::Danger => Some(Background::Color(palette.danger_secondary)),
                    },
                    _ => Some(Background::Color(Color::default())),
                }
            },
            border: Border {
                color: palette.secondary,
                width: 0.5,
                radius: Radius::new(8),
            },
            ..Default::default()
        }
    }
}
