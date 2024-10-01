use iced::{
    widget::{
        button, canvas, column, container, horizontal_space, image::Handle, row, stack, text,
        vertical_space, Image,
    },
    Alignment::Center,
    Length::Fill,
    Task,
};

pub mod annotate;

use crate::{
    assets::{
        CANCEL, DONE, ELLIPSE_FILLED, ELLIPSE_STROKE, ICON, LINE, RECT_FILLED, RECT_STROKE,
        STROKE_BROAD, STROKE_MEDIUM, STROKE_THIN,
    },
    entities::{
        capture::{
            shape::{ShapeColor, ShapeStroke, ShapeType},
            CaptureEvent, CaptureWindow, Mode,
        },
        style::{ButtonClass, TextClass},
    },
    theme::Element,
    utils::evaluate_points,
    AppEvent,
};

impl CaptureWindow {
    pub fn update(&mut self, message: CaptureEvent) -> Task<AppEvent> {
        match message {
            CaptureEvent::Done => {
                if self.mode == Mode::Draw {
                    self.mode = Mode::Crop;
                } else {
                    return Task::done(AppEvent::RequestClose);
                }
            }
            CaptureEvent::Cancel => {
                if self.mode == Mode::Draw {
                    self.shapes.clear();
                    self.cache.clear();
                    self.mode = Mode::Crop;
                } else {
                    if self.endpoints.final_pt.is_none() {
                        return Task::done(AppEvent::RequestClose);
                    }
                    self.endpoints.clear();
                }
            }
            CaptureEvent::ChooseShapeType(shape_type, is_filled) => {
                self.endpoints.clear();
                self.mode = Mode::Draw;
                self.shape.shape_type = shape_type;
                self.shape.is_filled = is_filled;
            }
            CaptureEvent::ChangeStroke(stroke_width) => {
                self.shape.stroke_width = stroke_width;
            }
            CaptureEvent::ChangeColor(color) => {
                self.shape.color = color;
            }
            CaptureEvent::SetInitialPoint => match self.mode {
                Mode::Draw => self.shape.endpoints.initial_pt = Some(self.cursor_position),
                Mode::Crop => {
                    self.endpoints.clear();
                    self.endpoints.initial_pt = Some(self.cursor_position)
                }
            },
            CaptureEvent::UpdateCurrentPosition(final_pt) => {
                if self.mode == Mode::Draw {
                    if let Some(initial_pt) = self.shape.endpoints.initial_pt {
                        match self.shape.shape_type {
                            ShapeType::Rectangle | ShapeType::Ellipse => {
                                let (initial_pt, final_pt) = evaluate_points(initial_pt, final_pt);
                                self.shape.endpoints.initial_pt = Some(initial_pt);
                                self.shape.endpoints.final_pt = Some(final_pt);
                            }
                            ShapeType::Line => self.shape.endpoints.final_pt = Some(final_pt),
                        }
                    }
                }
                self.cursor_position = final_pt;
            }
            CaptureEvent::SetFinalPoint => {
                match self.mode {
                    Mode::Draw => {
                        if self.shape.endpoints.initial_pt != Some(self.cursor_position) {
                            self.shapes.push(self.shape);
                            self.cache.clear();
                        }
                        self.shape.endpoints.clear();
                    }
                    Mode::Crop => {
                        if self.endpoints.initial_pt != Some(self.cursor_position) {
                            self.endpoints.final_pt = Some(self.cursor_position);
                        } else {
                            self.endpoints.clear();
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
            self.image.width(),
            self.image.height(),
            self.image.clone().into_raw(),
        ))
        .height(Fill)
        .width(Fill);

        const ROW: u16 = 8;
        const TEXT: u16 = 18;
        const SQUARE: u16 = 36;

        let panel = |row| container(row).align_x(Center).align_y(Center).padding(8);

        let mut toolbar = row![].spacing(10);

        toolbar = toolbar.push(horizontal_space().width(Fill));

        let shapes_icon = |utf, shape_type, is_filled| {
            let button_class = if self.mode == Mode::Draw
                && self.shape.shape_type == shape_type
                && self.shape.is_filled == is_filled
            {
                ButtonClass::Selected
            } else {
                ButtonClass::Default
            };

            button(text(utf).font(ICON).size(TEXT).center())
                .on_press(CaptureEvent::ChooseShapeType(shape_type, is_filled))
                .height(SQUARE)
                .width(SQUARE)
                .class(button_class)
        };

        let shapes = panel(
            row![
                shapes_icon(RECT_FILLED, ShapeType::Rectangle, true),
                shapes_icon(RECT_STROKE, ShapeType::Rectangle, false),
                shapes_icon(ELLIPSE_FILLED, ShapeType::Ellipse, true),
                shapes_icon(ELLIPSE_STROKE, ShapeType::Ellipse, false),
                shapes_icon(LINE, ShapeType::Line, false),
            ]
            .spacing(ROW),
        );

        toolbar = toolbar.push(shapes);

        if self.mode == Mode::Draw {
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

            let color_icon = |color| {
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
                        .class(TextClass::Custom(color.into_iced_color())),
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

        let mut options = row![].spacing(ROW);

        if self.mode == Mode::Draw || self.endpoints.final_pt.is_some() {
            options = options.push(
                button(text(DONE).font(ICON).size(TEXT).center())
                    .on_press(CaptureEvent::Done)
                    .height(SQUARE)
                    .width(SQUARE)
                    .class(ButtonClass::Selected),
            )
        };

        options = options.push(
            button(text(CANCEL).font(ICON).size(TEXT).center())
                .on_press(CaptureEvent::Cancel)
                .height(SQUARE)
                .width(SQUARE)
                .class(ButtonClass::Danger),
        );

        toolbar = toolbar.push(panel(options));

        toolbar = toolbar.push(horizontal_space().width(Fill));

        stack![
            background,
            canvas(self).height(Fill).width(Fill),
            column![vertical_space().height(5), toolbar]
        ]
        .height(Fill)
        .width(Fill)
        .into()
    }
}
