#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::BTreeMap;

use config::Config;
use consts::{APPID, APPNAME, BOLD_FONT_TTF, ICON_FONT_TTF, MEDIUM_FONT, MEDIUM_FONT_TTF};
use iced::{Task, daemon, window::Id};
use interprocess::local_socket::{self, GenericNamespaced, ToNsName, traits::Stream};
use tray_icon::create_tray_icon;
use window::AppWindow;

mod action;
mod consts;
mod ipc;
mod key_listener;
mod notify;
mod subscription;
mod theme;
mod tray_icon;
mod update;
mod view;
mod window;

mod capture;
mod config;
mod settings;

fn main() -> Result<(), iced::Error> {
    let name = APPNAME
        .to_ns_name::<GenericNamespaced>()
        .expect("Name must be valid");

    if local_socket::Stream::connect(name).is_ok() {
        return Ok(());
    };

    #[cfg(not(target_os = "linux"))]
    let _tray_icon = create_tray_icon();

    #[cfg(target_os = "linux")]
    std::thread::spawn(|| {
        gtk::init().expect("GTK must be initialized");
        let _tray_icon = create_tray_icon();
        gtk::main();
    });

    daemon(App::title, App::update, App::view)
        .font(MEDIUM_FONT_TTF)
        .font(BOLD_FONT_TTF)
        .font(ICON_FONT_TTF)
        .default_font(MEDIUM_FONT)
        .style(App::style)
        .theme(App::theme)
        .subscription(App::subscription)
        .antialiasing(true)
        .run_with(App::new)
}

pub struct App {
    #[cfg(target_os = "windows")]
    notifier: win32_notif::ToastsNotifier,

    config: Config,
    windows: BTreeMap<Id, AppWindow>,
}

#[derive(Debug, Clone)]
pub enum Message {
    ConfigInitialized,
    OpenSettingsWindow,
    OpenCaptureWindow,
    Undo,
    Done,
    Cancel,
    RequestClose(Id),
    WindowClosed(Id),
    ExitApp,
    Settings(Id, settings::Message),
    Capture(Id, capture::Message),
}

impl App {
    pub fn new() -> (App, Task<Message>) {
        let (config, task) = match Config::load() {
            Ok((config, is_first_creation)) => (
                config,
                if is_first_creation {
                    Task::done(Message::OpenSettingsWindow)
                } else {
                    Task::none()
                },
            ),
            Err(_) => (Config::default(), Task::done(Message::OpenSettingsWindow)),
        };

        (
            App {
                #[cfg(target_os = "windows")]
                notifier: win32_notif::ToastsNotifier::new(APPID)
                    .expect("Notifier must be created"),

                config,
                windows: BTreeMap::new(),
            },
            task,
        )
    }
}
