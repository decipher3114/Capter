use iced_anim::Spring;

use crate::theme::Theme;

use super::ConfigureWindow;

impl ConfigureWindow {
    pub fn new(path: String, theme: Theme) -> Self {
        Self {
            path,
            theme: Spring::new(theme),
        }
    }
}
