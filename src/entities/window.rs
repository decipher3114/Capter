use crate::entities::{config::Config, freeform::FreeForm};

#[derive(Debug, Clone)]
pub enum WindowType {
    ConfigureWindow(Config),
    FreeFormWindow(FreeForm),
}
