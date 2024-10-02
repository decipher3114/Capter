use arboard::{Clipboard, ImageData};
use iced::{widget::canvas::Cache, Point};
use indexmap::IndexMap;
use tiny_skia::{FillRule, Paint, PathBuilder, Pixmap, Rect, Stroke, Transform};
use xcap::{
    image::{imageops::overlay, DynamicImage, ImageFormat, RgbaImage},
    Monitor,
};

use crate::entities::{
    capture::{
        shape::{Shape, ShapeType},
        CaptureWindow, CapturedMonitor, CapturedWindow, Endpoints, Mode, CropMode,
    },
    config::Config,
};

pub mod endpoints;
pub mod shape;

impl CaptureWindow {
    pub fn new() -> Self {
        let display = get_display();
        let windows = get_windows(display.id);
        CaptureWindow {
            cursor_position: Point::ORIGIN,
            crop_mode: CropMode::FullScreen,
            mode_desc: String::from("Fullscreen"),
            display,
            windows,
            scale_factor: 1.0,
            mode: Mode::default(),
            shape: Shape::default(),
            endpoints: Endpoints::default(),
            shapes: Vec::new(),
            cache: Cache::new(),
        }
    }

    pub fn draw_shapes(&self) -> RgbaImage {
        let scale_factor = self.scale_factor;
        let (width, height) = self.display.image.dimensions();
        let mut pixmap = Pixmap::new(width, height).unwrap();
        let transform = Transform::identity();
        for shape in self.shapes.iter() {
            let endpoints = shape.endpoints.unwrap();
            let mut paint = Paint::default();
            let color = shape.color.into_paint(shape.is_solid);
            paint.set_color(color);
            let mut stroke = Stroke::default();
            if !shape.is_filled {
                stroke.width = shape.stroke_width.f32()
            }
            match shape.shape_type {
                ShapeType::Rectangle | ShapeType::Ellipse => {
                    let (top_left, bottom_right) = endpoints.normalize();
                    let (x, y) = ((top_left.x * scale_factor), (top_left.y * scale_factor));
                    let size = bottom_right - top_left;
                    let w = size.x * scale_factor;
                    let h = size.y * scale_factor;
                    let rect = Rect::from_xywh(x, y, w, h).unwrap();
                    if matches!(shape.shape_type, ShapeType::Rectangle) {
                        let path = PathBuilder::from_rect(rect);
                        if shape.is_filled {
                            pixmap.fill_path(&path, &paint, FillRule::EvenOdd, transform, None);
                        } else {
                            pixmap.stroke_path(&path, &paint, &stroke, transform, None);
                        }
                    } else {
                        let path = PathBuilder::from_oval(rect).unwrap();
                        if shape.is_filled {
                            pixmap.fill_path(&path, &paint, FillRule::EvenOdd, transform, None);
                        } else {
                            pixmap.stroke_path(&path, &paint, &stroke, transform, None);
                        }
                    }
                }
                ShapeType::Line => {
                    let mut builder = PathBuilder::new();
                    builder.move_to(endpoints.initial_pt.x * scale_factor, endpoints.initial_pt.y * scale_factor);
                    builder.line_to(endpoints.final_pt.x * scale_factor, endpoints.final_pt.y * scale_factor);
                    let path = builder.finish().unwrap();
                    pixmap.stroke_path(&path, &paint, &stroke, transform, None);
                }
            }
        }
        RgbaImage::from_vec(width, height, pixmap.take()).unwrap()
    }

    pub fn take_screenshot(mut self, config: &Config) {
        let (img_width, img_height) = self.display.image.dimensions();
        let top = self.draw_shapes();

        let final_image = match self.crop_mode {
            CropMode::FullScreen => {
                let mut base = self.display.image;
                overlay(&mut base, &top, 0, 0);
                base
            }
            CropMode::SpecificWindow(id) => {
                let window = self.windows.swap_remove(&id).unwrap();

                let x = window.x;
                let y = window.y;
                let width = window.width;
                let height = window.height;
                let window_img = window.image.clone();

                let mut base = RgbaImage::new(img_width, img_height);
                overlay(&mut base, &window_img, x as i64, y as i64);
                overlay(&mut base, &top, 0, 0);
                let x = if x < 0 { 0u32 } else { x as u32 };
                let y = if y < 0 { 0u32 } else { y as u32 };
                let final_image = DynamicImage::from(base)
                    .crop_imm(x, y, width, height)
                    .into_rgba8();
                final_image
            }
            CropMode::ManualSelection | CropMode::SelectionInProgress => {
                let (top_left, bottom_right) = self.endpoints.normalize();
                let x = top_left.x * self.scale_factor;
                let y = top_left.y * self.scale_factor;
                let size = bottom_right - top_left;
                let width = size.x * self.scale_factor;
                let height = size.y * self.scale_factor;
                let mut base = self.display.image;
                overlay(&mut base, &top, 0, 0);
                let final_image = DynamicImage::from(base)
                    .crop_imm(x as u32, y as u32, width as u32, height as u32)
                    .into_rgba8();
                final_image
            }
        };

        save_image(config, final_image);
    }
}

pub fn get_display() -> CapturedMonitor {
    let monitors = Monitor::all().unwrap();

    let monitor = monitors
        .into_iter()
        .find(|monitor| monitor.is_primary())
        .unwrap();

    CapturedMonitor {
        id: monitor.id(),
        image: monitor.capture_image().unwrap(),
    }
}

pub fn get_windows(monitor_id: u32) -> IndexMap<u32, CapturedWindow> {
    let all_windows = xcap::Window::all().unwrap();

    let valid_windows = all_windows
        .into_iter()
        .filter_map(|window| {
            if window.current_monitor().id() == monitor_id
                && !window.is_minimized()
                && window.width() != 0
                && window.height() != 0
                && window.title() != ""
            {
                Some((
                    window.id(),
                    CapturedWindow {
                        name: window.app_name().to_string(),
                        x: window.x(),
                        y: window.y(),
                        width: window.width(),
                        height: window.height(),
                        image: window.capture_image().unwrap(),
                    },
                ))
            } else {
                None
            }
        })
        .collect();

    valid_windows
}

fn save_image(config: &Config, image: RgbaImage) {
    let date = chrono::Local::now().format("%Y-%m-%d-%H-%M-%S");

    let image_path = format!("{}\\Capture_{}.png", config.directory, date);

    Clipboard::new()
        .unwrap()
        .set_image(ImageData {
            width: image.width() as usize,
            height: image.height() as usize,
            bytes: std::borrow::Cow::Borrowed(image.as_raw()),
        })
        .unwrap();

    image
        .save_with_format(image_path, ImageFormat::Png)
        .unwrap();
}
