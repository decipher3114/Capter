use crate::config::Config;

use super::Settings;

impl Settings {
    pub fn init(config: &Config) -> Self {
        Self {
            theme: config.theme.clone(),
            notifications: config.notifications,
            screenshot_dir: config.display_screenshot_dir(),
        }
    }
}
