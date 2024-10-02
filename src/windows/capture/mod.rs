use iced::{
    widget::{
        button, canvas, column, container, horizontal_space, image::Handle, row, stack, text,
        vertical_space, Image,
    },
    window::Id,
    Alignment::Center,
    Length::Fill,
    Point, Task,
};

pub mod annotate;

use crate::{
    assets::{
        ELLIPSE_FILLED, ELLIPSE_STROKE, HIGHLIGHT, ICON, LINE, RECT_FILLED, RECT_STROKE,
        STROKE_BROAD, STROKE_MEDIUM, STROKE_THIN,
    },
    entities::{
        capture::{
            shape::{ShapeColor, ShapeStroke, ShapeType},
            CaptureEvent, CaptureWindow, Endpoints, Mode, CropMode,
        },
        style::{ButtonClass, TextClass},
    },
    theme::Element,
    AppEvent,
};

impl CaptureWindow {
    pub fn update(&mut self, id: Id, message: CaptureEvent) -> Task<AppEvent> {
        match message {
            CaptureEvent::Undo => {
                if matches!(self.mode, Mode::Draw) {
                    self.shapes.pop();
                    self.cache.clear();
                } else {
                    self.crop_mode = CropMode::FullScreen;
                }
            }
            CaptureEvent::Done => {
                if matches!(self.mode, Mode::Draw) {
                    self.mode = Mode::Crop;
                } else {
                    return Task::done(AppEvent::RequestClose(id));
                }
            }
            CaptureEvent::Cancel => {
                if matches!(self.mode, Mode::Draw) {
                    self.shapes.clear();
                    self.cache.clear();
                    self.mode = Mode::Crop;
                } else {
                    return Task::done(AppEvent::RequestClose(id));
                }
            }
            CaptureEvent::ChooseShapeType(shape_type, is_filled, is_solid) => {
                self.shape.endpoints = None;
                self.mode = Mode::Draw;
                self.shape.shape_type = shape_type;
                self.shape.is_filled = is_filled;
                self.shape.is_solid = is_solid;
            }
            CaptureEvent::ChangeStroke(stroke_width) => {
                self.shape.stroke_width = stroke_width;
            }
            CaptureEvent::ChangeColor(color) => {
                self.shape.color = color;
            }
            CaptureEvent::SetInitialPoint => match self.mode {
                Mode::Draw => {
                    self.shape.endpoints = Some(Endpoints {
                        initial_pt: self.cursor_position,
                        final_pt: self.cursor_position,
                    })
                }
                Mode::Crop => {
                    self.crop_mode = CropMode::SelectionInProgress;
                    self.endpoints.initial_pt = self.cursor_position;
                    self.endpoints.final_pt = self.cursor_position;
                }
            },
            CaptureEvent::UpdateCurrentPosition(final_pt) => {
                self.cursor_position = final_pt;
                if matches!(self.mode, Mode::Draw) {
                    if let Some(ref mut endpoints) = self.shape.endpoints {
                        endpoints.final_pt = final_pt;
                    }
                } else if matches!(
                    self.crop_mode,
                    CropMode::FullScreen | CropMode::SpecificWindow(_)
                ) {
                    let window = self.windows.iter().find_map(|(id, window)| {
                        let scale = self.scale_factor;
                        let top_left = (window.x as f32 / scale, window.y as f32 / scale);
                        let bottom_right = (
                            (window.x + window.width as i32) as f32 / scale,
                            (window.y + window.height as i32) as f32 / scale,
                        );
                        if (top_left.0..bottom_right.0).contains(&(self.cursor_position.x as f32))
                            && (top_left.1..bottom_right.1)
                                .contains(&(self.cursor_position.y as f32))
                        {
                            Some((id, window.name.clone(), top_left, bottom_right))
                        } else {
                            None
                        }
                    });
                    if let Some((id, name, top_left, bottom_right)) = window {
                        self.endpoints.initial_pt = Point::new(top_left.0, top_left.1);
                        self.endpoints.final_pt = Point::new(bottom_right.0, bottom_right.1);
                        self.crop_mode = CropMode::SpecificWindow(*id);
                        self.mode_desc = name;
                    } else {
                        self.crop_mode = CropMode::FullScreen;
                        self.mode_desc = String::from("FullScreen");
                    }
                } else if matches!(self.crop_mode, CropMode::SelectionInProgress) {
                    self.endpoints.final_pt = final_pt;
                    let (initial_pt, final_pt) = self.endpoints.normalize();
                    let size = initial_pt - final_pt;
                    self.mode_desc = format!("{} x {}", size.x as u32, size.y as u32);
                }
            }
            CaptureEvent::SetFinalPoint => {
                match self.mode {
                    Mode::Draw => {
                        if self.shape.endpoints.is_some() {
                            self.shapes.push(self.shape);
                            self.cache.clear();
                        }
                        self.shape.endpoints = None
                    }
                    Mode::Crop => {
                        if self.endpoints.initial_pt != self.cursor_position {
                            self.endpoints.final_pt = self.cursor_position;
                            self.crop_mode = CropMode::ManualSelection
                        } else {
                            self.crop_mode = CropMode::FullScreen;
                        }
                    }
                }
                return Task::none();
            }
        }
        Task::none()
    }

