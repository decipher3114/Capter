#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod assets;
mod entities;
mod theme;
mod utils;
mod windows;

use std::collections::BTreeMap;

use assets::{APPNAME, FONT_BOLD, FONT_MEDIUM, ICON, MEDIUM};
use entities::{
    app::{App, AppEvent},
    config::{Config, ConfigureWindow},
    crop::CropWindow,
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
        self, change_mode, close, close_events, gain_focus, icon, settings::PlatformSpecific, Id,
        Mode,
    },
    Size, Subscription, Task,
};
use interprocess::local_socket::{traits::Stream, GenericNamespaced, ToNsName};
use theme::Element;
use utils::{
    capture::{fullscreen::capture_fullscreen, window::capture_window},
    ipc::ipc,
    key_listener::global_key_listener,
    shorten_path,
    tray_icon::{tray_icon, tray_icon_listener, tray_menu_listener},
};

pub fn main() -> Result<(), iced::Error> {
    let name = APPNAME.to_ns_name::<GenericNamespaced>().unwrap();

    if let Ok(_) = interprocess::local_socket::Stream::connect(name) {
        return Ok(());
    };
    daemon(App::title, App::update, App::view)
        .font(FONT_MEDIUM)
        .font(FONT_BOLD)
        .default_font(MEDIUM)
        .style(App::style)
        .theme(App::theme)
        .subscription(App::subscription)
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
            Some(WindowType::ConfigureWindow(_)) => String::from("Capter"),
            Some(WindowType::CropWindow(_)) => String::from("Capter: Crop"),
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
                        resizable: false,
                        icon: Some(icon::from_file_data(ICON, Some(ImageFormat::Png)).unwrap()),
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
                        WindowType::ConfigureWindow(ConfigureWindow::new(
                            shorten_path(self.config.directory.clone()),
                            self.config.theme.clone(),
                        )),
                    );
                    open_task.discard().chain(gain_focus(id))
                } else {
                    Task::none()
                }
            }
            AppEvent::OpenDirectory => self.config.open_directory().into(),
            AppEvent::UpdateDirectory(id) => {
                self.config.update_directory();
                if let Some(WindowType::ConfigureWindow(config_window)) = self.windows.get_mut(&id)
                {
                    config_window.path = shorten_path(self.config.directory.clone());
                }
                Task::none()
            }
            AppEvent::OpenCropWindow => {
                if self.windows.len() <= 1 {
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
                    let crop_window = CropWindow::new();
                    self.windows.insert(id, WindowType::CropWindow(crop_window));
                    open_task
                        .discard()
                        .chain(gain_focus(id))
                        .chain(change_mode(id, Mode::Fullscreen))
                } else {
                    Task::none()
                }
            }
            AppEvent::CaptureFullscreen => {
                capture_fullscreen(&self.config);
                Task::none()
            }
            AppEvent::CaptureWindow => {
                capture_window(&self.config);
                Task::none()
            }
            AppEvent::CloseWindow => window::get_latest().and_then::<Id>(close).discard(),
            AppEvent::WindowClosed(id) => {
                match self.windows.remove(&id) {
                    Some(WindowType::CropWindow(crop_window)) => {
                        crop_window.crop_screenshot(&self.config);
                    }
                    Some(WindowType::ConfigureWindow(config_window)) => {
                        self.config.theme = config_window.theme.target().clone();
                        self.config.update_config()
                    }
                    None => (),
                }
                Task::none()
            }
            AppEvent::ExitApp => {
                self.config.update_config();
                iced::exit()
            }
            AppEvent::Config(id, message) => {
                if let Some(WindowType::ConfigureWindow(config_window)) = self.windows.get_mut(&id)
                {
                    config_window.update(id, message)
                } else {
                    Task::none()
                }
            }
            AppEvent::Crop(id, message) => {
                if let Some(WindowType::CropWindow(crop_window)) = self.windows.get_mut(&id) {
                    crop_window.update(message)
                } else {
                    Task::none()
                }
            }
        }
    }

    pub fn view(&self, id: Id) -> Element<AppEvent> {
        let content = match &self.windows.get(&id) {
            Some(WindowType::ConfigureWindow(config_window)) => config_window
                .view()
                .map(move |message| AppEvent::Config(id, message)),
            Some(WindowType::CropWindow(crop_window)) => crop_window
                .view()
                .map(move |message| AppEvent::Crop(id, message)),
            None => horizontal_space().into(),
        };

        content
    }

    pub fn theme(&self, id: Id) -> Theme {
        match self.windows.get(&id) {
            Some(WindowType::ConfigureWindow(config_window)) => config_window.theme.value().clone(),
            _ => self.config.theme.clone(),
        }
    }

    pub fn style(&self, theme: &Theme) -> Appearance {
        theme.default_style()
    }

    pub fn subscription(&self) -> Subscription<AppEvent> {
        let window_events = close_events().map(AppEvent::WindowClosed);

        let app_key_listener = on_key_press(|key, modifiers| match (key, modifiers) {
            (key::Key::Named(key::Named::Escape | key::Named::Enter), _) => {
                Some(AppEvent::CloseWindow)
            }
            (key::Key::Character(char), m)
                if m.contains(Modifiers::SHIFT) && m.contains(Modifiers::ALT) =>
            {
                match char.as_str() {
                    "s" => Some(AppEvent::OpenCropWindow),
                    "f" => Some(AppEvent::CaptureFullscreen),
                    _ => None,
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
