mod canvas;
mod image;
mod init;
mod update;
mod view;

mod crop;
mod draw;
mod mode;

use std::rc::Rc;

use draw::{
    Tool,
    ToolColor,
};
use iced::{
    Point,
    widget::{
        canvas::Cache,
        image::Handle,
    },
};
use mode::Mode;
use xcap::image::RgbaImage;

use crate::capture::draw::DrawElements;

pub struct Capture {
    // Attributes
    scale_factor: f32,

    // Screenshot
    screenshot: RgbaImage,
    screenshot_handle: Handle,
    windows: Vec<Rc<CapturedWindow>>,

    // UI
    toolbar_at_top: bool,

    // State
    cursor_position: Point,
    mode: Mode,
    elements: DrawElements,
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
