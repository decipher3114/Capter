use crate::entities::capture::shape::{ShapeColor, ShapeStroke};

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
            Self::Medium => 4.0,
            Self::Broad => 6.0,
        }
    }
}
