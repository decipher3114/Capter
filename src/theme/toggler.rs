use iced::widget::toggler::{Catalog, Status, Style};

use crate::theme::Theme;

pub enum TogglerClass {
    Default,
}

impl Catalog for Theme {
    type Class<'a> = TogglerClass;

    fn default<'a>() -> Self::Class<'a> {
        TogglerClass::Default
    }

    fn style(&self, _class: &Self::Class<'_>, status: Status) -> Style {
        let extended_palette = self.extended_palette();

        Style {
            background: extended_palette.background.base.color,
            background_border_width: 0.5,
            background_border_color: extended_palette.background.strongest.color,
            foreground: match status {
                Status::Active { is_toggled } => match is_toggled {
                    true => extended_palette.primary.base.color,
                    false => extended_palette.secondary.base.color,
                },
                Status::Hovered { is_toggled } => match is_toggled {
                    true => extended_palette.primary.strong.color,
                    false => extended_palette.secondary.strong.color,
                },
                Status::Disabled => extended_palette.background.strongest.color,
            },
            foreground_border_width: 0.5,
            foreground_border_color: extended_palette.background.strongest.color,
        }
    }
}
