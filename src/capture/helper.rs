use std::path::{Path, PathBuf};

use anyhow::{Error, Result};
use arboard::Clipboard;
use chrono::Local;
use edit_xml::{Document, ElementBuilder};

use iced::Point;
use resvg::{tiny_skia, usvg};
use xcap::image::{
    ImageFormat, RgbaImage,
    imageops::{crop_imm, overlay},
};

use crate::consts::{FONT_NAME, MEDIUM_FONT_TTF};

use super::{
    Capture, DrawElement,
    crop::CropState,
    draw::{FONT_SIZE_FACTOR, STROKE_WIDHT_FACTOR, Tool},
    mode::Mode,
};

impl Capture {
    pub fn finalize(mut self, directory: &Path) -> Result<PathBuf> {
        if let Mode::Crop {
            top_left,
            bottom_right,
            status,
            ..
        } = self.mode
        {
            let top_left = Point::new(
                top_left.x * self.scale_factor,
                top_left.y * self.scale_factor,
            );
            let bottom_right = Point::new(
                bottom_right.x * self.scale_factor,
                bottom_right.y * self.scale_factor,
            );

            let (img_width, img_height) = self.image.dimensions();
            let annotation_overlay =
                create_annotation_overlay(img_width, img_height, self.shapes, self.scale_factor)
                    .unwrap_or(RgbaImage::new(0, 0));

            match status {
                CropState::FullScreen => {
                    overlay(&mut self.image, &annotation_overlay, 0, 0);
                }
                CropState::Window(window) => {
                    let mut base_image = RgbaImage::new(img_width, img_height);

                    overlay(
                        &mut base_image,
                        &window.screenshot,
                        top_left.x as i64,
                        top_left.y as i64,
                    );

                    overlay(&mut base_image, &annotation_overlay, 0, 0);

                    self.image = crop_imm(
                        &base_image,
                        top_left.x as u32,
                        top_left.y as u32,
                        window.width as u32,
                        window.height as u32,
                    )
                    .to_image();
                }
                CropState::Area | CropState::InProgress { .. } => {
                    let x = top_left.x;
                    let y = top_left.y;
                    let size = bottom_right - top_left;
                    let width = size.x;
                    let height = size.y;
                    overlay(&mut self.image, &annotation_overlay, 0, 0);
                    self.image =
                        crop_imm(&self.image, x as u32, y as u32, width as u32, height as u32)
                            .to_image();
                }
                CropState::None => {
                    return Err(Error::msg("Screenshot Cancelled"));
                }
            };
        }

        save_image(self.image, directory)
    }
}

