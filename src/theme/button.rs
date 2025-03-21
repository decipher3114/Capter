use iced::{
    Background, Color,
    widget::button::{Catalog, Status, Style},
};

use super::{Theme, border};

pub enum ButtonClass {
    Default,
    Danger,
    Selected,
}

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
                        ButtonClass::Selected => Some(Background::Color(palette.active_primary)),
                    },
                    Status::Hovered | Status::Pressed => match class {
                        ButtonClass::Default => Some(Background::Color(palette.secondary)),
                        ButtonClass::Danger => Some(Background::Color(palette.danger_secondary)),
                        ButtonClass::Selected => Some(Background::Color(palette.active_secondary)),
                    },
                    Status::Disabled => Some(Background::Color(Color::default())),
                }
            },
            border: border(palette),
            text_color: match status {
                Status::Disabled => palette.secondary,
                _ => palette.text,
            },
            ..Default::default()
        }
    }
}
