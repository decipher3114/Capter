use std::time::Duration;

use iced::{
    advanced::graphics::image::image_rs::load_from_memory,
    futures::{SinkExt, Stream},
    stream,
};
use tokio::{sync::mpsc, time::sleep};
use tray_icon::{
    Icon,
    MouseButton::Left,
    TrayIcon, TrayIconBuilder, TrayIconEvent,
    menu::{
        Menu, MenuEvent, MenuId, MenuItem, PredefinedMenuItem,
        accelerator::{Accelerator, Code, Modifiers},
    },
};

use crate::{
    app::AppEvent,
    consts::{APPICON, APPNAME},
};

pub fn create_tray_icon() -> TrayIcon {
    let icon_image = load_from_memory(APPICON).unwrap();
    let (width, height) = (icon_image.width(), icon_image.height());

    let icon = Icon::from_rgba(icon_image.into_bytes(), width, height).unwrap();

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
    .unwrap();

    TrayIconBuilder::new()
        .with_icon(icon)
        .with_menu(Box::new(menu))
        .with_tooltip(format!("{} {}", APPNAME, env!("CARGO_PKG_VERSION")))
        .build()
        .unwrap()
}

pub fn tray_icon_listener() -> impl Stream<Item = AppEvent> {
    stream::channel(1, |mut output| async move {
        let (sender, mut reciever) = mpsc::channel(1);

        std::thread::spawn(move || {
            loop {
                if let Ok(event) = TrayIconEvent::receiver().recv() {
                    sender.blocking_send(event).unwrap()
                }
            }
        });

        loop {
            if let Some(TrayIconEvent::DoubleClick { button: Left, .. }) = reciever.recv().await {
                output.send(AppEvent::OpenCaptureWindow).await.unwrap()
            }
        }
    })
}

pub fn tray_menu_listener() -> impl Stream<Item = AppEvent> {
    stream::channel(1, |mut output| async move {
        let (sender, mut reciever) = mpsc::channel(1);

        std::thread::spawn(move || {
            loop {
                if let Ok(event) = MenuEvent::receiver().recv() {
                    sender.blocking_send(event).unwrap()
                }
            }
        });

        loop {
            if let Some(MenuEvent { id: MenuId(id) }) = reciever.recv().await {
                match id.as_str() {
                    "open" => output.send(AppEvent::OpenConfigureWindow).await.unwrap(),
                    "capture" => {
                        sleep(Duration::from_secs(1)).await;
                        output.send(AppEvent::OpenCaptureWindow).await.unwrap()
                    }
                    "exit" => output.send(AppEvent::ExitApp).await.unwrap(),
                    _ => (),
                }
            }
        }
    })
}
