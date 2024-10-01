use std::collections::BTreeMap;

use iced::window::Id;
use tray_icon::TrayIcon;

use crate::entities::{
    capture::CaptureEvent,
    config::{Config, ConfigEvent},
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
    GetScaleFactor(Id, f32),
    OpenCaptureWindow,
    CaptureFullscreen,
    CaptureWindow,
    Done,
    Cancel,
    RequestClose,
    WindowClosed(Id),
    ExitApp,
    Config(Id, ConfigEvent),
    Capture(Id, CaptureEvent),
}
