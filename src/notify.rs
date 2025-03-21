use crate::App;

impl App {
    #[cfg(target_os = "windows")]
    pub fn notify(&self, body: &str, image_path: Option<String>) {
        use win32_notif::{
            NotificationActivatedEventHandler, NotificationBuilder,
            notification::visual::{Image, Placement, Text},
        };

        if !self.config.notifications {
            return;
        };

        use crate::consts::{APPID, APPNAME};

        let mut notification_builder = NotificationBuilder::new().visual(Text::create(1, body));

        if let Some(image_path) = image_path {
            notification_builder = notification_builder.visual(Image::new(
                1,
                image_path.clone(),
                None,
                true,
                Placement::Hero,
                true,
            ));

            notification_builder = notification_builder.on_activated(
                NotificationActivatedEventHandler::new(move |_, _| {
                    let _ = opener::open(image_path.clone());
                    Ok(())
                }),
            );
        };

        let _ = notification_builder
            .build(1, &self.notifier, APPNAME, APPID)
            .expect("Notification must be built")
            .show();
    }

    #[cfg(not(target_os = "windows"))]
    pub fn notify(&self, body: &str, image_path: Option<String>) {
        use notify_rust::Notification;

        use crate::consts::{APPID, APPNAME};

        if !self.config.notifications {
            return;
        };

        let mut notification = Notification::new();

        notification.app_id(APPID);

        notification.appname(APPNAME);

        notification.summary(body);

        notification.auto_icon();

        if let Some(image_path) = image_path {
            notification.image_path(&image_path);
        };

        let _ = notification.show();
    }
}
