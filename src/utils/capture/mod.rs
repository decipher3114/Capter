use arboard::{Clipboard, ImageData};
use display_info::DisplayInfo;
use iced::Point;
use xcap::image::{DynamicImage, ImageFormat, RgbaImage};

use crate::{
    entities::{
        config::Config,
        capture::{CaptureWindow, Area},
    },
    utils::evaluate_points,
};

pub mod fullscreen;
pub mod window;

impl CaptureWindow {
    pub fn new() -> Self {
        let image = fullscreen::get_fullscreen().unwrap();
        CaptureWindow {
            cursor_position: Point::ORIGIN,
            image,
            selection_area: Area {
                initial_pos: None,
                final_pos: None,
            },
        }
    }

    pub fn crop_screenshot(self, config: &Config) {
        if let (Some(initial_pos), Some(final_pos)) = (
            self.selection_area.initial_pos,
            self.selection_area.final_pos,
        ) {
            let scale_factor = DisplayInfo::all()
                .unwrap()
                .into_iter()
                .find(|d| d.is_primary)
                .unwrap()
                .scale_factor;

            let (initial_pos, final_pos) = evaluate_points(initial_pos, final_pos);

            let cropped_image = DynamicImage::from(self.image).crop(
                (initial_pos.x * scale_factor) as u32,
                (initial_pos.y * scale_factor) as u32,
                ((final_pos.x - initial_pos.x) * scale_factor) as u32,
                ((final_pos.y - initial_pos.y) * scale_factor) as u32,
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
