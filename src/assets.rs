pub const FONT_MEDIUM: &[u8; 86616] = include_bytes!("../assets/SpaceGrotesk-Medium.ttf");

pub const FONT_BOLD: &[u8; 86520] = include_bytes!("../assets/SpaceGrotesk-Bold.ttf");

pub const ICON: &[u8; 8647] = include_bytes!("../assets/icon.png");

pub const SVG_FOLDER_OPEN: &[u8; 653] = include_bytes!("../assets/folder-open.svg");

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
