use iced::{Point, Size, Vector};

use crate::consts::{
    ARROW_ICON, FILLED_ELLIPSE_ICON, FILLED_RECTANGLE_ICON, FREE_HAND_ICON, HIGHLIGHTER_ICON,
    HOLLOW_ELLIPSE_ICON, HOLLOW_RECTANGLE_ICON, LINE_ICON, TEXT_ICON,
};

pub const STROKE_WIDHT_FACTOR: u32 = 2;
pub const FONT_SIZE_FACTOR: u32 = 12;

#[derive(Debug, Clone)]
pub struct DrawElement {
    pub tool: Tool,
    pub color: ToolColor,
    pub size: u32,
}

impl Default for DrawElement {
    fn default() -> Self {
        Self {
            tool: Tool::default(),
            color: ToolColor::default(),
            size: 3,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Tool {
    Rectangle {
        top_left: Point,
        bottom_right: Point,
        size: Size,
        filled: bool,
        opaque: bool,
    },
    Ellipse {
        center: Point,
        radii: Vector,
        filled: bool,
    },
    FreeHand {
        points: Vec<Point>,
    },
    Line {
        start: Point,
        end: Point,
    },
    Arrow {
        start: Point,
        end: Point,
        right: Point,
        left: Point,
    },
    Text {
        anchor_point: Point,
        text: String,
    },
}

impl Default for Tool {
    fn default() -> Self {
        Self::Rectangle {
            top_left: Point::default(),
            bottom_right: Point::default(),
            size: Size::default(),
            filled: true,
            opaque: true,
        }
    }
}

impl PartialEq for Tool {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Self::Rectangle {
                    filled: l_filled,
                    opaque: l_opaque,
                    ..
                },
                Self::Rectangle {
                    filled: r_filled,
                    opaque: r_opaque,
                    ..
                },
            ) => l_filled == r_filled && l_opaque == r_opaque,
            (
                Self::Ellipse {
                    filled: l_filled, ..
                },
                Self::Ellipse {
                    filled: r_filled, ..
                },
            ) => l_filled == r_filled,
            (Self::FreeHand { .. }, Self::FreeHand { .. }) => true,
            (Self::Line { .. }, Self::Line { .. }) => true,
            (Self::Arrow { .. }, Self::Arrow { .. }) => true,
            (Self::Text { .. }, Self::Text { .. }) => true,
            _ => false,
        }
    }
}

impl Tool {
    pub const ALL: [Tool; 9] = [
        Self::Rectangle {
            top_left: Point::ORIGIN,
            bottom_right: Point::ORIGIN,
            size: Size::ZERO,
            filled: true,
            opaque: true,
        },
        Self::Rectangle {
            top_left: Point::ORIGIN,
            bottom_right: Point::ORIGIN,
            size: Size::ZERO,
            filled: false,
            opaque: true,
        },
        Self::Ellipse {
            center: Point::ORIGIN,
            radii: Vector::ZERO,
            filled: true,
        },
        Self::Ellipse {
            center: Point::ORIGIN,
            radii: Vector::ZERO,
            filled: false,
        },
        Self::FreeHand { points: Vec::new() },
        Self::Line {
            start: Point::ORIGIN,
            end: Point::ORIGIN,
        },
        Self::Arrow {
            start: Point::ORIGIN,
            end: Point::ORIGIN,
            right: Point::ORIGIN,
            left: Point::ORIGIN,
        },
        Self::Rectangle {
            top_left: Point::ORIGIN,
            bottom_right: Point::ORIGIN,
            size: Size::ZERO,
            filled: true,
            opaque: false,
        },
        Self::Text {
            anchor_point: Point::ORIGIN,
            text: String::new(),
        },
    ];

