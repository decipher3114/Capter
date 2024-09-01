use std::collections::BTreeMap;

use iced::window::Id;

use crate::entities::{
    config::{Config, ConfigEvent},
    freeform::FreeFormEvent,
    window::WindowType,
};

#[derive(Default)]
pub struct App {
    pub config: Config,
    pub windows: BTreeMap<Id, WindowType>,
}

#[derive(Debug, Clone)]
pub enum AppEvent {
    OpenConfigureWindow,
    UpdateConfig,
    InitiateFreeForm,
    CaptureFullscreen,
    CaptureWindow,
    CloseWindow,
    WindowClosed(Id),
    ExitApp,
    ConfigAppEvent(Id, ConfigEvent),
    FreeFormAppEvent(Id, FreeFormEvent),
}
