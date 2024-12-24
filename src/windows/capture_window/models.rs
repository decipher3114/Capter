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
    pub text: String,
    pub points: Vec<Point>,
    pub color: ToolColor,
    pub is_filled: bool,
    pub is_solid: bool,
    pub size: ToolSize,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum DrawingTool {
    #[default]
    Rectangle,
    Ellipse,
    FreeHand,
    Line,
    Arrow,
    Text,
}

impl From<DrawingTool> for String {
    fn from(value: DrawingTool) -> String {
        let tool = match value {
            DrawingTool::Rectangle => "rect",
            DrawingTool::Ellipse => "ellipse",
            DrawingTool::FreeHand => "polyline",
            DrawingTool::Line => "line",
            DrawingTool::Arrow => "line",
            DrawingTool::Text => "text",
        };
        tool.to_string()
    }
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
pub enum ToolSize {
    Min,
    #[default]
    Regular,
    Max,
}

#[derive(Debug, Clone)]
pub enum KeyType {
    Backspace,
    Space,
    Char(String),
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
}

impl From<ToolColor> for String {
    fn from(value: ToolColor) -> String {
        let color = match value {
            ToolColor::Red => "#FF0000",
            ToolColor::Green => "#00FF00",
            ToolColor::Blue => "#0000FF",
            ToolColor::Yellow => "#FFFF00",
            ToolColor::Black => "#000000",
            ToolColor::White => "#FFFFFF",
        };
        color.to_string()
    }
}

impl ToolSize {
    pub fn to_stroke_f32(self) -> f32 {
        match self {
            Self::Min => 2.0,
            Self::Regular => 5.0,
            Self::Max => 8.0,
        }
    }

    pub fn to_text_size_f32(self) -> f32 {
        match self {
            Self::Min => 36.0,
            Self::Regular => 48.0,
            Self::Max => 60.0,
        }
    }
}
