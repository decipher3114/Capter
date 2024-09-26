use display_info::DisplayInfo;
use iced::Point;
use xcap::image::DynamicImage;

use crate::{
    entities::{
        config::Config,
        crop::{CropWindow, SelectionArea},
    },
    utils::evaluate_points,
};

use crate::utils::capture::{fullscreen, save_image};

impl CropWindow {
    pub fn new() -> Self {
        let image = fullscreen::get_fullscreen().unwrap();
        CropWindow {
            cursor_position: Point::ORIGIN,
            image,
            selection_area: SelectionArea {
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
