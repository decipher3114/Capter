use capture_window::CaptureWindow;
use config_window::ConfigureWindow;

pub mod capture_window;
pub mod config_window;

pub enum AppWindow {
    Configure(Box<ConfigureWindow>),
    Capture(Box<CaptureWindow>),
}
