use iced::{
    alignment::{Horizontal, Vertical},
    event::Status,
    mouse::{Button, Cursor, Interaction},
    widget::canvas::{
        path::{arc::Elliptical, Builder},
        Event, Fill, Frame, Geometry, LineCap, LineDash, Path, Program, Stroke, Style, Text,
    },
    Color, Pixels, Point, Radians, Rectangle, Renderer, Size, Vector,
};

use crate::{
    consts::FONT_MEDIUM,
    theme::Theme,
    windows::capture_window::models::{Mode, SelectionMode},
};

use super::{
    models::{DrawingTool, Shape},
    utils::{normalize, resolve_arrow_points},
    CaptureEvent, CaptureWindow,
};

impl Program<CaptureEvent, Theme> for CaptureWindow {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry<Renderer>> {
        let shapes_frame = self.cache.draw(renderer, bounds.size(), |frame| {
            self.shapes
                .iter()
                .for_each(|shape| draw_shape(frame, shape))
        });

        let mut frame = Frame::new(renderer, bounds.size());

        match self.mode {
            Mode::Draw => {
                draw_shape(&mut frame, &self.shape);
            }
            Mode::Crop => {
                let overlay = Fill::from(Color::from_rgba(0.0, 0.0, 0.0, 0.8));
                let (top_left, bottom_right) = match self.selection_mode {
                    SelectionMode::FullScreen => {
                        let (x, y) = self.image.dimensions();
                        (Point::ORIGIN, Point::new(x as f32, y as f32))
                    }
                    SelectionMode::Window(id) => {
                        let window = self.windows.get(&id).unwrap();
                        let x = if window.x < 0 { 0u32 } else { window.x as u32 };
                        let y = if window.y < 0 { 0u32 } else { window.y as u32 };
                        (
                            Point::new(x as f32, y as f32),
                            Point::new(
                                (window.x + window.width as i32) as f32,
                                (window.y + window.height as i32) as f32,
                            ),
                        )
                    }
                    SelectionMode::InProgress(initial_pt) => {
                        normalize(initial_pt, self.cursor_position)
                    }
                    SelectionMode::Area(points) => (points[0], points[1]),
                };

                let selection = Path::rectangle(top_left, (bottom_right - top_left).into());
                let stroke = Stroke {
                    style: Style::Solid(Color::from_rgba8(255, 255, 255, 0.2)),
                    width: 1.0,
                    ..Default::default()
                };
                frame.fill_rectangle(
                    Point::new(0.0, 0.0),
                    Size {
                        height: top_left.y,
                        width: bounds.width,
                    },
                    overlay,
                );
                frame.fill_rectangle(
                    Point::new(0.0, bottom_right.y),
                    Size {
                        height: bounds.height - bottom_right.y,
                        width: bounds.width,
                    },
                    overlay,
                );
                frame.fill_rectangle(
                    Point::new(0.0, top_left.y),
                    Size {
                        height: bottom_right.y - top_left.y,
                        width: top_left.x,
                    },
                    overlay,
                );
                frame.fill_rectangle(
                    Point::new(bottom_right.x, top_left.y),
                    Size {
                        height: bottom_right.y - top_left.y,
                        width: bounds.width - bottom_right.x,
                    },
                    overlay,
                );

                frame.stroke(&selection, stroke);

                let (width, height) = (bottom_right.x - top_left.x, bottom_right.y - top_left.y);

                let horizontal_segment_len = if width > 80.0 { 20.0 } else { width / 4.0 };

                let vertical_segment_len = if height > 80.0 { 20.0 } else { height / 4.0 };

                let dashed_stroke = Stroke {
                    style: Style::Solid(Color::WHITE),
                    width: 4.0,
                    line_cap: LineCap::Square,
                    line_dash: LineDash {
                        segments: &[
                            horizontal_segment_len,
                            width - (2.0 * horizontal_segment_len),
                            horizontal_segment_len,
                            0.0,
                            vertical_segment_len,
                            height - (2.0 * vertical_segment_len),
                            vertical_segment_len,
                            0.0,
                            horizontal_segment_len,
                            width - (2.0 * horizontal_segment_len),
                            horizontal_segment_len,
                            0.0,
                            vertical_segment_len,
                            height - (2.0 * vertical_segment_len),
                            vertical_segment_len,
                        ],
                        offset: 0,
                    },
                    ..Default::default()
                };

                frame.stroke(&selection, dashed_stroke);
            }
        }

