use std::rc::Rc;

use draw::{DrawElement, Tool, ToolColor};
use iced::{Point, widget::canvas::Cache};
use mode::Mode;
use xcap::image::RgbaImage;

mod canvas;
mod helper;
mod init;
mod update;
mod view;

mod crop;
mod draw;
mod mode;

pub struct Capture {
    toolbar_at_top: bool,
    scale_factor: f32,
    image: RgbaImage,
    windows: Vec<Rc<CapturedWindow>>,
    cursor_position: Point,
    mode: Mode,
    shapes: Vec<DrawElement>,
    cache: Cache,
}

#[derive(Debug, Clone)]
pub enum Message {
    MoveToolBar,
    Undo,
    Done,
    Cancel,
    ChangeTool(Tool),
    ChangeSize(u32),
    ChangeColor(ToolColor),
    UpdateText(String),
    MousePressed,
    MouseMoved(Point),
    MouseReleased,
}

pub enum Request {
    Close,
}

#[derive(Debug)]
pub struct CapturedWindow {
    pub name: String,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub screenshot: RgbaImage,
}
