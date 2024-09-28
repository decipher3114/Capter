use std::collections::BTreeMap;

use iced::window::Id;
use tray_icon::TrayIcon;

use crate::entities::{
    config::{Config, ConfigEvent},
    capture::CaptureEvent,
    window::WindowType,
};

pub struct App {
    pub _tray_icon: TrayIcon,
    pub config: Config,
    pub windows: BTreeMap<Id, WindowType>,
}

#[derive(Debug, Clone)]
pub enum AppEvent {
    OpenConfigureWindow,
    OpenDirectory,
    UpdateDirectory(Id),
    OpenCaptureWindow,
    CaptureFullscreen,
    CaptureWindow,
    CloseWindow,
    WindowClosed(Id),
    ExitApp,
    Config(Id, ConfigEvent),
    Capture(Id, CaptureEvent),
}
