use iced::{Point, Size, widget::text_input::focus};

use crate::{
    action::Action,
    capture::{
        Capture, Message, Request,
        crop::CropState,
        draw::{DrawElement, DrawState},
        mode::Mode,
    },
};

impl Capture {
    pub fn update(&mut self, message: Message) -> Action<Message, Request> {
        match message {
            Message::MoveToolBar => {
                self.toolbar_at_top = !self.toolbar_at_top;
            }
            Message::Undo => {
                if self.mode.is_draw_mode() {
                    self.shapes.pop();
                    self.cache.clear();
                }
            }
            Message::Done => {
                match &self.mode {
                    Mode::Draw { element: shape, .. } => {
                        if shape.tool.is_text_tool() {
                            self.push_shape();
                        }
                        self.mode = Mode::default();
                        self.mode.get_window_below_cursor(
                            &self.windows,
                            &self.cursor_position,
                            self.scale_factor,
                            self.image.dimensions(),
                        );
                    }
                    Mode::Crop { .. } => {
                        return Action::requests([Request::Close]);
                    }
                }
            }
            Message::Cancel => {
                match &mut self.mode {
                    Mode::Draw { .. } => {
                        self.shapes.clear();
                        self.cache.clear();
                        self.mode = Mode::default();
                        self.mode.get_window_below_cursor(
                            &self.windows,
                            &self.cursor_position,
                            self.scale_factor,
                            self.image.dimensions(),
                        );
                    }
                    Mode::Crop { state: status, .. } => {
                        *status = CropState::None;
                        return Action::requests([Request::Close]);
                    }
                }
            }
            Message::ChangeTool(tool) => {
                self.push_shape();
                if let Mode::Draw { element: shape, .. } = &mut self.mode {
                    shape.tool = tool;
                } else {
                    self.mode = Mode::Draw {
                        element: DrawElement {
                            tool,
                            ..Default::default()
                        },
                        state: DrawState::Idle,
                    }
                }
            }
            Message::ChangeSize(stroke_width) => {
                self.push_shape();
                if let Mode::Draw {
                    element: shape,
                    state: status,
                } = &mut self.mode
                {
                    shape.size = stroke_width;
                    *status = DrawState::Idle;
                }
            }
            Message::ChangeColor(color) => {
                self.push_shape();
                if let Mode::Draw {
                    element: shape,
                    state: status,
                } = &mut self.mode
                {
                    shape.color = color;
                    *status = DrawState::Idle;
                }
            }
            Message::UpdateText(text) => {
                if let Mode::Draw { element: shape, .. } = &mut self.mode {
                    shape.tool.update_text(text);
                }
            }
            Message::MousePressed => {
                match &mut self.mode {
                    Mode::Crop {
                        top_left,
                        bottom_right,
                        size,
                        state: status,
                    } => {
                        *top_left = self.cursor_position;
                        *bottom_right = self.cursor_position;
                        *size = Size::ZERO;
                        *status = CropState::InProgress {
                            start: self.cursor_position,
                            end: self.cursor_position,
                        };
                    }
                    Mode::Draw {
                        element: shape,
                        state: status,
                    } => {
                        if shape.tool.is_text_tool() && shape.tool.is_valid() {
                            self.shapes.push(shape.clone());
                            self.cache.clear();
                            shape.tool.reset();
                        }

                        shape.tool.initiate(self.cursor_position);
                        *status = DrawState::InProgress {
                            initial_pt: self.cursor_position,
                            final_pt: self.cursor_position,
                        };
                    }
                }
            }
            Message::MouseMoved(position) => {
                self.cursor_position = position;
                match &mut self.mode {
                    Mode::Crop {
                        top_left,
                        bottom_right,
                        size,
                        state: status,
                    } => {
                        match status {
                            CropState::FullScreen | CropState::Window(_) => {
                                self.mode.get_window_below_cursor(
                                    &self.windows,
                                    &self.cursor_position,
                                    self.scale_factor,
                                    self.image.dimensions(),
                                );
                            }
                            CropState::InProgress { start, end } => {
                                *end = position;
                                *top_left = Point::new(start.x.min(end.x), start.y.min(end.y));
                                *bottom_right = Point::new(start.x.max(end.x), start.y.max(end.y));
                                *size = Size::new(
                                    bottom_right.x - top_left.x,
                                    bottom_right.y - top_left.y,
                                );
                            }
                            _ => {}
                        }
                    }
                    Mode::Draw {
                        element: shape,
                        state: status,
                    } => {
                        if shape.tool.is_text_tool() {
                            return Action::none();
                        };
                        if let DrawState::InProgress {
                            initial_pt,
                            final_pt,
                        } = status
                        {
                            *final_pt = position;
                            shape.tool.update(*initial_pt, *final_pt);
                        }
                    }
                }
            }
            Message::MouseReleased => {
                match &mut self.mode {
                    Mode::Crop { state: status, .. } => {
                        if let CropState::InProgress { start, end } = status {
                            if start != end {
                                *status = CropState::Area;
                            } else {
                                self.mode.get_window_below_cursor(
                                    &self.windows,
                                    &self.cursor_position,
                                    self.scale_factor,
                                    self.image.dimensions(),
                                );
                            }
                        }
                    }
                    Mode::Draw {
                        element: shape,
                        state: status,
                    } => {
                        if shape.tool.is_text_tool() {
                            *status = DrawState::TextInput;
                            return focus("text_input").into();
                        } else {
                            if shape.tool.is_valid() {
                                self.shapes.push(shape.clone());
                                self.cache.clear();
                                shape.tool.reset();
                            }
                            *status = DrawState::Idle;
                        }
                    }
                }
            }
        }
        Action::none()
    }

    fn push_shape(&mut self) {
        if let Mode::Draw {
            element: shape,
            state: status,
        } = &mut self.mode
        {
            if shape.tool.is_valid() {
                self.shapes.push(shape.clone());
            }
            shape.tool.reset();
            *status = DrawState::Idle;
        }
        self.cache.clear();
    }
}
