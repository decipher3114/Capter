use iced::Point;
use xcap::image::RgbaImage;

#[derive(Debug, Default)]
pub enum CropMode {
    #[default]
    FullScreen,
    SpecificWindow(u32),
    SelectionInProgress,
    ManualSelection,
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

#[derive(Debug, Default)]
pub enum Mode {
    Draw,
    #[default]
    Crop,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Endpoints {
    pub initial_pt: Point,
    pub final_pt: Point,
}

impl Endpoints {
    pub fn normalize(self) -> (Point, Point) {
        let (initial_pt, final_pt) = (self.initial_pt, self.final_pt);
        let (mut start, mut end) = (initial_pt, final_pt);
        if initial_pt.x > final_pt.x {
            (start.x, end.x) = (final_pt.x, initial_pt.x)
        };
        if initial_pt.y > final_pt.y {
            (start.y, end.y) = (final_pt.y, initial_pt.y)
        };

        (start, end)
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Shape {
    pub shape_type: ShapeType,
    pub endpoints: Option<Endpoints>,
    pub color: ShapeColor,
    pub is_filled: bool,
    pub is_solid: bool,
    pub stroke_width: ShapeStroke,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum ShapeType {
    #[default]
    Rectangle,
    Ellipse,
    Line,
    Arrow,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum ShapeColor {
    #[default]
    Red,
    Green,
    Blue,
    Yellow,
    Black,
    White,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum ShapeStroke {
    Thin,
    #[default]
    Medium,
    Broad,
}

impl ShapeColor {
    pub fn into_iced_color(self, solid: bool) -> iced::Color {
        let opacity = if solid { 1.0 } else { 0.3 };
        match self {
            ShapeColor::Red => iced::Color::from_rgba8(255, 0, 0, opacity),
            ShapeColor::Green => iced::Color::from_rgba8(0, 255, 0, opacity),
            ShapeColor::Blue => iced::Color::from_rgba8(0, 0, 255, opacity),
            ShapeColor::Yellow => iced::Color::from_rgba8(255, 255, 0, opacity),
            ShapeColor::Black => iced::Color::from_rgba8(0, 0, 0, opacity),
            ShapeColor::White => iced::Color::from_rgba8(255, 255, 255, opacity),
        }
    }

    pub fn into_paint(self, solid: bool) -> tiny_skia::Color {
        let opacity = if solid { 255 } else { 77 };
        match self {
            ShapeColor::Red => tiny_skia::Color::from_rgba8(255, 0, 0, opacity),
            ShapeColor::Green => tiny_skia::Color::from_rgba8(0, 255, 0, opacity),
            ShapeColor::Blue => tiny_skia::Color::from_rgba8(0, 0, 255, opacity),
            ShapeColor::Yellow => tiny_skia::Color::from_rgba8(255, 255, 0, opacity),
            ShapeColor::Black => tiny_skia::Color::from_rgba8(0, 0, 0, opacity),
            ShapeColor::White => tiny_skia::Color::from_rgba8(255, 255, 255, opacity),
        }
    }
}

impl ShapeStroke {
    pub fn f32(&self) -> f32 {
        match self {
            Self::Thin => 2.0,
            Self::Medium => 5.0,
            Self::Broad => 8.0,
        }
    }
}
