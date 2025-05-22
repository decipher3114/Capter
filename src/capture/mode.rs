use std::rc::Rc;

use iced::{Point, Size};

use super::{CapturedWindow, DrawElement, crop::CropState, draw::DrawState};

#[derive(Debug)]
pub enum Mode {
    Draw {
        element: DrawElement,
        state: DrawState,
    },
    Crop {
        top_left: Point,
        bottom_right: Point,
        size: Size,
        state: CropState,
    },
}

impl Default for Mode {
    fn default() -> Self {
        Self::Crop {
            top_left: Point::default(),
            bottom_right: Point::default(),
            size: Size::default(),
            state: CropState::default(),
        }
    }
}

impl Mode {
    pub fn is_draw_mode(&self) -> bool {
        matches!(self, Self::Draw { .. })
    }

    pub fn get_window_below_cursor(
        &mut self,
        windows: &[Rc<CapturedWindow>],
        cursor_position: &Point,
        scale_factor: f32,
        (x, y): (u32, u32),
    ) {
        if let Mode::Crop {
            top_left,
            bottom_right,
            size,
            state: status,
        } = self
        {
            let _ = windows
                .iter()
                .find_map(|window| {
                    let window_top_left = Point::new(
                        window.x.max(0.0) / scale_factor,
                        window.y.max(0.0) / scale_factor,
                    );

                    let window_bottom_right = Point::new(
                        (window.x + window.width) / scale_factor,
                        (window.y + window.height) / scale_factor,
                    );

                    if (window_top_left.x..=window_bottom_right.x).contains(&cursor_position.x)
                        && (window_top_left.y..=window_bottom_right.y).contains(&cursor_position.y)
                    {
                        *top_left = window_top_left;
                        *bottom_right = window_bottom_right;
                        *size = (*bottom_right - *top_left).into();
                        *status = CropState::Window(window.clone());
                        Some(())
                    } else {
                        None
                    }
                })
                .or_else(|| {
                    *top_left = Point::ORIGIN;
                    *bottom_right = Point::new(x as f32 / scale_factor, y as f32 / scale_factor);
                    *size = (*bottom_right - *top_left).into();
                    *status = CropState::FullScreen;
                    Some(())
                });
        }
    }

    pub fn allows_drawing(&self) -> bool {
        if let Mode::Draw {
            element: shape,
            state: status,
        } = self
        {
            shape.tool.is_valid() || (shape.tool.is_text_tool() && status.is_waiting_for_input())
        } else {
            false
        }
    }
}
