use iced::{
    event::Status,
    mouse::{Button, Cursor, Interaction},
    widget::canvas::{
        path::{arc::Elliptical, Builder},
        Event, Fill, Frame, Geometry, LineCap, LineDash, LineJoin, Path, Program, Stroke, Style,
    },
    Color, Point, Radians, Rectangle, Renderer, Size, Vector,
};

use crate::{
    entities::{
        capture::{
            shape::{Shape, ShapeType},
            CaptureEvent, CaptureWindow, Mode,
        },
        theme::Theme,
    },
    utils::evaluate_points,
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
            for shape in self.shapes.iter() {
                draw_shape(frame, shape);
            }
        });

        let mut frame = Frame::new(renderer, bounds.size());

        let overlay = Fill::from(Color::from_rgba(0.0, 0.0, 0.0, 0.5));

        match self.mode {
            Mode::Draw => {
                draw_shape(&mut frame, &self.shape);
            }
            Mode::Crop => {
                if let Some(initial_pt) = self.endpoints.initial_pt {
                    let final_pt = if let Some(final_pt) = self.endpoints.final_pt {
                        final_pt
                    } else {
                        self.cursor_position
                    };

                    let (top_left, bottom_right) = evaluate_points(initial_pt, final_pt);
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

                    let (width, height) =
                        (bottom_right.x - top_left.x, bottom_right.y - top_left.y);

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
                } else {
                    frame.fill_rectangle(Point::ORIGIN, bounds.size(), overlay);
                };
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
    if let (Some(initial_pt), Some(final_pt)) =
        (shape.endpoints.initial_pt, shape.endpoints.final_pt)
    {
        let shape_type = shape.shape_type;
        let color = shape.color.into_iced_color();
        match shape_type {
            ShapeType::Rectangle => {
                let top_left = initial_pt;
                let size = (final_pt - initial_pt).into();
                let path = Path::rectangle(top_left, size);
                if shape.is_filled {
                    let fill = Fill::from(color);
                    frame.fill(&path, fill);
                } else {
                    let stroke = Stroke::default()
                        .with_width(shape.stroke_width.f32())
                        .with_color(color)
                        .with_line_join(LineJoin::Round);
                    frame.stroke(&path, stroke);
                }
            }
            ShapeType::Ellipse => {
                let top_left = initial_pt;
                let size = final_pt - initial_pt;
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
                    let stroke = Stroke::default()
                        .with_width(shape.stroke_width.f32())
                        .with_color(color);
                    frame.stroke(&path, stroke);
                };
            }
            ShapeType::Line => {
                let path = Path::line(initial_pt, final_pt);
                let stroke = Stroke::default()
                    .with_width(shape.stroke_width.f32())
                    .with_color(color);
                frame.stroke(&path, stroke);
            }
        }
    }
}
