use iced::Point;
use xcap::image::DynamicImage;

#[derive(Debug, Clone)]
pub struct FreeForm {
    pub cursor_position: Point,
    pub image: DynamicImage,
    pub selection_area: SelectionArea,
}

#[derive(Debug, Clone)]
pub enum FreeFormEvent {
    SetInitialPoint,
    UpdateCurrentPosition(Point),
    SetFinalPoint,
}

#[derive(Debug, Clone)]
pub struct SelectionArea {
    pub initial_pos: Option<Point>,
    pub final_pos: Option<Point>,
}
