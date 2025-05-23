#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

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

mod organize_type;

use std::collections::BTreeMap;

use ::tray_icon::TrayIcon;
use config::Config;
use consts::{APPNAME, BOLD_FONT_TTF, ICON_FONT_TTF, MEDIUM_FONT, MEDIUM_FONT_TTF};
use iced::{Task, daemon, window::Id};
use interprocess::local_socket::{self, GenericNamespaced, ToNsName, traits::Stream};
use window::AppWindow;

fn main() -> Result<(), iced::Error> {
    let name = APPNAME
        .to_ns_name::<GenericNamespaced>()
        .expect("Name must be valid");

    if local_socket::Stream::connect(name).is_ok() {
        return Ok(());
    };

    #[cfg(target_os = "linux")]
    std::thread::spawn(|| {
        gtk::init().expect("GTK must be initialized");
        let _tray_icon = tray_icon::create_tray_icon();
        gtk::main();
    });

    daemon(App::new, App::update, App::view)
        .font(MEDIUM_FONT_TTF)
        .font(BOLD_FONT_TTF)
        .font(ICON_FONT_TTF)
        .default_font(MEDIUM_FONT)
        .title(App::title)
        .style(App::style)
        .theme(App::theme)
        .subscription(App::subscription)
        .antialiasing(true)
        .run()
}

pub struct App {
    #[cfg(target_os = "windows")]
    notifier: win32_notif::ToastsNotifier,

    config: Config,
    windows: BTreeMap<Id, AppWindow>,
    tray_icon: Option<TrayIcon>,
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
    CreateTrayIcon,
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
        let task = Task::batch([
            task,
            #[cfg(not(target_os = "linux"))]
            Task::done(Message::CreateTrayIcon),
        ]);

        (
            App {
                #[cfg(target_os = "windows")]
                notifier: win32_notif::ToastsNotifier::new(consts::APPID)
                    .expect("Notifier must be created"),

                config,
                windows: BTreeMap::new(),
                tray_icon: None,
            },
            task,
        )
    }
}
