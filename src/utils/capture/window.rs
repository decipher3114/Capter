use xcap::{image::DynamicImage, Window};

use crate::{entities::config::Config, utils::capture::save_image};

pub fn capture_window(config: &Config) {
    let windows = Window::all().unwrap();

    let active_window = active_win_pos_rs::get_active_window().unwrap();

    for window in windows {
        if window.title() == active_window.title {
            let capture = window.capture_image().unwrap();

            let image = DynamicImage::from(capture);

            save_image(config, image);
        }
    }
}
