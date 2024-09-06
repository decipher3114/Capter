use display_info::DisplayInfo;
use iced::Point;
use xcap::image::DynamicImage;

use crate::{
    entities::{
        config::Config,
        freeform::{FreeForm, SelectionArea},
    },
    utils::evaluate_points,
};

use crate::utils::capture::{fullscreen, save_image};

impl FreeForm {
    pub fn new() -> Self {
        let image = fullscreen::get_fullscreen().unwrap();
        FreeForm {
            cursor_position: Point::ORIGIN,
            image,
            selection_area: SelectionArea {
                initial_pos: None,
                final_pos: None,
            },
        }
    }

    pub fn capture_freeform(self, config: &Config) {

        if let (Some(initial_pos), Some(final_pos)) = (
            self.selection_area.initial_pos,
            self.selection_area.final_pos,
        ) {
            let mut scale_factor = 1.0;
            let displays = DisplayInfo::all().unwrap();
            for display in displays {
                if display.is_primary {
                    scale_factor = display.scale_factor;
                }
            }

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