        vec![shapes_frame, frame.into_geometry()]
    }

    fn update(
        &self,
        _state: &mut Self::State,
        event: Event,
        _bounds: Rectangle,
        _cursor: Cursor,
    ) -> (Status, Option<CaptureEvent>) {
        match event {
            iced::widget::canvas::Event::Mouse(event) => match event {
                iced::mouse::Event::CursorMoved { position } => (
                    Status::Captured,
                    Some(CaptureEvent::UpdateCurrentPosition(position)),
                ),
                iced::mouse::Event::ButtonPressed(button) => {
                    if button == Button::Left {
                        (Status::Captured, Some(CaptureEvent::SetInitialPoint))
                    } else {
                        (Status::Ignored, None)
                    }
                }
                iced::mouse::Event::ButtonReleased(button) => {
                    if button == Button::Left {
                        (Status::Captured, Some(CaptureEvent::SetFinalPoint))
                    } else {
                        (Status::Ignored, None)
                    }
                }
                _ => (Status::Ignored, None),
            },
            _ => (Status::Ignored, None),
        }
    }

    fn mouse_interaction(
        &self,
        _state: &Self::State,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> Interaction {
        if cursor.is_over(bounds) {
            Interaction::Crosshair
        } else {
            Interaction::default()
        }
    }
}

fn draw_shape(frame: &mut Frame, shape: &Shape) {
    if !shape.points.is_empty() {
        let points = shape.points.as_slice();
        let shape_type = shape.tool;
        let color = shape.color.into_iced_color(shape.is_solid);
        let stroke = Stroke::default()
            .with_width(shape.size.to_stroke_f32())
            .with_color(color);
        match shape_type {
            DrawingTool::Rectangle => {
                let (top_left, bottom_right) = normalize(points[0], points[1]);
                let size = (bottom_right - top_left).into();
                let path = Path::rectangle(top_left, size);
                if shape.is_filled {
                    let fill = Fill::from(color);
                    frame.fill(&path, fill);
                } else {
                    frame.stroke(&path, stroke);
                }
            }
            DrawingTool::Ellipse => {
                let (top_left, bottom_right) = normalize(points[0], points[1]);
                let size = bottom_right - top_left;
                let radii = Vector::new(size.x / 2.0, size.y / 2.0);
                let center = Point::new(top_left.x + radii.x, top_left.y + radii.y);
                let arc = Elliptical {
                    center,
                    radii,
                    rotation: Radians(0.0),
                    start_angle: Radians(0.0),
                    end_angle: Radians(360.0),
                };
                let mut builder = Builder::new();
                builder.ellipse(arc);
                let path = builder.build();
                if shape.is_filled {
                    let fill = Fill::from(color);
                    frame.fill(&path, fill);
                } else {
                    frame.stroke(&path, stroke);
                };
            }
            DrawingTool::FreeHand => {
                let mut builder = Builder::new();
                builder.move_to(points[0]);
                for point in points.iter().skip(1) {
                    builder.line_to(*point);
                }
                let path = builder.build();
                frame.stroke(&path, stroke);
            }
            DrawingTool::Line => {
                let path = Path::line(points[0], points[1]);
                frame.stroke(&path, stroke);
            }
            DrawingTool::Arrow => {
                let (right_pt, left_pt) = resolve_arrow_points(points[0], points[1]);
                let mut builder = Builder::new();
                builder.move_to(points[0]);
                builder.line_to(points[1]);
                builder.move_to(right_pt);
                builder.line_to(points[1]);
                builder.line_to(left_pt);
                let path = builder.build();
                frame.stroke(&path, stroke);
            }
            DrawingTool::Text => {
                let text = Text {
                    content: shape.text.clone(),
                    position: points[0],
                    size: Pixels(shape.size.to_text_size_f32()),
                    color,
                    font: FONT_MEDIUM,
                    horizontal_alignment: Horizontal::Left,
                    vertical_alignment: Vertical::Top,
                    ..Default::default()
                };
                frame.fill_text(text);
            }
        }
    }
}
