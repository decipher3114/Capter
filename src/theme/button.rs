use iced::{
    Background, Color,
    widget::button::{Catalog, Status, Style},
};

use crate::theme::{Theme, border};

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
        let extended_palette = self.extended_palette();

        Style {
            background: {
                match status {
                    Status::Active => match class {
                        ButtonClass::Default => {
                            Some(Background::Color(extended_palette.background.strong.color))
                        }
                        ButtonClass::Danger => {
                            Some(Background::Color(extended_palette.danger.base.color))
                        }
                        ButtonClass::Selected => {
                            Some(Background::Color(extended_palette.primary.base.color))
                        }
                    },
                    Status::Hovered | Status::Pressed => match class {
                        ButtonClass::Default => {
                            Some(Background::Color(extended_palette.background.strong.color))
                        }
                        ButtonClass::Danger => {
                            Some(Background::Color(extended_palette.danger.strong.color))
                        }
                        ButtonClass::Selected => {
                            Some(Background::Color(extended_palette.primary.strong.color))
                        }
                    },
                    Status::Disabled => Some(Background::Color(Color::default())),
                }
            },
            border: border(extended_palette),
            text_color: palette.text,
            ..Default::default()
        }
    }
}
