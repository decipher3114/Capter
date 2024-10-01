use iced::Color;

pub struct ContainerClass;

pub enum ButtonClass {
    Default,
    Danger,
    Selected,
}

pub enum TextClass {
    Default,
    Custom(Color),
}