    pub fn view(&self) -> Element<CaptureEvent> {
        let background = Image::new(Handle::from_rgba(
            self.display.image.width(),
            self.display.image.height(),
            self.display.image.clone().into_raw(),
        ))
        .height(Fill)
        .width(Fill);

        const CONTAINER: u16 = 8;
        const ROW: u16 = 8;
        const TEXT: u16 = 18;
        const SQUARE: u16 = 36;

        let panel = |row| {
            container(row)
                .align_x(Center)
                .align_y(Center)
                .padding(CONTAINER)
        };

        let mut toolbar = row![].spacing(10);

        toolbar = toolbar.push(horizontal_space().width(Fill));

        let shapes_icon = |utf, shape_type, is_filled, is_solid| {
            let button_class = if matches!(self.mode, Mode::Draw)
                && self.shape.shape_type == shape_type
                && self.shape.is_filled == is_filled
                && self.shape.is_solid == is_solid
            {
                ButtonClass::Selected
            } else {
                ButtonClass::Default
            };

            button(text(utf).font(ICON).size(TEXT).center())
                .on_press(CaptureEvent::ChooseShapeType(
                    shape_type, is_filled, is_solid,
                ))
                .height(SQUARE)
                .width(SQUARE)
                .class(button_class)
        };

        let shapes = panel(
            row![
                shapes_icon(RECT_FILLED, ShapeType::Rectangle, true, true),
                shapes_icon(RECT_STROKE, ShapeType::Rectangle, false, true),
                shapes_icon(ELLIPSE_FILLED, ShapeType::Ellipse, true, true),
                shapes_icon(ELLIPSE_STROKE, ShapeType::Ellipse, false, true),
                shapes_icon(LINE, ShapeType::Line, false, true),
                shapes_icon(HIGHLIGHT, ShapeType::Rectangle, true, false)
            ]
            .spacing(ROW),
        );

        toolbar = toolbar.push(shapes);

        if matches!(self.mode, Mode::Draw) {
            if !self.shape.is_filled {
                let stroke_icon = |utf, stroke| {
                    let button_class = if self.shape.stroke_width == stroke {
                        ButtonClass::Selected
                    } else {
                        ButtonClass::Default
                    };

                    button(text(utf).font(ICON).size(TEXT).center())
                        .on_press(CaptureEvent::ChangeStroke(stroke))
                        .height(SQUARE)
                        .width(SQUARE)
                        .class(button_class)
                };
                toolbar = toolbar.push(panel(
                    row![
                        stroke_icon(STROKE_THIN, ShapeStroke::Thin),
                        stroke_icon(STROKE_MEDIUM, ShapeStroke::Medium),
                        stroke_icon(STROKE_BROAD, ShapeStroke::Broad)
                    ]
                    .spacing(ROW),
                ))
            };

            let color_icon = |color: ShapeColor| {
                let button_class = if self.shape.color == color {
                    ButtonClass::Selected
                } else {
                    ButtonClass::Default
                };

                button(
                    text(RECT_FILLED)
                        .font(ICON)
                        .size(TEXT)
                        .center()
                        // .color(color!(0.0, 0.0, 0.0, 1.0))
                        .class(TextClass::Custom(color.into_iced_color(true))),
                )
                .on_press(CaptureEvent::ChangeColor(color))
                .height(SQUARE)
                .width(SQUARE)
                .class(button_class)
            };

            toolbar = toolbar.push(panel(
                row![
                    color_icon(ShapeColor::Red),
                    color_icon(ShapeColor::Green),
                    color_icon(ShapeColor::Blue),
                    color_icon(ShapeColor::Yellow),
                    color_icon(ShapeColor::Black),
                    color_icon(ShapeColor::White)
                ]
                .spacing(ROW),
            ))
        };

        toolbar = toolbar.push(horizontal_space().width(Fill));

        let mut overlay = column![vertical_space().height(5)];

        if matches!(self.crop_mode, CropMode::SelectionInProgress) || self.shape.endpoints.is_none()
        {
            overlay = overlay.push(toolbar);
        };

        overlay = overlay.push(vertical_space().height(Fill));
        if matches!(self.mode, Mode::Crop) {
            overlay = overlay.push(row![
                horizontal_space().width(Fill),
                container(text(self.mode_desc.clone()).size(TEXT).center())
                    .align_x(Center)
                    .align_y(Center)
                    .padding(CONTAINER),
                horizontal_space().width(Fill)
            ]);
        }

        overlay = overlay.push(vertical_space().height(5));

        stack![background, canvas(self).height(Fill).width(Fill), overlay]
            .height(Fill)
            .width(Fill)
            .into()
    }
}
