use xcap::image::RgbaImage;
use xcap::Monitor;

use crate::entities::config::Config;

use crate::utils::capture::save_image;

pub fn get_fullscreen() -> Option<RgbaImage> {
    let monitors = Monitor::all().unwrap();
    let mut image = None;
    for monitor in monitors {
        if monitor.is_primary() {
            image = Some(monitor.capture_image().unwrap());
        }
    }
    image
}

pub fn capture_fullscreen(config: &Config) {
    if let Some(image) = get_fullscreen() { save_image(config, image) }
}
