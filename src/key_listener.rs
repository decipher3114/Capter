use iced::{
    futures::{SinkExt, Stream, StreamExt, channel::mpsc},
    stream,
};
use rdev::{EventType, Key, listen};

use crate::Message;

pub fn global_key_listener() -> impl Stream<Item = Message> {
    stream::channel(1, async |mut output| {
        let (mut sender, mut receiver) = mpsc::channel(1);

        std::thread::spawn(move || {
            let _ = listen(move |event| {
                let _ = sender.try_send(event);
            });
        });

        let mut alt_pressed = false;
        let mut shift_pressed = false;

        loop {
            let event = receiver.select_next_some().await;
            match event.event_type {
                EventType::KeyPress(key) => {
                    match key {
                        Key::Alt => alt_pressed = true,
                        Key::ShiftLeft | Key::ShiftRight => shift_pressed = true,
                        Key::KeyS if alt_pressed && shift_pressed => {
                            let _ = output.send(Message::OpenCaptureWindow).await;
                        }
                        Key::KeyO if alt_pressed && shift_pressed => {
                            let _ = output.send(Message::OpenSettingsWindow).await;
                        }
                        _ => {}
                    }
                }
                EventType::KeyRelease(key) => {
                    match key {
                        Key::Alt => alt_pressed = false,
                        Key::ShiftLeft | Key::ShiftRight => shift_pressed = false,
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    })
}
