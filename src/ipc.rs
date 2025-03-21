use iced::{
    futures::{SinkExt, Stream},
    stream,
};
use interprocess::local_socket::{
    GenericNamespaced, ListenerOptions, ToNsName, traits::tokio::Listener,
};

use crate::{Message, consts::APPNAME};

pub fn ipc_listener() -> impl Stream<Item = Message> {
    stream::channel(1, async |mut output| {
        let name = APPNAME
            .to_ns_name::<GenericNamespaced>()
            .expect("Name should be created");

        let listner_opts = ListenerOptions::new().name(name);

        let listener = listner_opts
            .create_tokio()
            .expect("Listener should be created");

        loop {
            if let Ok(_stream) = listener.accept().await {
                let _ = output.send(Message::OpenSettingsWindow).await;
            }
        }
    })
}
