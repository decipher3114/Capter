use iced::{
    widget::{
        button, canvas, canvas::Cache, column, container, horizontal_space, image::Handle, row,
        stack, text, vertical_space, Image,
    },
    window::Id,
    Alignment::Center,
    Length::Fill,
    Point, Task,
};
use indexmap::IndexMap;
use models::{CapturedWindow, Mode, SelectionMode, Shape, ToolColor, StrokeWidth, DrawingTool};
use utils::normalize;
use xcap::image::RgbaImage;

use crate::{
    app::AppEvent,
    consts::{
        ARROW, ELLIPSE_FILLED, ELLIPSE_STROKE, FREE_HAND, HIGHLIGHT, ICON, LINE, RECT_FILLED,
        RECT_STROKE, STROKE_BROAD, STROKE_MEDIUM, STROKE_THIN,
    },
    theme::{button::ButtonClass, text::TextClass, Element},
};

pub mod annotate;
pub mod capture;
pub mod models;
pub mod utils;

pub struct CaptureWindow {
    pub scale_factor: f32,
    image: RgbaImage,
    windows: IndexMap<u32, CapturedWindow>,
    cursor_position: Point,
    selection_mode: SelectionMode,
    mode: Mode,
    shape: Shape,
    shapes: Vec<Shape>,
    cache: Cache,
}

#[derive(Debug, Clone)]
pub enum CaptureEvent {
    Undo,
    Done,
    Cancel,
    ChooseShapeType(DrawingTool, bool, bool),
    ChangeStroke(StrokeWidth),
    ChangeColor(ToolColor),
    SetInitialPoint,
    UpdateCurrentPosition(Point),
    SetFinalPoint,
}

impl CaptureWindow {
    pub fn update(&mut self, id: Id, message: CaptureEvent) -> Task<AppEvent> {
        match message {
            CaptureEvent::Undo => {
                if matches!(self.mode, Mode::Draw) {
                    self.shapes.pop();
                    self.cache.clear();
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
                self.shape.points = Vec::new();
                self.mode = Mode::Draw;
                self.shape.tool = shape_type;
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
                    self.shape.points.push(self.cursor_position);
                    self.shape.points.push(self.cursor_position);
                }
                Mode::Crop => {
                    self.selection_mode = SelectionMode::InProgress(self.cursor_position);
                }
            },
            CaptureEvent::UpdateCurrentPosition(final_pt) => {
                self.cursor_position = final_pt;
                match self.mode {
                    Mode::Crop => match self.selection_mode {
                        SelectionMode::FullScreen | SelectionMode::Window(_) => {
                            self.auto_detect_area();
                        }
                        _ => (),
                    },
                    Mode::Draw => {
                        if !self.shape.points.is_empty() {
                            match self.shape.tool {
                                DrawingTool::FreeHand => {
                                    self.shape.points.push(final_pt);
                                }
                                _ => {
                                    self.shape.points[1] = final_pt;
                                }
                            }
                        }
                    }
                }
            }
            CaptureEvent::SetFinalPoint => {
                match self.mode {
                    Mode::Draw => {
                        if !self.shape.points.is_empty() {
                            self.shapes.push(self.shape.clone());
                            self.cache.clear();
                        }
                        self.shape.points.clear();
                    }
                    Mode::Crop => {
                        if let SelectionMode::InProgress(initial_pt) = self.selection_mode {
                            if self.cursor_position != initial_pt {
                                let (top_left, bottom_right) =
                                    normalize(initial_pt, self.cursor_position);
                                self.selection_mode = SelectionMode::Area([top_left, bottom_right])
                            } else {
                                self.auto_detect_area();
                            }
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

        const CONTAINER: u16 = 10;
        const ROW: u16 = 10;
        const TEXT: u16 = 24;
        const SQUARE: u16 = 44;

        let panel = |row| {
            container(row)
                .align_x(Center)
                .align_y(Center)
                .padding(CONTAINER)
        };

        let mut toolbar = row![].spacing(12);

        toolbar = toolbar.push(horizontal_space().width(Fill));

        let shapes_icon = |utf, shape_type, is_filled, is_solid| {
            let button_class = if matches!(self.mode, Mode::Draw)
                && self.shape.tool == shape_type
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

        let row = row![
            shapes_icon(RECT_FILLED, DrawingTool::Rectangle, true, true),
            shapes_icon(RECT_STROKE, DrawingTool::Rectangle, false, true),
            shapes_icon(ELLIPSE_FILLED, DrawingTool::Ellipse, true, true),
            shapes_icon(ELLIPSE_STROKE, DrawingTool::Ellipse, false, true),
            shapes_icon(FREE_HAND, DrawingTool::FreeHand, false, true),
            shapes_icon(LINE, DrawingTool::Line, false, true),
            shapes_icon(ARROW, DrawingTool::Arrow, false, true),
            shapes_icon(HIGHLIGHT, DrawingTool::Rectangle, true, false)
        ];
        let shapes = panel(row.spacing(ROW));

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
                        stroke_icon(STROKE_THIN, StrokeWidth::Thin),
                        stroke_icon(STROKE_MEDIUM, StrokeWidth::Medium),
                        stroke_icon(STROKE_BROAD, StrokeWidth::Broad)
                    ]
                    .spacing(ROW),
                ))
            };

            let color_icon = |color: ToolColor| {
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
                        .class(TextClass::Custom(color.into_iced_color(true))),
                )
                .on_press(CaptureEvent::ChangeColor(color))
                .height(SQUARE)
                .width(SQUARE)
                .class(button_class)
            };

            toolbar = toolbar.push(panel(
                row![
                    color_icon(ToolColor::Red),
                    color_icon(ToolColor::Green),
                    color_icon(ToolColor::Blue),
                    color_icon(ToolColor::Yellow),
                    color_icon(ToolColor::Black),
                    color_icon(ToolColor::White)
                ]
                .spacing(ROW),
            ))
        };

        toolbar = toolbar.push(horizontal_space().width(Fill));

        let mut overlay = column![vertical_space().height(5)];

        if self.shape.points.is_empty() {
            overlay = overlay.push(toolbar);
        };

        let mode_description = match self.selection_mode {
            SelectionMode::FullScreen => String::from("Fullscreen"),
            SelectionMode::Window(id) => self.windows.get(&id).unwrap().name.clone(),
            SelectionMode::InProgress(initial_pt) => {
                let (top_left, bottom_right) = normalize(initial_pt, self.cursor_position);
                let area = bottom_right - top_left;
                format!("Area: {} x {}", area.x as u32, area.y as u32)
            }
            SelectionMode::Area(points) => {
                let area = points[1] - points[0];
                format!("Area: {} x {}", area.x as u32, area.y as u32)
            }
        };

        overlay = overlay.push(vertical_space().height(Fill));
        if matches!(self.mode, Mode::Crop) {
            overlay = overlay.push(row![
                horizontal_space().width(Fill),
                container(text(mode_description).size(TEXT).center())
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
