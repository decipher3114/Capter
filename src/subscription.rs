use iced::{
    Subscription,
    keyboard::{
        self,
        Modifiers,
        key,
    },
    window,
};

use crate::{
    App,
    Message,
    ipc::ipc_listener,
    key_listener::global_key_listener,
    tray_icon::{
        tray_icon_listener,
        tray_menu_listener,
    },
};

impl App {
    pub fn subscription(&self) -> Subscription<Message> {
        let window_events = window::close_events().map(Message::WindowClosed);

        let app_key_listener = keyboard::on_key_press(|key, modifiers| match key {
            key::Key::Named(named) => match named {
                key::Named::Escape => Some(Message::Cancel),
                key::Named::Enter => Some(Message::Done),
                _ => None,
            },
            key::Key::Character(char) => match char.as_str() {
                "s" if modifiers.contains(Modifiers::SHIFT)
                    && modifiers.contains(Modifiers::ALT) =>
                {
                    Some(Message::OpenCaptureWindow)
                }
                "z" if modifiers == Modifiers::CTRL => Some(Message::Undo),
                _ => None,
            },
            _ => None,
        });

        let global_key_listener = Subscription::run(global_key_listener);

        let tray_icon_listener = Subscription::run(tray_icon_listener);

        let tray_menu_listener = Subscription::run(tray_menu_listener);

        let ipc = Subscription::run(ipc_listener);

        Subscription::batch([
            window_events,
            app_key_listener,
            global_key_listener,
            tray_icon_listener,
            tray_menu_listener,
            ipc,
        ])
    }
}
