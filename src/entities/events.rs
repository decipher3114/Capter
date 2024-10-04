use iced::window::Id;

use super::{capture::CaptureEvent, config::ConfigEvent};

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
