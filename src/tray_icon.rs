use std::time::Duration;

use iced::{
    futures::{SinkExt, Stream, StreamExt, channel::mpsc},
    stream,
};
use tokio::time::sleep;
use tray_icon::{
    Icon,
    MouseButton::Left,
    TrayIcon, TrayIconBuilder, TrayIconEvent,
    menu::{
        Menu, MenuEvent, MenuItem, PredefinedMenuItem,
        accelerator::{Accelerator, Code, Modifiers},
    },
};
use xcap::image::load_from_memory;

use crate::{
    Message,
    consts::{APPICON, APPNAME},
};

pub fn create_tray_icon() -> TrayIcon {
    let icon_image = load_from_memory(APPICON).expect("Icon should be loaded");
    let (width, height) = (icon_image.width(), icon_image.height());

    let icon =
        Icon::from_rgba(icon_image.into_bytes(), width, height).expect("Icon should be created");

    let menu = Menu::with_items(&[
        &MenuItem::with_id(
            "open",
            "Open",
            true,
            Some(Accelerator::new(
                Some(Modifiers::SHIFT.union(Modifiers::ALT)),
                Code::KeyO,
            )),
        ),
        &PredefinedMenuItem::separator(),
        &MenuItem::with_id(
            "capture",
            "Capture",
            true,
            Some(Accelerator::new(
                Some(Modifiers::SHIFT.union(Modifiers::ALT)),
                Code::KeyS,
            )),
        ),
        &PredefinedMenuItem::separator(),
        &MenuItem::with_id("exit", "Exit", true, None),
    ])
    .expect("Menu should be created");

    TrayIconBuilder::new()
        .with_icon(icon)
        .with_menu(Box::new(menu))
        .with_tooltip(format!("{} {}", APPNAME, env!("CARGO_PKG_VERSION")))
        .build()
        .expect("Tray icon should be created")
}

pub fn tray_icon_listener() -> impl Stream<Item = Message> {
    stream::channel(1, async |mut output| {
        let (mut sender, mut reciever) = mpsc::channel(1);

        std::thread::spawn(move || {
            loop {
                if let Ok(event) = TrayIconEvent::receiver().recv() {
                    let _ = sender.try_send(event);
                }
            }
        });

        loop {
            if let TrayIconEvent::DoubleClick { button: Left, .. } =
                reciever.select_next_some().await
            {
                let _ = output.send(Message::OpenCaptureWindow).await;
            }
        }
    })
}

pub fn tray_menu_listener() -> impl Stream<Item = Message> {
    stream::channel(1, async |mut output| {
        let (mut sender, mut reciever) = mpsc::channel(1);

        std::thread::spawn(move || {
            loop {
                if let Ok(event) = MenuEvent::receiver().recv() {
                    let _ = sender.try_send(event);
                }
            }
        });

        loop {
            match reciever.select_next_some().await.id().0.as_str() {
                "open" => {
                    let _ = output.send(Message::OpenSettingsWindow).await;
                }
                "capture" => {
                    sleep(Duration::from_secs(1)).await;
                    let _ = output.send(Message::OpenCaptureWindow).await;
                }
                "exit" => {
                    let _ = output.send(Message::ExitApp).await;
                }
                _ => {}
            }
        }
    })
}
