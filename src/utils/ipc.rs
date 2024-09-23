use iced::{futures::{SinkExt, Stream}, stream};
use interprocess::local_socket::{traits::tokio::Listener, GenericNamespaced, ListenerOptions, ToNsName};

use crate::{assets::APPNAME, entities::app::AppEvent};

pub fn ipc() -> impl Stream<Item = AppEvent> {
    stream::channel(
        10,
        |mut output| async move {

            let name = APPNAME.to_ns_name::<GenericNamespaced>().unwrap();
            
            let listner_opts = ListenerOptions::new().name(name);

            let listener = listner_opts.create_tokio().unwrap();


            loop {
                if let Ok(_stream) = listener.accept().await {
                    output.send(AppEvent::OpenConfigureWindow).await.unwrap();
                }
            }
            
        }
    )
}