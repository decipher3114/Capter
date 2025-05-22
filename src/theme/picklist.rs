use iced::{
    Background,
    widget::pick_list::{Catalog, Style},
};

use crate::theme::{Theme, border};

pub enum PickListClass {
    Default,
}

impl Catalog for Theme {
    type Class<'a> = PickListClass;

    fn default<'a>() -> <Self as Catalog>::Class<'a> {
        PickListClass::Default
    }

    fn style(
        &self,
        _class: &<Self as Catalog>::Class<'_>,
        _status: iced::widget::pick_list::Status,
    ) -> Style {
        let palette = self.palette();
        let extended_palette = self.extended_palette();

        Style {
            text_color: palette.text,
            placeholder_color: extended_palette.background.weak.color,
            handle_color: palette.text,
            background: Background::Color(extended_palette.background.strong.color),
            border: border(extended_palette),
        }
    }
}
