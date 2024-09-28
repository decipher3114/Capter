use crate::entities::{config::ConfigureWindow, capture::CaptureWindow};

#[derive(Debug)]
pub enum WindowType {
    Configure(ConfigureWindow),
    Capture(CaptureWindow),
}
