use iced_anim::{Animated, Easing};

use crate::theme::Theme;

use super::ConfigureWindow;

impl ConfigureWindow {
    pub fn new(path: String, theme: Theme) -> Self {
        Self {
            path,
            theme: Animated::new(theme, Easing::default()),
        }
    }
}
