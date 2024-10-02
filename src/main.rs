#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod assets;
mod entities;
mod theme;
mod utils;
mod windows;

use std::collections::BTreeMap;

use assets::{APPICON, APPNAME, FONT_BOLD, FONT_ICONS, FONT_MEDIUM, MEDIUM};
use entities::{
    app::{App, AppEvent},
    capture::{CaptureEvent, CaptureWindow},
    config::{Config, ConfigureWindow},
    theme::Theme,
    window::WindowType,
};
use iced::{
    advanced::graphics::image::image_rs::ImageFormat,
    application::DefaultStyle,
    daemon::{daemon, Appearance},
    keyboard::{key, on_key_press, Modifiers},
    widget::horizontal_space,
    window::{
        self, change_mode, close, close_events, gain_focus, get_scale_factor, icon,
        settings::PlatformSpecific, Id, Mode, Position,
    },
    Size, Subscription, Task,
};
use interprocess::local_socket::{traits::Stream, GenericNamespaced, ToNsName};
use theme::Element;
use utils::{
    ipc::ipc,
    key_listener::global_key_listener,
    shorten_path,
    tray_icon::{tray_icon, tray_icon_listener, tray_menu_listener},
};

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
        .style(App::style)
        .theme(App::theme)
        .subscription(App::subscription)
        .antialiasing(true)
        .run_with(App::new)
}

impl App {
    pub fn new() -> (App, Task<AppEvent>) {
        let (config, is_initial) = Config::new();
        let _tray_icon = tray_icon();
        (
            App {
                _tray_icon,
                config,
                windows: BTreeMap::new(),
            },
            if is_initial {
                Task::done(AppEvent::OpenConfigureWindow)
            } else {
                Task::none()
            },
        )
    }

    pub fn title(&self, id: Id) -> String {
        match self.windows.get(&id) {
            Some(WindowType::Configure(_)) => String::from("Capter"),
            Some(WindowType::Capture(_)) => String::from("Capter: Capture"),
            None => String::new(),
        }
    }

