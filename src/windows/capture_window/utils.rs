use std::f32::consts::PI;

use arboard::{Clipboard, ImageData};
use edit_xml::{Document, ElementBuilder, WriteOptions};
use iced::Point;
use indexmap::IndexMap;
use resvg::{
    render,
    tiny_skia::Pixmap,
    usvg::{Options, Transform, Tree},
};
use xcap::{
    image::{imageops::overlay, DynamicImage, ImageFormat, RgbaImage},
    Monitor,
};

use crate::consts::{FONT_MEDIUM_TTF, FONT_NAME};

use super::{
    models::{CapturedWindow, DrawingTool, SelectionMode, Shape},
    CaptureWindow,
};

impl CaptureWindow {
    pub fn get_content(monitor: Monitor) -> (RgbaImage, IndexMap<u32, CapturedWindow>, f32) {
        let image = monitor.capture_image().unwrap();

        let valid_windows = xcap::Window::all()
            .unwrap()
            .into_iter()
            .filter_map(|window| {
                if window.current_monitor().ok()?.id().ok()? == monitor.id().ok()?
                    && !window.is_minimized().ok()?
                    && window.width().ok()? != 0
                    && window.height().ok()? != 0
                    && window.title().ok()? != ""
                {
                    Some((
                        window.id().ok()?,
                        CapturedWindow {
                            name: window.app_name().ok()?.to_string(),
                            x: window.x().ok()?,
                            y: window.y().ok()?,
                            width: window.width().ok()?,
                            height: window.height().ok()?,
                            image: window.capture_image().ok()?,
                        },
                    ))
                } else {
                    None
                }
            })
            .collect();
        let scale_factor = monitor
            .scale_factor()
            .expect("Scale factor must be present for existing monitor");
        (image, valid_windows, scale_factor)
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

pub fn normalize(initial_pt: Point, final_pt: Point) -> (Point, Point) {
    let (initial_pt, final_pt) = (initial_pt, final_pt);
    let (mut start, mut end) = (initial_pt, final_pt);
    if initial_pt.x > final_pt.x {
        (start.x, end.x) = (final_pt.x, initial_pt.x)
    };
    if initial_pt.y > final_pt.y {
        (start.y, end.y) = (final_pt.y, initial_pt.y)
    };

    (start, end)
}

pub fn resolve_arrow_points(initial_pt: Point, final_pt: Point) -> (Point, Point) {
    let line = final_pt - initial_pt;
    let length = f32::sqrt(line.x.powf(2.0) + line.y.powf(2.0));
    let size = if length < 60.0 { length / 2.0 } else { 30.0 };
    let rad = line.y.atan2(line.x);
    let right_pt = Point::new(
        final_pt.x - size * (rad - PI / 5.0).cos(),
        final_pt.y - size * (rad - PI / 5.0).sin(),
    );
    let left_pt = Point::new(
        final_pt.x - size * (rad + PI / 5.0).cos(),
        final_pt.y - size * (rad + PI / 5.0).sin(),
    );
    (right_pt, left_pt)
}

pub fn draw_shapes(image: &RgbaImage, shapes: Vec<Shape>) -> RgbaImage {
    let (width, height) = image.dimensions();
    let mut pixmap = Pixmap::new(width, height).unwrap();
    let transform = Transform::identity();
    let mut xml = Document::new();
    let svg = ElementBuilder::new("svg")
        .attribute("xmlns", "http://www.w3.org/2000/svg")
        .attribute("width", width.to_string())
        .attribute("height", height.to_string())
        .push_to_root_node(&mut xml);
    for shape in shapes.iter() {
        let element = ElementBuilder::new(shape.tool);
        let points = shape.points.as_slice();
        let color = shape.color;
        let drawn_element = match shape.tool {
            DrawingTool::Rectangle => {
                let (top_left, bottom_right) = normalize(points[0], points[1]);
                let size = bottom_right - top_left;
                let (width, height) = (size.x, size.y);
                element
                    .attribute("x", top_left.x.to_string())
                    .attribute("y", top_left.y.to_string())
                    .attribute("width", width.to_string())
                    .attribute("height", height.to_string())
            }
            DrawingTool::Ellipse => {
                let (top_left, bottom_right) = normalize(points[0], points[1]);
                let size = bottom_right - top_left;
                let radii = (size.x / 2.0, size.y / 2.0);
                let (cx, cy) = (top_left.x + radii.0, top_left.y + radii.1);
                let (rx, ry) = (radii.0, radii.1);
                element
                    .attribute("cx", cx.to_string())
                    .attribute("cy", cy.to_string())
                    .attribute("rx", rx.to_string())
                    .attribute("ry", ry.to_string())
            }
            DrawingTool::FreeHand => {
                let mut points_str = "".to_string();
                for point in points.iter() {
                    points_str.push_str(format!("{},{} ", point.x, point.y).as_str());
                }
                element.attribute("points", points_str)
            }
            DrawingTool::Line => element
                .attribute("x1", points[0].x.to_string())
                .attribute("y1", points[0].y.to_string())
                .attribute("x2", points[1].x.to_string())
                .attribute("y2", points[1].y.to_string()),
            DrawingTool::Arrow => {
                let (left_pt, right_pt) = resolve_arrow_points(points[0], points[1]);
                ElementBuilder::new("polyline")
                    .attribute(
                        "points",
                        format!(
                            "{},{}, {},{} {},{}",
                            left_pt.x, left_pt.y, points[1].x, points[1].y, right_pt.x, right_pt.y
                        )
                        .to_string(),
                    )
                    .attribute("fill", "none")
                    .attribute("stroke", color)
                    .attribute("stroke-width", shape.size.to_stroke_f32().to_string())
                    .push_to(&mut xml, svg);
                element
            }
            DrawingTool::Text => element
                .attribute("x", points[0].x.to_string())
                .attribute("y", points[0].y.to_string())
                .attribute("font-family", FONT_NAME)
                .attribute("font-size", shape.size.to_text_size_f32().to_string())
                .add_text(shape.text.clone()),
        };

        let final_element = if shape.is_filled
            && matches!(
                shape.tool,
                DrawingTool::Rectangle | DrawingTool::Ellipse | DrawingTool::Text
            ) {
            let filled = drawn_element.attribute("fill", color);
            if shape.is_solid {
                filled
            } else {
                filled.attribute("opacity", "0.3")
            }
        } else {
            drawn_element
                .attribute("fill", "none")
                .attribute("stroke", color)
                .attribute("stroke-width", shape.size.to_stroke_f32().to_string())
        };

        final_element.push_to(&mut xml, svg);
    }

    let mut options = Options::default();
    options
        .fontdb_mut()
        .load_font_data(FONT_MEDIUM_TTF.to_vec());

    let tree = Tree::from_str(
        xml.write_str_with_opts(WriteOptions {
            write_decl: false,
            ..Default::default()
        })
        .unwrap()
        .as_str(),
        &options,
    )
    .unwrap();

    render(&tree, transform, &mut pixmap.as_mut());

    RgbaImage::from_vec(width, height, pixmap.take()).unwrap()
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
