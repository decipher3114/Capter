pub const APPNAME: &str = "Capter";

pub const APPID: &str = "app.decipher.capter";

pub const FONT_NAME: &str = "Space Grotesk";

pub const MEDIUM_FONT_TTF: &[u8] = include_bytes!("../assets/fonts/SpaceGrotesk-Medium.ttf");

pub const BOLD_FONT_TTF: &[u8] = include_bytes!("../assets/fonts/SpaceGrotesk-Bold.ttf");

pub const ICON_FONT_TTF: &[u8] = include_bytes!("../assets/fonts/capter.ttf");

pub const APPICON: &[u8] = include_bytes!("../assets/resources/icon.png");

use iced::Font;

pub const MEDIUM_FONT: Font = Font {
    family: iced::font::Family::Name(FONT_NAME),
    weight: iced::font::Weight::Medium,
    stretch: iced::font::Stretch::Normal,
    style: iced::font::Style::Normal,
};

pub const BOLD_FONT: Font = Font {
    family: iced::font::Family::Name(FONT_NAME),
    weight: iced::font::Weight::Bold,
    stretch: iced::font::Stretch::Normal,
    style: iced::font::Style::Normal,
};

pub const ICON_FONT: Font = Font::with_name("capter");

pub const FOLDER_ICON_ICON: char = '\u{E101}';

pub const FILLED_RECTANGLE_ICON: char = '\u{F101}';

pub const HOLLOW_RECTANGLE_ICON: char = '\u{F102}';

pub const FILLED_ELLIPSE_ICON: char = '\u{F103}';

pub const HOLLOW_ELLIPSE_ICON: char = '\u{F104}';

pub const LINE_ICON: char = '\u{F105}';

pub const ARROW_ICON: char = '\u{F106}';

pub const FREE_HAND_ICON: char = '\u{F107}';

pub const HIGHLIGHTER_ICON: char = '\u{F108}';

pub const TEXT_ICON: char = '\u{F109}';

pub const MOVE_ICON: char = '\u{F201}';