    pub fn update(&mut self, message: AppEvent) -> Task<AppEvent> {
        match message {
            AppEvent::OpenConfigureWindow => {
                if self.windows.is_empty() {
                    let (id, open_task) = window::open(window::Settings {
                        size: Size {
                            width: 700.0,
                            height: 430.0,
                        },
                        position: Position::Centered,
                        resizable: false,
                        icon: Some(icon::from_file_data(APPICON, Some(ImageFormat::Png)).unwrap()),
                        #[cfg(target_os = "macos")]
                        platform_specific: PlatformSpecific {
                            title_hidden: true,
                            titlebar_transparent: true,
                            fullsize_content_view: true,
                        },
                        #[cfg(target_os = "linux")]
                        platform_specific: PlatformSpecific {
                            application_id: String::from("Capter"),
                            override_redirect: true,
                        },
                        ..Default::default()
                    });
                    self.windows.insert(
                        id,
                        WindowType::Configure(ConfigureWindow::new(
                            shorten_path(self.config.directory.clone()),
                            self.config.theme.clone(),
                        )),
                    );
                    return open_task.discard().chain(gain_focus(id));
                }
                Task::none()
            }
            AppEvent::OpenDirectory => {
                self.config.open_directory();
                Task::none()
            }
            AppEvent::UpdateDirectory(id) => {
                self.config.update_directory();
                if let Some(WindowType::Configure(config_window)) = self.windows.get_mut(&id) {
                    config_window.path = shorten_path(self.config.directory.clone());
                }
                Task::none()
            }
            AppEvent::GetScaleFactor(id, scale_factor) => {
                if let Some(WindowType::Capture(capture_window)) = self.windows.get_mut(&id) {
                    capture_window.scale_factor = scale_factor;
                }
                Task::none()
            }
            AppEvent::OpenCaptureWindow => {
                if self.windows.is_empty()
                    || !matches!(
                        self.windows.first_key_value().unwrap().1,
                        WindowType::Capture(_)
                    )
                {
                    let (id, open_task) = window::open(window::Settings {
                        transparent: true,
                        decorations: false,
                        #[cfg(target_os = "windows")]
                        platform_specific: PlatformSpecific {
                            drag_and_drop: false,
                            skip_taskbar: true,
                            undecorated_shadow: false,
                        },
                        ..Default::default()
                    });
                    let capture_window = CaptureWindow::new();
                    self.windows.insert(id, WindowType::Capture(capture_window));
                    return open_task
                        .discard()
                        .chain(gain_focus(id))
                        .chain(change_mode(id, Mode::Fullscreen))
                        .chain(
                            get_scale_factor(id).map(move |scale_factor| {
                                AppEvent::GetScaleFactor(id, scale_factor)
                            }),
                        );
                }
                Task::none()
            }
            AppEvent::Undo => {
                if let Some((id, WindowType::Capture(_))) = self.windows.last_key_value() {
                    return Task::done(AppEvent::Capture(*id, CaptureEvent::Undo));
                }
                Task::none()
            }
            AppEvent::Done => {
                if let Some((id, WindowType::Capture(_))) = self.windows.last_key_value() {
                    return Task::done(AppEvent::Capture(*id, CaptureEvent::Done));
                }
                Task::none()
            }
            AppEvent::Cancel => {
                if let Some((id, WindowType::Capture(_))) = self.windows.last_key_value() {
                    return Task::done(AppEvent::Capture(*id, CaptureEvent::Cancel));
                }
                Task::none()
            }
            AppEvent::RequestClose(id) => close(id),
            AppEvent::WindowClosed(id) => {
                match self.windows.remove(&id) {
                    Some(WindowType::Capture(capture_window)) => {
                        capture_window.take_screenshot(&self.config);
                    }
                    Some(WindowType::Configure(config_window)) => {
                        self.config.theme = config_window.theme.target().clone();
                        self.config.update_config();
                    }
                    None => (),
                };
                Task::none()
            }
            AppEvent::ExitApp => {
                self.config.update_config();
                iced::exit()
            }
            AppEvent::Config(id, message) => {
                if let Some(WindowType::Configure(config_window)) = self.windows.get_mut(&id) {
                    return config_window.update(id, message);
                }
                Task::none()
            }
            AppEvent::Capture(id, message) => {
                if let Some(WindowType::Capture(capture_window)) = self.windows.get_mut(&id) {
                    return capture_window.update(id, message);
                }
                Task::none()
            }
        }
    }

    pub fn view(&self, id: Id) -> Element<AppEvent> {
        let content = match &self.windows.get(&id) {
            Some(WindowType::Configure(config_window)) => config_window
                .view()
                .map(move |message| AppEvent::Config(id, message)),
            Some(WindowType::Capture(capture_window)) => capture_window
                .view()
                .map(move |message| AppEvent::Capture(id, message)),
            None => horizontal_space().into(),
        };

        content
    }

    pub fn theme(&self, id: Id) -> Theme {
        match self.windows.get(&id) {
            Some(WindowType::Configure(config_window)) => config_window.theme.value().clone(),
            _ => self.config.theme.clone(),
        }
    }

    pub fn style(&self, theme: &Theme) -> Appearance {
        theme.default_style()
    }

    pub fn subscription(&self) -> Subscription<AppEvent> {
        let window_events = close_events().map(AppEvent::WindowClosed);

        let app_key_listener = on_key_press(|key, modifiers| match (key, modifiers) {
            (key::Key::Named(key::Named::Escape), _) => Some(AppEvent::Cancel),
            (key::Key::Named(key::Named::Enter), _) => Some(AppEvent::Done),
            (key::Key::Character(char), m)
                if m.contains(Modifiers::SHIFT) && m.contains(Modifiers::ALT) =>
            {
                match char.as_str() {
                    "s" => Some(AppEvent::OpenCaptureWindow),
                    _ => None,
                }
            }
            (key::Key::Character(char), Modifiers::CTRL) => {
                match char.as_str() {
                    "z" => Some(AppEvent::Undo),
                    _ => None
                }
            }
            _ => None,
        });

        let global_key_listener = Subscription::run(global_key_listener);

        let tray_icon_listener = Subscription::run(tray_icon_listener);

        let tray_menu_listener = Subscription::run(tray_menu_listener);

        let ipc = Subscription::run(ipc);

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
