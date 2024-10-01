use super::Endpoints;

#[derive(Debug, Default, Clone, Copy)]
pub struct Shape {
    pub shape_type: ShapeType,
    pub endpoints: Endpoints,
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
