use iced::{
    Background,
    overlay::menu::{
        Catalog,
        Style,
    },
};

use crate::theme::{
    Theme,
    border,
};

pub enum MenuClass {
    Default,
}

impl Catalog for Theme {
    type Class<'a> = MenuClass;

    fn default<'a>() -> <Self as Catalog>::Class<'a> {
        MenuClass::Default
    }

    fn style(&self, _class: &<Self as Catalog>::Class<'_>) -> Style {
        let palette = self.palette();
        let extended_palette = self.extended_palette();

        Style {
            background: Background::Color(extended_palette.background.strong.color),
            border: border(extended_palette),
            text_color: palette.text,
            selected_text_color: palette.text,
            selected_background: Background::Color(extended_palette.primary.base.color),
        }
    }
}
