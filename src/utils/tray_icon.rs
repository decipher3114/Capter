use std::{thread::{sleep, spawn}, time::Duration};

use iced::{advanced::graphics::image::image_rs::load_from_memory, futures::{SinkExt, Stream}, stream};
use tokio::sync::mpsc;
use tray_icon::{menu::{Menu, MenuEvent, MenuId, MenuItem}, Icon, MouseButton::Left, TrayIcon, TrayIconAttributes, TrayIconEvent};

use crate::{assets::{APPNAME, ICON}, entities::app::AppEvent};

pub fn tray_icon() -> TrayIcon {

    // let icon =  load_from_memory(buffer)
    let icon_image= load_from_memory(ICON).unwrap();
    let (width, height) = (icon_image.width(), icon_image.height());

    let icon = Icon::from_rgba(icon_image.into_bytes(), width, height).unwrap();

    let menu = Menu::new();
        menu.append_items(
            &[
                &MenuItem::with_id("open", "Open Capter", true, None),
                &MenuItem::with_id("freeform", "Capture FreeForm", true, None),
                &MenuItem::with_id("fullscreen", "Capture FullScreen", true, None),
                &MenuItem::with_id("exit", "Exit", true, None)
            ]
        ).unwrap();

    TrayIcon::new(
        TrayIconAttributes {
            tooltip: Some(APPNAME.to_string()),
            menu: Some(Box::new(menu)),
            icon: Some(icon),
            temp_dir_path: None,
            icon_is_template: false,
            menu_on_left_click: false,
            title: Some(APPNAME.to_string()),
        }
    ).unwrap()
}

pub fn tray_icon_listener() -> impl Stream<Item = AppEvent> {
    stream::channel(
        16,
        |mut output| async move {
            let (sender, mut reciever) = mpsc::channel(16);

            spawn(
                move || loop {
                    if let Ok(event) = TrayIconEvent::receiver().recv() {
                        sender.blocking_send(event).unwrap()
                    }
                }
            );

            loop {
                if let Some(
                    TrayIconEvent::DoubleClick {
                        id: _,
                        position: _,
                        rect: _,
                        button: Left
                    }
                ) = reciever.recv().await {
                    output.send(AppEvent::OpenConfigureWindow).await.unwrap();
                }
            }

        }
    )
}



pub fn tray_menu_listener() -> impl Stream<Item = AppEvent> {
    stream::channel(
        16,
        |mut output| async move {
            let (sender, mut reciever) = mpsc::channel(16);

            spawn(
                move || loop {
                    if let Ok(event) = MenuEvent::receiver().recv() {
                        sender.blocking_send(event).unwrap()
                    }
                }
            );

            loop {
                if let Some(MenuEvent { id: MenuId(id)} ) = reciever.recv().await {
                    let event = match id.as_str() {
                        "open" => AppEvent::OpenConfigureWindow,
                        "freeform" => {
                            sleep(Duration::from_secs(1));
                            AppEvent::InitiateFreeForm
                        },
                        "fullscreen" => {
                            sleep(Duration::from_secs(1));
                            AppEvent::CaptureFullscreen
                        },
                        "exit" => AppEvent::ExitApp,
                        _ => AppEvent::CloseWindow
                    };
                    output.send(event).await.unwrap()
                }
            }

        }
    )
}