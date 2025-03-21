use iced_anim::{Animated, Easing};

use crate::config::Config;

use super::Settings;

impl Settings {
    pub fn init(config: &Config) -> Self {
        Self {
            theme: Animated::new(config.theme.clone(), Easing::EASE_IN),
            notifications: config.notifications,
            screenshot_dir: config.display_screenshot_dir(),
        }
    }
}
