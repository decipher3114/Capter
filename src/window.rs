use crate::{
    capture::Capture,
    settings::Settings,
};

pub enum AppWindow {
    Settings(Box<Settings>),
    Capture(Box<Capture>),
}

impl From<Settings> for AppWindow {
    fn from(settings: Settings) -> Self {
        AppWindow::Settings(Box::new(settings))
    }
}

impl From<Capture> for AppWindow {
    fn from(capture: Capture) -> Self {
        AppWindow::Capture(Box::new(capture))
    }
}
