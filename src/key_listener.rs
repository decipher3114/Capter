use iced::{
    futures::{SinkExt, Stream},
    stream,
};
#[cfg(not(target_os = "linux"))]
use rdev::grab;
#[cfg(target_os = "linux")]
use rdev::start_grab_listen;
use rdev::{EventType, Key};
use tokio::sync::mpsc::channel;

use crate::app::AppEvent;

pub fn global_key_listener() -> impl Stream<Item = AppEvent> {
    stream::channel(10, |mut output| async move {
        let (sender, mut receiver) = channel(10);

        std::thread::spawn(move || {
            #[cfg(target_os = "linux")]
            start_grab_listen(move |event| {
                sender.blocking_send(event.clone()).ok();
                Some(event)
            })
            .unwrap();
            #[cfg(not(target_os = "linux"))]
            grab(move |event| {
                sender.blocking_send(event.clone()).ok();
                Some(event)
            })
            .unwrap();
        });

        let mut alt_pressed = false;
        let mut shift_pressed = false;

        loop {
            let event = receiver.recv().await.unwrap();
            match event.event_type {
                EventType::KeyPress(key) => match key {
                    Key::Alt => alt_pressed = true,
                    Key::ShiftLeft | Key::ShiftRight => shift_pressed = true,
                    Key::KeyS if alt_pressed && shift_pressed => {
                        output.send(AppEvent::OpenCaptureWindow).await.unwrap();
                    }
                    Key::KeyO if alt_pressed && shift_pressed => {
                        output.send(AppEvent::OpenConfigureWindow).await.unwrap()
                    }
                    _ => (),
                },
                EventType::KeyRelease(key) => match key {
                    Key::Alt => {
                        alt_pressed = false;
                    }
                    Key::ShiftLeft | Key::ShiftRight => {
                        shift_pressed = false;
                    }
                    _ => (),
                },
                _ => (),
            }
        }
    })
}
