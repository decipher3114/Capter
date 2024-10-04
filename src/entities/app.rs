use std::collections::BTreeMap;

use iced::window::Id;
use tray_icon::TrayIcon;

pub use crate::entities::events::AppEvent;
use crate::entities::{config::Config, window::WindowType};

pub struct App {
    #[expect(dead_code)]
    tray_icon: TrayIcon,
    pub config: Config,
    pub windows: BTreeMap<Id, WindowType>,
}

impl App {
    pub fn new(tray_icon: TrayIcon, config: Config) -> Self {
        Self {
            tray_icon,
            config,
            windows: BTreeMap::new(),
        }
    }
}