    pub fn icon(&self) -> String {
        match self {
            Tool::Rectangle {
                filled: true,
                opaque: true,
                ..
            } => FILLED_RECTANGLE_ICON,
            Tool::Rectangle {
                filled: false,
                opaque: true,
                ..
            } => HOLLOW_RECTANGLE_ICON,
            Tool::Rectangle {
                filled: true,
                opaque: false,
                ..
            } => HIGHLIGHTER_ICON,
            Tool::Ellipse { filled: true, .. } => FILLED_ELLIPSE_ICON,
            Tool::Ellipse { filled: false, .. } => HOLLOW_ELLIPSE_ICON,
            Tool::FreeHand { .. } => FREE_HAND_ICON,
            Tool::Line { .. } => LINE_ICON,
            Tool::Arrow { .. } => ARROW_ICON,
            Tool::Text { .. } => TEXT_ICON,
            _ => ' ',
        }
        .to_string()
    }

    pub fn xml_tag(&self) -> String {
        match self {
            Tool::Rectangle { .. } => "rect",
            Tool::Ellipse { .. } => "ellipse",
            Tool::FreeHand { .. } => "polyline",
            Tool::Line { .. } | Tool::Arrow { .. } => "line",
            Tool::Text { .. } => "text",
        }
        .to_string()
    }

    pub fn reset(&mut self) {
        if let Some(tool) = Self::ALL.into_iter().find(|tool| tool == self) {
            *self = tool
        };
    }

    pub fn initiate(&mut self, point: Point) {
        match self {
            Self::Rectangle {
                top_left,
                bottom_right,
                ..
            } => {
                *top_left = point;
                *bottom_right = point;
            }
            Self::Ellipse { center, .. } => {
                *center = point;
            }
            Self::FreeHand { points } => {
                points.push(point);
            }
            Self::Line { start, end, .. } => {
                *start = point;
                *end = point;
            }
            Self::Arrow { start, end, .. } => {
                *start = point;
                *end = point;
            }
            Self::Text {
                anchor_point: anchor,
                ..
            } => {
                *anchor = point;
            }
        }
    }

    pub fn update(&mut self, initial_pt: Point, final_pt: Point) {
        match self {
            Self::Rectangle {
                top_left,
                bottom_right,
                size,
                ..
            } => {
                *top_left = Point::new(initial_pt.x.min(final_pt.x), initial_pt.y.min(final_pt.y));
                *bottom_right =
                    Point::new(initial_pt.x.max(final_pt.x), initial_pt.y.max(final_pt.y));
                *size = Size::new(bottom_right.x - top_left.x, bottom_right.y - top_left.y);
            }
            Self::Ellipse { center, radii, .. } => {
                *center = Point::new(
                    (initial_pt.x + final_pt.x) / 2.0,
                    (initial_pt.y + final_pt.y) / 2.0,
                );
                *radii = Vector::new(
                    (final_pt.x - initial_pt.x) / 2.0,
                    (final_pt.y - initial_pt.y) / 2.0,
                );
            }
            Self::FreeHand { points } => {
                points.push(final_pt);
            }
            Self::Line { end, .. } => {
                *end = final_pt;
            }
            Self::Arrow {
                start,
                end,
                right,
                left,
            } => {
                *end = final_pt;
                let line = final_pt - *start;
                let length = final_pt.distance(*start);
                let size = if length < 60.0 { length / 2.0 } else { 30.0 };
                let rad = line.y.atan2(line.x);

                *right = Point::new(
                    final_pt.x - size * (rad - std::f32::consts::PI / 5.0).cos(),
                    final_pt.y - size * (rad - std::f32::consts::PI / 5.0).sin(),
                );
                *left = Point::new(
                    final_pt.x - size * (rad + std::f32::consts::PI / 5.0).cos(),
                    final_pt.y - size * (rad + std::f32::consts::PI / 5.0).sin(),
                );
            }
            _ => {}
        }
    }

