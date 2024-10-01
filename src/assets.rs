pub const APPNAME: &str = "Capter";

pub const FONT_MEDIUM: &[u8; 86616] = include_bytes!("../assets/fonts/SpaceGrotesk-Medium.ttf");

pub const FONT_BOLD: &[u8; 86520] = include_bytes!("../assets/fonts/SpaceGrotesk-Bold.ttf");

pub const FONT_ICONS: &[u8; 2168] = include_bytes!("../assets/fonts/icons.ttf");

pub const APPICON: &[u8; 1358] = include_bytes!("../assets/icons/icon.png");

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

pub const ICON: Font = Font::with_name("icomoon");

pub const FOLDER_ICON: char = '\u{E901}';

pub const CANCEL: char = '\u{E902}';

pub const DONE: char = '\u{E903}';

pub const RECT_FILLED: char = '\u{E904}';

pub const RECT_STROKE: char = '\u{E905}';

pub const ELLIPSE_FILLED: char = '\u{E906}';

pub const ELLIPSE_STROKE: char = '\u{E907}';

pub const LINE: char = '\u{E908}';

pub const STROKE_THIN: char = '\u{E909}';

pub const STROKE_MEDIUM: char = '\u{E910}';

pub const STROKE_BROAD: char = '\u{E911}';
