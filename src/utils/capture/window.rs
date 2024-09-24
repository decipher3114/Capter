use xcap::Window;

use crate::{entities::config::Config, utils::capture::save_image};

pub fn capture_window(config: &Config) {
    let windows = Window::all().unwrap();

    let active_window_title = active_win_pos_rs::get_active_window().unwrap().title;

    let window = windows
        .into_iter()
        .find(|x| x.title() == active_window_title)
        .unwrap();

    let image = window.capture_image().unwrap();

    save_image(config, image);
}
