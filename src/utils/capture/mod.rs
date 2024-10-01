use arboard::{Clipboard, ImageData};
use iced::{widget::canvas::Cache, Point};
use tiny_skia::{FillRule, Paint, PathBuilder, Pixmap, Rect, Stroke, Transform};
use xcap::image::{imageops::overlay, DynamicImage, ImageFormat, RgbaImage};

use crate::{
    entities::{
        capture::{
            shape::{Shape, ShapeType},
            CaptureWindow, Endpoints, Mode,
        },
        config::Config,
    },
    utils::evaluate_points,
};

pub mod endpoints;
pub mod fullscreen;
pub mod shape;
pub mod window;

impl CaptureWindow {
    pub fn new() -> Self {
        let image = fullscreen::get_fullscreen().unwrap();
        CaptureWindow {
            cursor_position: Point::ORIGIN,
            image,
            scale_factor: 1.0,
            mode: Mode::default(),
            shape: Shape::default(),
            endpoints: Endpoints {
                initial_pt: None,
                final_pt: None,
            },
            shapes: Vec::new(),
            cache: Cache::new(),
        }
    }

    pub fn draw_shapes(&self) -> RgbaImage {
        let scale_factor = self.scale_factor;
        let (width, height) = self.image.dimensions();
        let mut pixmap = Pixmap::new(width, height).unwrap();
        let transform = Transform::identity();
        for shape in self.shapes.iter() {
            let initial_pt = shape.endpoints.initial_pt.unwrap();
            let final_pt = shape.endpoints.final_pt.unwrap();
            let mut paint = Paint::default();
            let color = shape.color.into_paint(shape.is_solid);
            paint.set_color(color);
            let mut stroke = Stroke::default();
            if !shape.is_filled {
                stroke.width = shape.stroke_width.f32()
            }
            match shape.shape_type {
                ShapeType::Rectangle | ShapeType::Ellipse => {
                    let (x, y) = ((initial_pt.x * scale_factor), (initial_pt.y * scale_factor));
                    let size = final_pt - initial_pt;
                    let w = size.x * scale_factor;
                    let h = size.y * scale_factor;
                    let rect = Rect::from_xywh(x, y, w, h).unwrap();
                    if shape.shape_type == ShapeType::Rectangle {
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
                    builder.move_to(initial_pt.x * scale_factor, initial_pt.y * scale_factor);
                    builder.line_to(final_pt.x * scale_factor, final_pt.y * scale_factor);
                    let path = builder.finish().unwrap();
                    pixmap.stroke_path(&path, &paint, &stroke, transform, None);
                }
            }
        }
        RgbaImage::from_vec(width, height, pixmap.take()).unwrap()
    }

    pub fn take_screenshot(mut self, config: &Config) {
        if let (Some(initial_pt), Some(final_pt)) =
            (self.endpoints.initial_pt, self.endpoints.final_pt)
        {
            let top = self.draw_shapes();
            overlay(&mut self.image, &top, 0, 0);
            let scale_factor = self.scale_factor;

            let (initial_pt, final_pt) = evaluate_points(initial_pt, final_pt);

            let cropped_image = DynamicImage::from(self.image).crop(
                (initial_pt.x * scale_factor) as u32,
                (initial_pt.y * scale_factor) as u32,
                ((final_pt.x - initial_pt.x) * scale_factor) as u32,
                ((final_pt.y - initial_pt.y) * scale_factor) as u32,
            );

            save_image(config, cropped_image.into_rgba8());
        }
    }
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
