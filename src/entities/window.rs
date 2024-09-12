use crate::entities::{config::ConfigureWindow, crop::CropWindow};

#[derive(Debug)]
pub enum WindowType {
    ConfigureWindow(ConfigureWindow),
    CropWindow(CropWindow),
}
