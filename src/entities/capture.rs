use iced::Point;
use xcap::image::RgbaImage;

#[derive(Debug)]
pub struct CaptureWindow {
    pub cursor_position: Point,
    pub image: RgbaImage,
    pub selection_area: Area,
}

#[derive(Debug, Clone)]
pub enum CaptureEvent {
    SetInitialPoint,
    UpdateCurrentPosition(Point),
    SetFinalPoint,
}

#[derive(Debug, Clone)]
pub struct Area {
    pub initial_pos: Option<Point>,
    pub final_pos: Option<Point>,
}
