use crate::entities::{capture::CaptureWindow, config::ConfigureWindow};

pub enum WindowType {
    Configure(Box<ConfigureWindow>),
    Capture(Box<CaptureWindow>),
}
