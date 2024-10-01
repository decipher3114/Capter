use iced::{widget::canvas, Point};
use shape::{Shape, ShapeColor, ShapeStroke, ShapeType};
use xcap::image::RgbaImage;

pub mod shape;

pub struct CaptureWindow {
    pub cursor_position: Point,
    pub image: RgbaImage,
    pub scale_factor: f32,
    pub mode: Mode,
    pub shape: Shape,
    pub endpoints: Endpoints,
    pub shapes: Vec<Shape>,
    pub cache: canvas::Cache,
}

#[derive(Debug, Clone)]
pub enum CaptureEvent {
    Cancel,
    Done,
    ChooseShapeType(ShapeType, bool),
    ChangeStroke(ShapeStroke),
    ChangeColor(ShapeColor),
    SetInitialPoint,
    UpdateCurrentPosition(Point),
    SetFinalPoint,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Endpoints {
    pub initial_pt: Option<Point>,
    pub final_pt: Option<Point>,
}

#[derive(Debug, Default, PartialEq)]
pub enum Mode {
    Draw,
    #[default]
    Crop,
}
