use iced::{futures::{SinkExt, Stream}, stream};
use rdev::{listen, EventType, Key};
use tokio::sync::mpsc::channel;

use crate::entities::app::AppEvent;

pub fn global_key_listener() -> impl Stream<Item = AppEvent> {
    stream::channel(10, |mut output| async move {
        let (sender, mut receiver) = channel(10);

        std::thread::spawn(move || {
            listen(move |event| {
                sender.blocking_send(event.clone()).unwrap();
            })
            .unwrap()
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
                        output.send(AppEvent::InitiateFreeForm).await.unwrap();
                    }
                    Key::KeyF if alt_pressed && shift_pressed => {
                        output.send(AppEvent::CaptureFullscreen).await.unwrap();
                    }
                    Key::KeyW if alt_pressed && shift_pressed => {
                        output.send(AppEvent::CaptureWindow).await.unwrap();
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
