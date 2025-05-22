use iced::{
    Point, Size, Task,
    window::{self, settings::PlatformSpecific},
};
use mouse_position::mouse_position::Mouse;

use crate::{
    App, Message,
    capture::{self, Capture},
    consts::APPICON,
    settings::{self, Settings},
    window::AppWindow,
};

impl App {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ConfigInitialized => {
                self.notify("", None);
                return Task::done(Message::OpenCaptureWindow);
            }
            Message::OpenSettingsWindow => {
                if self.windows.is_empty() {
                    let (id, task) = window::open(window::Settings {
                        size: Size {
                            width: 700.0,
                            height: 430.0,
                        },
                        position: window::Position::Centered,
                        resizable: false,
                        level: window::Level::Normal,
                        icon: Some(
                            window::icon::from_file_data(APPICON, None)
                                .expect("AppIcon should be loaded"),
                        ),
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

                    self.windows.insert(id, Settings::init(&self.config).into());

                    return task.discard().chain(window::gain_focus(id));
                }
            }
            Message::OpenCaptureWindow => {
                if self.windows.is_empty()
                    || !matches!(
                        self.windows
                            .first_key_value()
                            .expect("A window must exist")
                            .1,
                        AppWindow::Capture(_)
                    )
                {
                    let (x, y) = match Mouse::get_mouse_position() {
                        Mouse::Position { x, y } => (x, y),
                        Mouse::Error => (0, 0),
                    };

                    match xcap::Monitor::from_point(x, y) {
                        Ok(monitor) => {
                            let (id, open_task) = window::open(window::Settings {
                                position: window::Position::Specific(Point::new(
                                    x as f32, y as f32,
                                )),
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

                            match Capture::new(monitor) {
                                Ok(capture) => {
                                    self.windows.insert(id, capture.into());

                                    return open_task
                                        .discard()
                                        .chain(window::gain_focus(id))
                                        .chain(window::set_mode(id, window::Mode::Fullscreen));
                                }
                                Err(err) => {
                                    let error = err.to_string();
                                    self.notify(&error, None);
                                }
                            };
                        }
                        Err(err) => {
                            let error = err.to_string();
                            self.notify(&error, None);
                        }
                    }
                }
            }
            Message::Undo => {
                if let Some((id, AppWindow::Capture(_))) = self.windows.last_key_value() {
                    return Task::done(Message::Capture(*id, capture::Message::Undo));
                }
            }
            Message::Done => {
                if let Some((id, AppWindow::Capture(_))) = self.windows.last_key_value() {
                    return Task::done(Message::Capture(*id, capture::Message::Done));
                }
            }
            Message::Cancel => {
                if let Some((id, AppWindow::Capture(_))) = self.windows.last_key_value() {
                    return Task::done(Message::Capture(*id, capture::Message::Cancel));
                }
            }
            Message::RequestClose(id) => {
                return window::close(id);
            }
            Message::WindowClosed(id) => {
                match self.windows.remove(&id) {
                    Some(AppWindow::Settings(_)) => {
                        let _ = self.config.save();
                    }
                    Some(AppWindow::Capture(capture)) => {
                        let result = capture.finalize(&self.config);

                        let image_path = result
                            .as_ref()
                            .ok()
                            .and_then(|filename| filename.to_str().map(String::from));

                        let msg = result
                            .map(|_| "Screenshot saved and copied to clipboard".to_string())
                            .unwrap_or_else(|err| err.to_string());

                        self.notify(&msg, image_path);
                    }

                    None => (),
                };
            }
            Message::ExitApp => {
                let _ = self.config.save();
                return iced::exit();
            }
            Message::Settings(id, message) => {
                if let Some(AppWindow::Settings(config_window)) = self.windows.get_mut(&id) {
                    let action = config_window.update(message, &mut self.config);

                    let mut tasks = Vec::with_capacity(2);

                    tasks.push(
                        action
                            .task
                            .map(move |message| Message::Settings(id, message)),
                    );

                    action.requests.into_iter().for_each(|request| {
                        match request {
                            settings::Request::Exit => {
                                tasks.push(Task::done(Message::ExitApp));
                            }
                        }
                    });

                    return Task::batch(tasks);
                }
            }
            Message::Capture(id, message) => {
                if let Some(AppWindow::Capture(capture_window)) = self.windows.get_mut(&id) {
                    let action = capture_window.update(message);

                    let mut tasks = Vec::with_capacity(2);

                    tasks.push(
                        action
                            .task
                            .map(move |message| Message::Capture(id, message)),
                    );

                    action.requests.into_iter().for_each(|request| {
                        match request {
                            capture::Request::Close => {
                                tasks.push(Task::done(Message::RequestClose(id)));
                            }
                        }
                    });

                    return Task::batch(tasks);
                }
            }
        }
        Task::none()
    }
}
