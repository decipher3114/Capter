use std::{collections::BTreeMap, process::Command};

use iced::{advanced::graphics::image::image_rs::ImageFormat, daemon::{Appearance, DefaultStyle}, keyboard::{key, on_key_press, Modifiers}, widget::horizontal_space, window::{self, change_mode, close, close_events, gain_focus, get_scale_factor, icon, settings::PlatformSpecific, Id, Mode, Position}, Point, Size, Subscription, Task};
use rfd::FileDialog;
use tray_icon::TrayIcon;
use xcap::Monitor;

use crate::{consts::APPICON, config::{utils::shorten_path, Config}, ipc::ipc_listener, key_listener::global_key_listener, theme::{Element, Theme}, tray_icon::{create_tray_icon, tray_icon_listener, tray_menu_listener}, windows::{capture_window::{CaptureEvent, CaptureWindow}, config_window::{ConfigEvent, ConfigureWindow}, AppWindow}};


pub struct App {
    #[expect(dead_code)]
    pub tray_icon: TrayIcon,
    pub config: Config,
    pub monitors: Vec<Monitor>,
    pub windows: BTreeMap<Id, AppWindow>,
}

#[derive(Debug, Clone)]
pub enum AppEvent {
    OpenConfigureWindow,
    OpenDirectory,
    UpdateDirectory(Id),
    GetScaleFactor(Id, f32),
    OpenCaptureWindow,
    Undo,
    Done,
    Cancel,
    RequestClose(Id),
    WindowClosed(Id),
    ExitApp,
    Config(Id, ConfigEvent),
    Capture(Id, CaptureEvent),
}

impl App {
    pub fn new(monitors: Vec<Monitor>) -> (App, Task<AppEvent>) {
        let (config, is_initial) = Config::new();
        let tray_icon = create_tray_icon();
        (
            App {
                tray_icon,
                config,
                monitors,
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
            Some(AppWindow::Configure(_)) => String::from("Capter"),
            Some(AppWindow::Capture(_)) => String::from("Capter: Capture"),
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
                        AppWindow::Configure(Box::new(ConfigureWindow::new(
                            shorten_path(self.config.directory.clone()),
                            self.config.theme.clone(),
                        ))),
                    );
                    return open_task.discard().chain(gain_focus(id));
                }
                Task::none()
            }
            AppEvent::OpenDirectory => {
                #[cfg(target_os = "windows")]
                let cmd = "explorer";
                #[cfg(target_os = "linux")]
                let cmd = "xdg-open";
                #[cfg(target_os = "macos")]
                let cmd = "open";
                Command::new(cmd).arg(&self.config.directory).spawn().unwrap();
                Task::none()
            }
            AppEvent::UpdateDirectory(id) => {
                if let Some(path) = FileDialog::new()
                    .set_directory(self.config.directory.clone())
                    .pick_folder() {
                    self.config.directory = path.into_os_string().into_string().unwrap();
                    if let Some(AppWindow::Configure(config_window)) = self.windows.get_mut(&id) {
                        config_window.path = shorten_path(self.config.directory.clone());
                    }
                }
                Task::none()
            }
            AppEvent::GetScaleFactor(id, scale_factor) => {
                if let Some(AppWindow::Capture(capture_window)) = self.windows.get_mut(&id) {
                    capture_window.scale_factor = scale_factor;
                }
                Task::none()
            }
            AppEvent::OpenCaptureWindow => {
                if self.windows.is_empty()
                    || !matches!(
                        self.windows.first_key_value().unwrap().1,
                        AppWindow::Capture(_)
                    )
                {
                    let current_monitor = &self.monitors[0];
                    let (id, open_task) = window::open(window::Settings {
                        position: Position::Specific(Point::new(current_monitor.x() as f32, current_monitor.y() as f32)),
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
                    let capture_window = CaptureWindow::new(current_monitor.clone());
                    self.windows
                        .insert(id, AppWindow::Capture(Box::new(capture_window)));
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
                if let Some((id, AppWindow::Capture(_))) = self.windows.last_key_value() {
                    return Task::done(AppEvent::Capture(*id, CaptureEvent::Undo));
                }
                Task::none()
            }
            AppEvent::Done => {
                if let Some((id, AppWindow::Capture(_))) = self.windows.last_key_value() {
                    return Task::done(AppEvent::Capture(*id, CaptureEvent::Done));
                }
                Task::none()
            }
            AppEvent::Cancel => {
                if let Some((id, AppWindow::Capture(_))) = self.windows.last_key_value() {
                    return Task::done(AppEvent::Capture(*id, CaptureEvent::Cancel));
                }
                Task::none()
            }
            AppEvent::RequestClose(id) => close(id),
            AppEvent::WindowClosed(id) => {
                match self.windows.remove(&id) {
                    Some(AppWindow::Capture(capture_window)) => {
                        capture_window.take_screenshot(self.config.directory.clone());
                    }
                    Some(AppWindow::Configure(config_window)) => {
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
                if let Some(AppWindow::Configure(config_window)) = self.windows.get_mut(&id) {
                    return config_window.update(id, message);
                }
                Task::none()
            }
            AppEvent::Capture(id, message) => {
                if let Some(AppWindow::Capture(capture_window)) = self.windows.get_mut(&id) {
                    return capture_window.update(id, message);
                }
                Task::none()
            }
        }
    }

    pub fn view(&self, id: Id) -> Element<AppEvent> {
        let content = match &self.windows.get(&id) {
            Some(AppWindow::Configure(config_window)) => config_window
                .view()
                .map(move |message| AppEvent::Config(id, message)),
            Some(AppWindow::Capture(capture_window)) => capture_window
                .view()
                .map(move |message| AppEvent::Capture(id, message)),
            None => horizontal_space().into(),
        };

        content
    }

    pub fn theme(&self, id: Id) -> Theme {
        match self.windows.get(&id) {
            Some(AppWindow::Configure(config_window)) => config_window.theme.value().clone(),
            _ => self.config.theme.clone(),
        }
    }

    pub fn scale_factor(&self, id: Id) -> f64 {
        match self.windows.get(&id) {
            Some(AppWindow::Capture(capture_window)) => (1.0 / capture_window.scale_factor) as f64,
            _ => 1.0
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
            (key::Key::Character(char), Modifiers::CTRL) => match char.as_str() {
                "z" => Some(AppEvent::Undo),
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
