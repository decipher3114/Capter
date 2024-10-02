use iced::{widget::canvas, Point};
use indexmap::IndexMap;
use shape::{Shape, ShapeColor, ShapeStroke, ShapeType};
use xcap::image::RgbaImage;

pub mod shape;

pub struct CaptureWindow {
    pub scale_factor: f32,
    pub crop_mode: CropMode,
    pub mode_desc: String,
    pub display: CapturedMonitor,
    pub windows: IndexMap<u32, CapturedWindow>,
    pub cursor_position: Point,
    pub mode: Mode,
    pub endpoints: Endpoints,
    pub shape: Shape,
    pub shapes: Vec<Shape>,
    pub cache: canvas::Cache,
}

#[derive(Debug, Clone)]
pub enum CaptureEvent {
    Undo,
    Done,
    Cancel,
    ChooseShapeType(ShapeType, bool, bool),
    ChangeStroke(ShapeStroke),
    ChangeColor(ShapeColor),
    SetInitialPoint,
    UpdateCurrentPosition(Point),
    SetFinalPoint,
}

#[derive(Debug, Default)]
pub enum CropMode {
    #[default]
    FullScreen,
    SpecificWindow(u32),
    SelectionInProgress,
    ManualSelection,
}

#[derive(Debug)]
pub struct CapturedMonitor {
    pub id: u32,
    pub image: RgbaImage,
}

#[derive(Debug)]
pub struct CapturedWindow {
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub image: RgbaImage,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Endpoints {
    pub initial_pt: Point,
    pub final_pt: Point,
}

#[derive(Debug, Default)]
pub enum Mode {
    Draw,
    #[default]
    Crop,
}
