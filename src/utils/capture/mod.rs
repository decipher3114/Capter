use arboard::{Clipboard, ImageData};
use xcap::image::{DynamicImage, ImageFormat};

use crate::entities::config::Config;

pub mod freeform;
pub mod fullscreen;
pub mod window;

fn save_image(config: &Config, image: DynamicImage) {
    let date = chrono::Local::now().format("%Y-%m-%d-%H-%M-%S");

    let image_path = format!("{}\\Capture_{}.png", config.dir, date);

    Clipboard::new()
        .unwrap()
        .set_image(ImageData {
            width: image.width() as usize,
            height: image.height() as usize,
            bytes: image.clone().into_bytes().into(),
        })
        .unwrap();

    image
        .save_with_format(image_path, ImageFormat::Png)
        .unwrap();
}
