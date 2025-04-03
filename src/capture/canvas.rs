use std::ops::Mul;

use iced::{
    Color, Pixels, Point, Radians, Rectangle, Renderer, Size,
    alignment::{Horizontal, Vertical},
    widget::{
        Action,
        canvas::{
            Fill, Frame, Geometry, LineCap, LineDash, Path, Program, Stroke, Style, Text,
            path::{Builder, arc::Elliptical},
        },
        text::LineHeight,
    },
};

use crate::{capture::mode::Mode, consts::MEDIUM_FONT, theme::Theme};

use super::{
    Capture, DrawElement, Message,
    draw::{FONT_SIZE_FACTOR, STROKE_WIDHT_FACTOR, Tool},
};

impl Program<Message, Theme> for Capture {
    type State = ();

    fn update(
        &self,
        _state: &mut Self::State,
        event: &iced::Event,
        _bounds: Rectangle,
        _cursor: iced::advanced::mouse::Cursor,
    ) -> Option<Action<Message>> {
        match event {
            iced::Event::Mouse(event) => match event {
                iced::mouse::Event::CursorMoved { position } => {
                    Some(Action::publish(Message::MouseMoved(*position)))
                }
                iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left) => {
                    Some(Action::publish(Message::MousePressed))
                }
                iced::mouse::Event::ButtonReleased(iced::mouse::Button::Left) => {
                    Some(Action::publish(Message::MouseReleased))
                }
                _ => None,
            },
            _ => None,
        }
    }

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: iced::advanced::mouse::Cursor,
    ) -> Vec<Geometry<Renderer>> {
        let mut frame = Frame::new(renderer, bounds.size());

        let mut overlay_frame = Frame::new(renderer, bounds.size());
        overlay_frame.fill_rectangle(
            Point::ORIGIN,
            bounds.size(),
            Fill::from(Color::from_rgba(0.0, 0.0, 0.0, 0.5)),
        );

        let shapes_frame = self.cache.draw(renderer, bounds.size(), |frame| {
            self.shapes
                .iter()
                .for_each(|shape| draw_shape(frame, shape, false));
        });

        match &self.mode {
            Mode::Draw { element: shape, .. } => {
                if self.mode.allows_drawing() {
                    draw_shape(&mut frame, shape, true);
                }
            }
            Mode::Crop {
                top_left,
                bottom_right,
                size,
                ..
            } => {
                let overlay = Fill::from(Color::from_rgba(0.0, 0.0, 0.0, 0.9));

                let selection = Path::rectangle(top_left.to_owned(), size.to_owned());

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

                let (width, height) = (size.width, size.height);

                let segment_len = |dim| if dim > 80.0 { 20.0 } else { dim / 4.0 };
                let horizontal_segment_len = segment_len(width);
                let vertical_segment_len = segment_len(height);

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
                        ],
                        offset: 0,
                    },
                    ..Default::default()
                };

                frame.stroke(&selection, dashed_stroke);
            }
        }

        vec![
            overlay_frame.into_geometry(),
            shapes_frame,
            frame.into_geometry(),
        ]
    }

    fn mouse_interaction(
        &self,
        _state: &Self::State,
        bounds: Rectangle,
        cursor: iced::advanced::mouse::Cursor,
    ) -> iced::mouse::Interaction {
        if cursor.is_over(bounds) {
            if let Mode::Draw { element: shape, .. } = &self.mode {
                if shape.tool.is_text_tool() {
                    return iced::mouse::Interaction::Text;
                }
            }
            return iced::mouse::Interaction::Crosshair;
        }
        iced::mouse::Interaction::default()
    }
}

fn draw_shape(frame: &mut Frame, shape: &DrawElement, guide: bool) {
    let tool = shape.tool.clone();
    let color = shape.color.into();
    let stroke = Stroke::default()
        .with_width(shape.size.mul(STROKE_WIDHT_FACTOR) as f32)
        .with_color(color);
    match tool {
        Tool::Rectangle {
            top_left,
            bottom_right: _,
            size,
            is_filled,
            is_opaque,
            ..
        } => {
            let path = Path::rectangle(top_left, size);
            if is_filled {
                if is_opaque {
                    frame.fill(&path, color);
                } else {
                    frame.fill(&path, shape.color.into_translucent_color());
                }
            } else {
                frame.stroke(&path, stroke);
            }
        }
        Tool::Ellipse {
            center,
            radii,
            is_filled,
            ..
        } => {
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
            if is_filled {
                frame.fill(&path, color);
            } else {
                frame.stroke(&path, stroke);
            };
        }
        Tool::FreeHand { points } => {
            let mut builder = Builder::new();

            builder.move_to(points[0]);
            points
                .iter()
                .skip(1)
                .for_each(|point| builder.line_to(*point));
            let path = builder.build();

            frame.stroke(&path, stroke);
        }
        Tool::Line { start, end } => {
            let path = Path::line(start, end);
            frame.stroke(&path, stroke);
        }
        Tool::Arrow {
            start,
            end,
            right,
            left,
        } => {
            let mut builder = Builder::new();
            builder.move_to(start);
            builder.line_to(end);
            builder.move_to(right);
            builder.line_to(end);
            builder.line_to(left);
            let path = builder.build();
            frame.stroke(&path, stroke);
        }
        Tool::Text {
            anchor_point: mid_point,
            text,
        } => {
            let font_size = shape.size.mul(FONT_SIZE_FACTOR);

            let top_left = Point::new(mid_point.x, mid_point.y - (font_size / 2) as f32);

            if guide {
                frame.stroke_rectangle(
                    top_left,
                    Size::new(frame.width() - mid_point.x, font_size as f32),
                    Stroke::default().with_color(Color::WHITE),
                );
            }

            let text = Text {
                content: text,
                position: top_left,
                size: Pixels(font_size as f32),
                color,
                font: MEDIUM_FONT,
                align_x: Horizontal::Left,
                align_y: Vertical::Top,
                line_height: LineHeight::Relative(1.0),
                ..Default::default()
            };

            frame.fill_text(text);
        }
    }
}
