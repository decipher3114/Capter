use iced::{
    Background,
    Border,
    border::Radius,
    widget::text_input::{
        Catalog,
        Status,
        Style,
    },
};

use crate::theme::Theme;

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
        let extended_palette = self.extended_palette();

        Style {
            background: Background::Color(extended_palette.background.strong.color),
            border: Border {
                color: match status {
                    Status::Hovered | Status::Focused { .. } => extended_palette.primary.base.color,
                    Status::Active | Status::Disabled => extended_palette.background.weak.color,
                },
                width: 0.5,
                radius: Radius::new(8),
            },
            icon: palette.text,
            placeholder: extended_palette.background.weakest.text,
            value: palette.text,
            selection: extended_palette.primary.weak.color,
        }
    }
}
