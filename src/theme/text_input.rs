use iced::{
    Border,
    border::Radius,
    widget::text_input::{Catalog, Status, Style},
};

use super::Theme;

pub enum TextInputClass {
    Default,
}

impl Catalog for Theme {
    type Class<'a> = TextInputClass;

    fn default<'a>() -> Self::Class<'a> {
        TextInputClass::Default
    }

    fn style(&self, _class: &Self::Class<'_>, status: Status) -> Style {
        let palette = self.palette();

        Style {
            background: palette.primary.into(),
            border: Border {
                color: match status {
                    Status::Hovered | Status::Focused { .. } => palette.active_primary,
                    Status::Active | Status::Disabled => palette.secondary,
                },
                width: 0.5,
                radius: Radius::new(8),
            },
            icon: palette.text,
            placeholder: palette.secondary,
            value: palette.text,
            selection: palette.active_secondary,
        }
    }
}
