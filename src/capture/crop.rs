use std::rc::Rc;

use iced::Point;

use super::CapturedWindow;

#[derive(Debug, Default)]
pub enum CropState {
    #[default]
    FullScreen,
    Window(Rc<CapturedWindow>),
    InProgress {
        start: Point,
        end: Point,
    },
    Area,
    None,
}

impl CropState {
    pub fn is_idle(&self) -> bool {
        !matches!(self, Self::InProgress { .. })
    }
}
