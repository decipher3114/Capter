use crate::entities::{capture::CaptureWindow, config::ConfigureWindow};

pub enum WindowType {
    Configure(ConfigureWindow),
    Capture(CaptureWindow),
}
