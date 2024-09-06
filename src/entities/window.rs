use crate::entities::{config::ConfigureWindow, freeform::FreeFormWindow};

#[derive(Debug)]
pub enum WindowType {
    ConfigureWindow(ConfigureWindow),
    FreeFormWindow(FreeFormWindow),
}
