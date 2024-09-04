pub const APPNAME: &str = "Capter";

pub const FONT_MEDIUM: &[u8; 86616] = include_bytes!("../assets/fonts/SpaceGrotesk-Medium.ttf");

pub const FONT_BOLD: &[u8; 86520] = include_bytes!("../assets/fonts/SpaceGrotesk-Bold.ttf");

pub const ICON: &[u8; 629] = include_bytes!("../assets/icons/icon.png");

pub const SVG_FOLDER_OPEN: &[u8; 653] = include_bytes!("../assets/icons/folder-open.svg");

use iced::Font;

pub const MEDIUM: Font = Font {
    family: iced::font::Family::Name("Space Grotesk"),
    weight: iced::font::Weight::Medium,
    stretch: iced::font::Stretch::Normal,
    style: iced::font::Style::Normal,
};

pub const BOLD: Font = Font {
    family: iced::font::Family::Name("Space Grotesk"),
    weight: iced::font::Weight::Bold,
    stretch: iced::font::Stretch::Normal,
    style: iced::font::Style::Normal,
};
