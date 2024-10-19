pub const APPNAME: &str = "Capter";

pub const FONT_MEDIUM: &[u8] = include_bytes!("../assets/fonts/SpaceGrotesk-Medium.ttf");

pub const FONT_BOLD: &[u8] = include_bytes!("../assets/fonts/SpaceGrotesk-Bold.ttf");

pub const FONT_ICONS: &[u8] = include_bytes!("../assets/fonts/icons.ttf");

pub const APPICON: &[u8] = include_bytes!("../assets/icons/icon.png");

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

pub const ICON: Font = Font::with_name("icons");

pub const FOLDER_ICON: char = '\u{E100}';

pub const RECT_FILLED: char = '\u{F101}';

pub const RECT_STROKE: char = '\u{F102}';

pub const ELLIPSE_FILLED: char = '\u{F103}';

pub const ELLIPSE_STROKE: char = '\u{F104}';

pub const LINE: char = '\u{F105}';

pub const HIGHLIGHT: char = '\u{F106}';

// pub const CANCEL: char = '\u{F201}';

// pub const DONE: char = '\u{F202}';

pub const STROKE_THIN: char = '\u{F301}';

pub const STROKE_MEDIUM: char = '\u{F302}';

pub const STROKE_BROAD: char = '\u{F303}';