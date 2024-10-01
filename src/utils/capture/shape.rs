use iced::color;

use crate::entities::capture::shape::{ShapeColor, ShapeStroke};

impl ShapeColor {
    pub fn into_iced_color(self) -> iced::Color {
        match self {
            Self::Red => color!(0xff0000),
            Self::Blue => color!(0x0000ff),
            Self::Green => color!(0x00ff00),
            Self::Yellow => color!(0xffff00),
            Self::Black => color!(0x000000),
            Self::White => color!(0xffffff),
        }
    }

    pub fn into_paint(self) -> tiny_skia::Color {
        match self {
            ShapeColor::Red => tiny_skia::Color::from_rgba8(255, 0, 0, 255),
            ShapeColor::Green => tiny_skia::Color::from_rgba8(0, 255, 0, 255),
            ShapeColor::Blue => tiny_skia::Color::from_rgba8(0, 0, 255, 255),
            ShapeColor::Yellow => tiny_skia::Color::from_rgba8(255, 255, 0, 255),
            ShapeColor::Black => tiny_skia::Color::from_rgba8(0, 0, 0, 255),
            ShapeColor::White => tiny_skia::Color::from_rgba8(255, 255, 255, 255),
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
