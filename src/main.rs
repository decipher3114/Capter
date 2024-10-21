#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use app::App;
use consts::{APPNAME, FONT_BOLD, FONT_ICONS, FONT_MEDIUM, MEDIUM};
use iced::daemon;
use interprocess::local_socket::{traits::Stream, GenericNamespaced, ToNsName};

mod app;
mod config;
mod consts;
mod ipc;
mod key_listener;
mod theme;
mod tray_icon;
mod windows;

pub fn main() -> Result<(), iced::Error> {
    let name = APPNAME.to_ns_name::<GenericNamespaced>().unwrap();

    if interprocess::local_socket::Stream::connect(name).is_ok() {
        return Ok(());
    };

    daemon(App::title, App::update, App::view)
        .font(FONT_MEDIUM)
        .font(FONT_BOLD)
        .font(FONT_ICONS)
        .default_font(MEDIUM)
        .scale_factor(App::scale_factor)
        .style(App::style)
        .theme(App::theme)
        .subscription(App::subscription)
        .antialiasing(true)
        .run_with(App::new)
}
