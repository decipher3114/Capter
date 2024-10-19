use arboard::{Clipboard, ImageData};
use iced::{widget::canvas::Cache, Point};
use indexmap::IndexMap;
use tiny_skia::{FillRule, Paint, PathBuilder, Pixmap, Rect, Stroke, Transform};
use xcap::{
    image::{imageops::overlay, DynamicImage, ImageFormat, RgbaImage},
    Monitor,
};

use super::{
    models::{CapturedWindow, CropMode, Endpoints, Mode, Shape, ShapeType},
    CaptureWindow,
};

impl CaptureWindow {
    pub fn new(monitor: Monitor) -> Self {
        let id = monitor.id();
        let image = monitor.capture_image().unwrap();
        let windows = get_windows(id);
        let scale_factor = monitor.scale_factor();
        CaptureWindow {
            scale_factor,
            cursor_position: Point::ORIGIN,
            crop_mode: CropMode::FullScreen,
            mode_desc: String::from("Fullscreen"),
            image,
            windows,
            mode: Mode::default(),
            shape: Shape::default(),
            endpoints: Endpoints::default(),
            shapes: Vec::new(),
            cache: Cache::new(),
        }
    }

    pub fn take_screenshot(self, directory: String) {
        let (img_width, img_height) = self.image.dimensions();
        let top = draw_shapes(&self.image, self.shapes);

        let final_image = match self.crop_mode {
            CropMode::FullScreen => {
                let mut base = self.image;
                overlay(&mut base, &top, 0, 0);
                base
            }
            CropMode::SpecificWindow(id) => {
                let window = self.windows.get(&id).unwrap();

                let x = window.x;
                let y = window.y;
                let width = window.width;
                let height = window.height;
                let window_img = &window.image;

                let mut base = RgbaImage::new(img_width, img_height);
                overlay(&mut base, window_img, x as i64, y as i64);
                overlay(&mut base, &top, 0, 0);
                let x = if x < 0 { 0u32 } else { x as u32 };
                let y = if y < 0 { 0u32 } else { y as u32 };
                DynamicImage::from(base)
                    .crop_imm(x, y, width, height)
                    .into_rgba8()
            }
            CropMode::ManualSelection | CropMode::SelectionInProgress => {
                let (top_left, bottom_right) = self.endpoints.normalize();
                let x = top_left.x;
                let y = top_left.y;
                let size = bottom_right - top_left;
                let width = size.x;
                let height = size.y;
                let mut base = self.image;
                overlay(&mut base, &top, 0, 0);
                DynamicImage::from(base)
                    .crop_imm(x as u32, y as u32, width as u32, height as u32)
                    .into_rgba8()
            }
        };

        save_image(final_image, directory);
    }
}

pub fn draw_shapes(image: &RgbaImage, shapes: Vec<Shape>) -> RgbaImage {
    let (width, height) = image.dimensions();
    let mut pixmap = Pixmap::new(width, height).unwrap();
    let transform = Transform::identity();
    for shape in shapes.iter() {
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
                let (x, y) = (top_left.x, top_left.y);
                let size = bottom_right - top_left;
                let w = size.x;
                let h = size.y;
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
                builder.move_to(endpoints.initial_pt.x, endpoints.initial_pt.y);
                builder.line_to(endpoints.final_pt.x, endpoints.final_pt.y);
                let path = builder.finish().unwrap();
                pixmap.stroke_path(&path, &paint, &stroke, transform, None);
            }
        }
    }
    RgbaImage::from_vec(width, height, pixmap.take()).unwrap()
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

fn save_image(image: RgbaImage, directory: String) {
    let date = chrono::Local::now().format("%Y-%m-%d-%H-%M-%S");

    let image_path = format!("{}\\Capture_{}.png", directory, date);

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