pub fn create_annotation_overlay(
    width: u32,
    height: u32,
    shapes: Vec<DrawElement>,
    scale_factor: f32,
) -> Option<RgbaImage> {
    let mut pixmap = tiny_skia::Pixmap::new(width, height)?;
    let transform = usvg::Transform::identity();

    let mut xml = Document::new();

    let svg = ElementBuilder::new("svg")
        .attribute("xmlns", "http://www.w3.org/2000/svg")
        .attribute("width", width.to_string())
        .attribute("height", height.to_string())
        .push_to_root_node(&mut xml);

    for mut shape in shapes.into_iter() {
        let element = ElementBuilder::new(shape.tool.xml_tag());
        let color = shape.color;
        let stroke_width = ((shape.size * STROKE_WIDHT_FACTOR) as f32 * scale_factor).to_string();

        shape.tool.scale(scale_factor);

        match shape.tool {
            Tool::Rectangle {
                top_left,
                bottom_right: _,
                size,
                is_filled: fill,
                is_opaque: opaque,
                ..
            } => {
                let drawn_shape = element
                    .attribute("x", top_left.x.to_string())
                    .attribute("y", top_left.y.to_string())
                    .attribute("width", size.width.to_string())
                    .attribute("height", size.height.to_string());

                if fill {
                    if opaque {
                        drawn_shape.attribute("fill", color.as_hex())
                    } else {
                        drawn_shape
                            .attribute("fill", color.as_hex())
                            .attribute("opacity", "0.3")
                    }
                } else {
                    drawn_shape
                        .attribute("fill", "none")
                        .attribute("stroke", color.as_hex())
                        .attribute("stroke-width", stroke_width)
                }
                .push_to(&mut xml, svg);
            }
            Tool::Ellipse {
                center,
                radii,
                is_filled: fill,
                ..
            } => {
                let drawn_shape = element
                    .attribute("cx", center.x.to_string())
                    .attribute("cy", center.y.to_string())
                    .attribute("rx", radii.x.to_string())
                    .attribute("ry", radii.y.to_string());

                if fill {
                    drawn_shape.attribute("fill", color.as_hex())
                } else {
                    drawn_shape
                        .attribute("fill", "none")
                        .attribute("stroke", color.as_hex())
                        .attribute("stroke-width", stroke_width)
                }
                .push_to(&mut xml, svg);
            }
            Tool::FreeHand { points } => {
                let points_str = points
                    .iter()
                    .map(|point| format!("{},{}", point.x, point.y))
                    .collect::<Vec<String>>()
                    .join(" ");

                element
                    .attribute("points", points_str)
                    .attribute("fill", "none")
                    .attribute("stroke", color.as_hex())
                    .attribute("stroke-width", stroke_width)
                    .push_to(&mut xml, svg);
            }
            Tool::Line { start, end } => {
                element
                    .attribute("x1", start.x.to_string())
                    .attribute("y1", start.y.to_string())
                    .attribute("x2", end.x.to_string())
                    .attribute("y2", end.y.to_string())
                    .attribute("fill", "none")
                    .attribute("stroke", color.as_hex())
                    .attribute("stroke-width", stroke_width)
                    .push_to(&mut xml, svg);
            }
            Tool::Arrow {
                start,
                end,
                right,
                left,
            } => {
                ElementBuilder::new("polyline")
                    .attribute(
                        "points",
                        format!(
                            "{},{}, {},{} {},{}",
                            left.x, left.y, end.x, end.y, right.x, right.y
                        )
                        .to_string(),
                    )
                    .attribute("fill", "none")
                    .attribute("stroke", color.as_hex())
                    .attribute("stroke-width", stroke_width.clone())
                    .push_to(&mut xml, svg);

                element
                    .attribute("x1", start.x.to_string())
                    .attribute("y1", start.y.to_string())
                    .attribute("x2", end.x.to_string())
                    .attribute("y2", end.y.to_string())
                    .attribute("fill", "none")
                    .attribute("stroke", color.as_hex())
                    .attribute("stroke-width", stroke_width)
                    .push_to(&mut xml, svg);
            }
            Tool::Text {
                anchor_point: mid_point,
                text,
            } => {
                let text_size = (shape.size * FONT_SIZE_FACTOR) as f32 * scale_factor;

                let (bottom_right_x, bottom_right_y) = (
                    mid_point.x + (text_size / 2.0),
                    mid_point.y + (text_size / 2.0),
                );

                element
                    .attribute("x", bottom_right_x.to_string())
                    .attribute("y", bottom_right_y.to_string())
                    .attribute("font-family", FONT_NAME)
                    .attribute("font-size", text_size.to_string())
                    .attribute("fill", color.as_hex())
                    .add_text(text)
                    .push_to(&mut xml, svg);
            }
        };
    }

    let mut options = usvg::Options::default();
    options
        .fontdb_mut()
        .load_font_data(MEDIUM_FONT_TTF.to_vec());

    let tree = usvg::Tree::from_str(
        xml.write_str_with_opts(edit_xml::WriteOptions {
            write_decl: false,
            ..Default::default()
        })
        .expect("XML must be valid")
        .as_str(),
        &options,
    )
    .expect("SVG must be valid");

    resvg::render(&tree, transform, &mut pixmap.as_mut());

    RgbaImage::from_vec(width, height, pixmap.take())
}

fn save_image(image: RgbaImage, directory: &Path) -> Result<PathBuf> {
    let filename = format!("Capture_{}.png", Local::now().format("%Y-%m-%d-%H-%M-%S"));
    let image_path = directory.join(&filename);

    Clipboard::new()
        .expect("Failed to initialize clipboard")
        .set_image(arboard::ImageData {
            width: image.width() as usize,
            height: image.height() as usize,
            bytes: std::borrow::Cow::Borrowed(image.as_raw()),
        })
        .expect("Failed to copy image to clipboard");

    image
        .save_with_format(&image_path, ImageFormat::Png)
        .expect("Failed to save image");

    Ok(image_path)
}