    pub fn scale(&mut self, scale_factor: f32) {
        match self {
            Tool::Rectangle {
                top_left,
                bottom_right,
                size,
                ..
            } => {
                *top_left = Point::new(top_left.x * scale_factor, top_left.y * scale_factor);
                *bottom_right =
                    Point::new(bottom_right.x * scale_factor, bottom_right.y * scale_factor);
                *size = Size::new(size.width * scale_factor, size.height * scale_factor);
            }
            Tool::Ellipse { center, radii, .. } => {
                *center = Point::new(center.x * scale_factor, center.y * scale_factor);
                *radii = Vector::new(radii.x * scale_factor, radii.y * scale_factor);
            }
            Tool::FreeHand { points } => {
                *points = points
                    .iter()
                    .map(|point| Point::new(point.x * scale_factor, point.y * scale_factor))
                    .collect();
            }
            Tool::Line { start, end, .. } => {
                *start = Point::new(start.x * scale_factor, start.y * scale_factor);
                *end = Point::new(end.x * scale_factor, end.y * scale_factor);
            }
            Tool::Arrow {
                start,
                end,
                right,
                left,
                ..
            } => {
                *start = Point::new(start.x * scale_factor, start.y * scale_factor);
                *end = Point::new(end.x * scale_factor, end.y * scale_factor);
                *right = Point::new(right.x * scale_factor, right.y * scale_factor);
                *left = Point::new(left.x * scale_factor, left.y * scale_factor);
            }
            Tool::Text {
                anchor_point: mid_point,
                ..
            } => {
                *mid_point = Point::new(mid_point.x * scale_factor, mid_point.y * scale_factor);
            }
        };
    }

    pub fn update_text(&mut self, text: String) {
        if let Self::Text { text: old_text, .. } = self {
            *old_text = text;
        }
    }

    pub fn is_valid(&self) -> bool {
        match self {
            Self::Rectangle { size, .. } => size != &Size::ZERO,
            Self::Ellipse { radii, .. } => radii != &Vector::ZERO,
            Self::FreeHand { points } => points.len() > 1,
            Self::Line { start, end } => start != end,
            Self::Arrow { start, end, .. } => start != end,
            Self::Text { text, .. } => !text.is_empty(),
        }
    }

    pub fn need_size(&self) -> bool {
        match self {
            Self::Rectangle {
                filled: is_filled, ..
            }
            | Self::Ellipse {
                filled: is_filled, ..
            } => !*is_filled,
            Self::Line { .. } | Self::FreeHand { .. } | Self::Text { .. } => true,
            _ => false,
        }
    }

    pub fn is_text_tool(&self) -> bool {
        matches!(self, Self::Text { .. })
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

impl From<ToolColor> for iced::Color {
    fn from(value: ToolColor) -> Self {
        match value {
            ToolColor::Red => iced::Color::from_rgb8(255, 0, 0),
            ToolColor::Green => iced::Color::from_rgb8(0, 255, 0),
            ToolColor::Blue => iced::Color::from_rgb8(0, 0, 255),
            ToolColor::Yellow => iced::Color::from_rgb8(255, 255, 0),
            ToolColor::Black => iced::Color::from_rgb8(0, 0, 0),
            ToolColor::White => iced::Color::from_rgb8(255, 255, 255),
        }
    }
}

impl ToolColor {
    pub const ALL: [ToolColor; 6] = [
        Self::Red,
        Self::Green,
        Self::Blue,
        Self::Yellow,
        Self::Black,
        Self::White,
    ];

    pub fn icon(&self) -> String {
        FILLED_RECTANGLE_ICON.to_string()
    }

    pub fn as_hex(&self) -> String {
        match self {
            ToolColor::Red => "#FF0000",
            ToolColor::Green => "#00FF00",
            ToolColor::Blue => "#0000FF",
            ToolColor::Yellow => "#FFFF00",
            ToolColor::Black => "#000000",
            ToolColor::White => "#FFFFFF",
        }
        .to_string()
    }

    pub fn into_translucent_color(self) -> iced::Color {
        iced::Color::from(self).scale_alpha(0.3)
    }
}

#[derive(Debug, Default, Clone)]
pub enum DrawState {
    #[default]
    Idle,
    InProgress {
        initial_pt: Point,
        final_pt: Point,
    },
    TextInput,
}

impl DrawState {
    pub fn is_idle(&self) -> bool {
        !matches!(self, Self::InProgress { .. })
    }

    pub fn is_waiting_for_input(&self) -> bool {
        matches!(self, Self::TextInput)
    }
}
