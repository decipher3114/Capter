#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod assets;
mod entities;
mod style;
mod utils;
mod windows;

use std::collections::BTreeMap;

use assets::{FONT_BOLD, FONT_MEDIUM, ICON, MEDIUM};
use entities::{
    app::{App, AppEvent},
    config::Config,
    freeform::FreeForm,
    theme::Theme,
    window::WindowType,
};
use iced::{
    advanced::graphics::image::image_rs::ImageFormat,
    application::DefaultStyle,
    daemon::{daemon, Appearance},
    keyboard::{key, on_key_press},
    widget::horizontal_space,
    window::{
        self, change_mode, close, close_events, gain_focus, icon, Id,
        Mode,
    },
    Size, Subscription, Task,
};
use style::Element;
use utils::{
    capture::{fullscreen::capture_fullscreen, window::capture_window},
    key_listener::global_key_listener, tray_icon::{tray_icon, tray_icon_listener, tray_menu_listener},
};

pub fn main() -> Result<(), iced::Error> {
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

    pub fn title(&self, _id: Id) -> String {
        String::from("Capter")
    }

    pub fn update(&mut self, message: AppEvent) -> Task<AppEvent> {
        match message {
            AppEvent::OpenConfigureWindow => {
                if self.windows.is_empty() {
                    let (id, open_task) = window::open(window::Settings {
                        size: Size { width: 700.0, height: 430.0 },
                        resizable: false,
                        icon: Some(icon::from_file_data(ICON, Some(ImageFormat::Png)).unwrap()),
                        ..Default::default()
                    });
                    self.windows
                        .insert(id, WindowType::ConfigureWindow(self.config.clone()));
                    open_task.discard()
                } else {
                    Task::none()
                }
            }
            AppEvent::UpdateConfig => {
                if let Some((_, WindowType::ConfigureWindow(config))) =
                    self.windows.first_key_value()
                {
                    self.config = config.clone();
                }
                Task::none()
            }
            AppEvent::InitiateFreeForm => {
                if self.windows.len() <= 1 {
                    let (id, open_task) = window::open(window::Settings {
                        transparent: true,
                        decorations: false,
                        ..Default::default()
                    });
                    let freeform = FreeForm::new();
                    self.windows
                        .insert(id, WindowType::FreeFormWindow(freeform));
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
                ().into()
            },
            AppEvent::CaptureWindow => {
                capture_window(&self.config);
                ().into()
            },
            AppEvent::CloseWindow => window::get_latest().and_then::<Id>(close).discard(),
            AppEvent::WindowClosed(id) => {
                match self.windows.get(&id) {
                    Some(WindowType::FreeFormWindow(freeform)) => {
                        freeform.capture_freeform(&self.config)
                    }
                    Some(WindowType::ConfigureWindow(config)) => config.write_config(false),
                    None => (),
                }
                self.windows.remove(&id);
                Task::none()
            }
            AppEvent::ExitApp => {
                self.config.write_config(false);
                iced::exit()
            }
            AppEvent::ConfigAppEvent(id, message) => {
                if let Some(WindowType::ConfigureWindow(config)) = self.windows.get_mut(&id) {
                    config.update(message)
                } else {
                    Task::none()
                }
            }
            AppEvent::FreeFormAppEvent(id, message) => {
                if let Some(WindowType::FreeFormWindow(freeform)) = self.windows.get_mut(&id) {
                    freeform.update(message)
                } else {
                    Task::none()
                }
            }
        }
    }

    pub fn view(&self, id: Id) -> Element<AppEvent> {
        let content = match &self.windows.get(&id) {
            Some(WindowType::ConfigureWindow(config)) => config
                .view()
                .map(move |message| AppEvent::ConfigAppEvent(id, message)),
            Some(WindowType::FreeFormWindow(freeform)) => freeform
                .view()
                .map(move |message| AppEvent::FreeFormAppEvent(id, message)),
            None => horizontal_space().into(),
        };

        content
    }

    pub fn theme(&self, id: Id) -> Theme {
        match &self.windows.get(&id) {
            Some(WindowType::ConfigureWindow(config)) => config.theme(),
            _ => Theme::default(),
        }
    }

    pub fn style(&self, theme: &Theme) -> Appearance {
        theme.default_style()
    }

    pub fn subscription(&self) -> Subscription<AppEvent> {
        let window_events = close_events().map(AppEvent::WindowClosed);

        let app_key_listener = on_key_press(|key, _| match key {
            key::Key::Named(key::Named::Escape | key::Named::Enter) => Some(AppEvent::CloseWindow),
            _ => None,
        });
        let global_key_listener = Subscription::run(global_key_listener);

        let tray_icon_listener = Subscription::run(tray_icon_listener);

        let tray_menu_listener = Subscription::run(tray_menu_listener);

        Subscription::batch([window_events, app_key_listener, global_key_listener, tray_icon_listener, tray_menu_listener])
    }
}
