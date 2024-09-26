use xcap::image::RgbaImage;
use xcap::Monitor;

use crate::entities::config::Config;

use crate::utils::capture::save_image;

pub fn get_fullscreen() -> Option<RgbaImage> {
    let monitors = Monitor::all().unwrap();
    let image = monitors
        .into_iter()
        .find(|m| m.is_primary())
        .unwrap()
        .capture_image()
        .unwrap();

    Some(image)
}

pub fn capture_fullscreen(config: &Config) {
    if let Some(image) = get_fullscreen() {
        save_image(config, image)
    }
}
