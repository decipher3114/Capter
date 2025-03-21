use iced::widget::toggler::{Catalog, Status, Style};

use super::Theme;

pub enum TogglerClass {
    Default,
}

impl Catalog for Theme {
    type Class<'a> = TogglerClass;

    fn default<'a>() -> Self::Class<'a> {
        TogglerClass::Default
    }

    fn style(&self, _class: &Self::Class<'_>, status: Status) -> Style {
        let palette = self.palette();

        Style {
            background: palette.background,
            background_border_width: 0.5,
            background_border_color: palette.secondary,
            foreground: match status {
                Status::Active { is_toggled } => match is_toggled {
                    true => palette.active_primary,
                    false => palette.background,
                },
                Status::Hovered { is_toggled } => match is_toggled {
                    true => palette.active_secondary,
                    false => palette.background,
                },
                Status::Disabled => palette.surface,
            },
            foreground_border_width: 0.5,
            foreground_border_color: palette.secondary,
        }
    }
}
