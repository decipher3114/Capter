use std::collections::BTreeMap;

use iced::window::Id;
use tray_icon::TrayIcon;

use crate::entities::{
    capture::CaptureEvent,
    config::{Config, ConfigEvent},
    window::WindowType,
};

pub struct App {
    #[expect(dead_code)]
    tray_icon: TrayIcon,
    pub config: Config,
    pub windows: BTreeMap<Id, WindowType>,
}

impl App {
    pub(crate) fn new_internal(
        tray_icon: TrayIcon,
        config: Config,
        windows: BTreeMap<Id, WindowType>,
    ) -> Self {
        Self {
            tray_icon,
            config,
            windows,
        }
    }
}

#[derive(Debug, Clone)]
pub enum AppEvent {
    OpenConfigureWindow,
    OpenDirectory,
    UpdateDirectory(Id),
    GetScaleFactor(Id, f32),
    OpenCaptureWindow,
    Undo,
    Done,
    Cancel,
    RequestClose(Id),
    WindowClosed(Id),
    ExitApp,
    Config(Id, ConfigEvent),
    Capture(Id, CaptureEvent),
}
