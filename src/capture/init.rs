use std::rc::Rc;

use anyhow::{
    Context,
    Result,
};
use iced::widget::image::Handle;
use xcap::Monitor;

use crate::capture::{
    Capture,
    CapturedWindow,
};

impl Capture {
    pub fn new(monitor: Monitor) -> Result<Self> {
        let scale_factor = monitor
            .scale_factor()
            .with_context(|| "Unable to get scale factor")?;

        let windows = xcap::Window::all()
            .map(|windows| {
                windows
                    .into_iter()
                    .filter_map(|window| {
                        if window.current_monitor().ok()?.id().ok()? == monitor.id().ok()?
                            && !window.is_minimized().ok()?
                            && window.width().ok()? != 0
                            && window.height().ok()? != 0
                            && !window.title().ok()?.is_empty()
                            && !window.app_name().ok()?.is_empty()
                        {
                            Some(Rc::new(CapturedWindow {
                                name: window.title().ok()?.to_string(),
                                x: window.x().ok()? as f32,
                                y: window.y().ok()? as f32,
                                width: window.width().ok()? as f32,
                                height: window.height().ok()? as f32,
                                screenshot: window.capture_image().ok()?,
                            }))
                        } else {
                            None
                        }
                    })
                    .collect()
            })
            .unwrap_or_default();

        let screenshot = monitor
            .capture_image()
            .with_context(|| "Unable to capture Monitor")?;

        Ok(Capture {
            scale_factor,
            screenshot: screenshot.clone(),
            screenshot_handle: Handle::from_rgba(
                screenshot.width(),
                screenshot.height(),
                screenshot.into_raw(),
            ),
            windows,
            toolbar_at_top: true,
            cursor_position: Default::default(),
            mode: Default::default(),
            elements: Default::default(),
            cache: Default::default(),
        })
    }
}
