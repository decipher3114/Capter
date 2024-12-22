use iced::Point;
use xcap::image::RgbaImage;

#[derive(Debug, Default)]
pub enum SelectionMode {
    #[default]
    FullScreen,
    Window(u32),
    InProgress(Point),
    Area([Point; 2]),
}

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

#[derive(Debug, Default, Clone)]
pub struct Shape {
    pub tool: DrawingTool,
    pub points: Vec<Point>,
    pub color: ToolColor,
    pub is_filled: bool,
    pub is_solid: bool,
    pub stroke_width: StrokeWidth,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum DrawingTool {
    #[default]
    Rectangle,
    Ellipse,
    FreeHand,
    Line,
    Arrow,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum ToolColor {
    #[default]
    Red,
    Green,
    Blue,
    Yellow,
    Black,
    White,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum StrokeWidth {
    Thin,
    #[default]
    Medium,
    Broad,
}

impl ToolColor {
    pub fn into_iced_color(self, solid: bool) -> iced::Color {
        let opacity = if solid { 1.0 } else { 0.3 };
        match self {
            ToolColor::Red => iced::Color::from_rgba8(255, 0, 0, opacity),
            ToolColor::Green => iced::Color::from_rgba8(0, 255, 0, opacity),
            ToolColor::Blue => iced::Color::from_rgba8(0, 0, 255, opacity),
            ToolColor::Yellow => iced::Color::from_rgba8(255, 255, 0, opacity),
            ToolColor::Black => iced::Color::from_rgba8(0, 0, 0, opacity),
            ToolColor::White => iced::Color::from_rgba8(255, 255, 255, opacity),
        }
    }

    pub fn into_paint(self, solid: bool) -> tiny_skia::Color {
        let opacity = if solid { 255 } else { 77 };
        match self {
            ToolColor::Red => tiny_skia::Color::from_rgba8(255, 0, 0, opacity),
            ToolColor::Green => tiny_skia::Color::from_rgba8(0, 255, 0, opacity),
            ToolColor::Blue => tiny_skia::Color::from_rgba8(0, 0, 255, opacity),
            ToolColor::Yellow => tiny_skia::Color::from_rgba8(255, 255, 0, opacity),
            ToolColor::Black => tiny_skia::Color::from_rgba8(0, 0, 0, opacity),
            ToolColor::White => tiny_skia::Color::from_rgba8(255, 255, 255, opacity),
        }
    }
}

impl StrokeWidth {
    pub fn f32(&self) -> f32 {
        match self {
            Self::Thin => 2.0,
            Self::Medium => 5.0,
            Self::Broad => 8.0,
        }
    }
}
