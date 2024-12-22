use arboard::{Clipboard, ImageData};
use iced::{widget::canvas::Cache, Point};
use indexmap::IndexMap;
use tiny_skia::{FillRule, Paint, PathBuilder, Pixmap, Rect, Stroke, Transform};
use xcap::{
    image::{imageops::overlay, DynamicImage, ImageFormat, RgbaImage},
    Monitor,
};

use super::{
    models::{CapturedWindow, Mode, SelectionMode, Shape, DrawingTool},
    utils::{normalize, resolve_arrow_points},
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
            image,
            windows,
            selection_mode: SelectionMode::FullScreen,
            mode: Mode::default(),
            shape: Shape::default(),
            shapes: Vec::new(),
            cache: Cache::new(),
        }
    }

    pub fn take_screenshot(self, directory: String) {
        let top = draw_shapes(&self.image, self.shapes);

        let final_image = match self.selection_mode {
            SelectionMode::FullScreen | SelectionMode::InProgress(_) => {
                let mut base = self.image;
                overlay(&mut base, &top, 0, 0);
                base
            }
            SelectionMode::Window(id) => {
                let window = self.windows.get(&id).unwrap();

                let x = window.x;
                let y = window.y;
                let width = window.width;
                let height = window.height;
                let window_img = &window.image;
                let (img_width, img_height) = self.image.dimensions();
                let mut base = RgbaImage::new(img_width, img_height);
                overlay(&mut base, window_img, x as i64, y as i64);
                overlay(&mut base, &top, 0, 0);
                let x = if x < 0 { 0u32 } else { x as u32 };
                let y = if y < 0 { 0u32 } else { y as u32 };
                DynamicImage::from(base)
                    .crop_imm(x, y, width, height)
                    .into_rgba8()
            }
            SelectionMode::Area(points) => {
                let (top_left, bottom_right) = (points[0], points[1]);
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

    pub fn auto_detect_area(&mut self) {
        if let Some(id) = self.windows.iter().find_map(|(id, window)| {
            let top_left = Point::new(window.x as f32, window.y as f32);
            let bottom_right = Point::new(
                (window.x + window.width as i32) as f32,
                (window.y + window.height as i32) as f32,
            );
            if (top_left.x..bottom_right.x).contains(&(self.cursor_position.x))
                && (top_left.y..bottom_right.y).contains(&(self.cursor_position.y))
            {
                Some(id)
            } else {
                None
            }
        }) {
            self.selection_mode = SelectionMode::Window(*id);
        } else {
            self.selection_mode = SelectionMode::FullScreen;
        }
    }
}

pub fn draw_shapes(image: &RgbaImage, shapes: Vec<Shape>) -> RgbaImage {
    let (width, height) = image.dimensions();
    let mut pixmap = Pixmap::new(width, height).unwrap();
    let transform = Transform::identity();
    for shape in shapes.iter() {
        let points = shape.points.as_slice();
        let mut paint = Paint::default();
        let color = shape.color.into_paint(shape.is_solid);
        paint.set_color(color);
        let mut stroke = Stroke::default();
        if !shape.is_filled {
            stroke.width = shape.stroke_width.f32()
        }
        match shape.tool {
            DrawingTool::Rectangle | DrawingTool::Ellipse => {
                let (top_left, bottom_right) = normalize(points[0], points[1]);
                let (x, y) = (top_left.x, top_left.y);
                let size = bottom_right - top_left;
                let w = size.x;
                let h = size.y;
                let rect = Rect::from_xywh(x, y, w, h).unwrap();
                if matches!(shape.tool, DrawingTool::Rectangle) {
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
            DrawingTool::FreeHand => {
                let mut builder = PathBuilder::new();
                builder.move_to(points[0].x, points[0].y);
                shape.points.iter().for_each(|point| {
                    builder.line_to(point.x, point.y);
                });
                let path = builder.finish().unwrap();
                pixmap.stroke_path(&path, &paint, &stroke, transform, None);
            }
            DrawingTool::Line => {
                let (initial_pt, final_pt) = (points[0], points[1]);
                let mut builder = PathBuilder::new();
                builder.move_to(initial_pt.x, initial_pt.y);
                builder.line_to(final_pt.x, final_pt.y);
                let path = builder.finish().unwrap();
                pixmap.stroke_path(&path, &paint, &stroke, transform, None);
            }
            DrawingTool::Arrow => {
                let (initial_pt, final_pt) = (points[0], points[1]);
                let (right_pt, left_pt) = resolve_arrow_points(initial_pt, final_pt);
                let mut builder = PathBuilder::new();
                builder.move_to(initial_pt.x, initial_pt.y);
                builder.line_to(final_pt.x, final_pt.y);
                builder.move_to(right_pt.x, right_pt.y);
                builder.line_to(final_pt.x, final_pt.y);
                builder.line_to(left_pt.x, left_pt.y);
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

    #[cfg(target_os = "windows")]
    let image_path = format!("{}\\Capture_{}.png", directory, date);

    #[cfg(not(target_os = "windows"))]
    let image_path = format!("{}/Capture_{}.png", directory, date);

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
