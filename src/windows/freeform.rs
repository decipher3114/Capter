use iced::{
    advanced::graphics::geometry,
    mouse::{Cursor, Interaction},
    widget::{
        canvas::{Frame, Geometry, LineCap, LineDash, LineJoin, Path, Program, Stroke, Style}, column, container, horizontal_space, image::Handle, mouse_area, row, stack, text, vertical_space, Canvas, Image
    },
    Alignment::Center,
    Color,
    Length::Fill,
    Point, Rectangle, Renderer, Size, Task,
};

use crate::{
    entities::{
        freeform::{FreeFormWindow, FreeFormEvent},
        theme::Theme,
    },
    style::Element,
    utils::evaluate_points,
    AppEvent,
};

impl FreeFormWindow {
    pub fn update(&mut self, message: FreeFormEvent) -> Task<AppEvent> {
        match message {
            FreeFormEvent::SetInitialPoint => {
                self.selection_area.final_pos = None;
                self.selection_area.initial_pos = Some(self.cursor_position);
            }
            FreeFormEvent::UpdateCurrentPosition(point) => {
                self.cursor_position = point;
            }
            FreeFormEvent::SetFinalPoint => {
                if Some(self.cursor_position) != self.selection_area.initial_pos {
                    self.selection_area.final_pos = Some(self.cursor_position);
                } else {
                    self.selection_area.initial_pos = None;
                }
            }
        }
        Task::none()
    }

    pub fn view(&self) -> Element<FreeFormEvent> {

        let background: Image<Handle> = Image::new(
            Handle::from_rgba(
                self.image.width(),
                self.image.height(),
                self.image.clone().into_raw(),
            )
            // Handle::from_bytes(self.image.clone().into_raw())
        )
        .height(Fill)
        .width(Fill);

        let text_hint = match self.selection_area.final_pos {
            Some(_) => "Enter to Capture | Esc to Cancel",
            None => "Esc to Cancel"
        };
        stack![
            background,
            column![
                vertical_space().height(5),
                row![
                    horizontal_space().width(Fill),
                    container(text(text_hint).size(15).align_x(Center).align_y(Center))
                        .padding(10)
                        .align_x(Center)
                        .align_x(Center),
                    horizontal_space().width(Fill)
                ]
            ],
            mouse_area(Canvas::new(self).height(Fill).width(Fill))
            .on_move(FreeFormEvent::UpdateCurrentPosition)
            .on_press(FreeFormEvent::SetInitialPoint)
            .on_release(FreeFormEvent::SetFinalPoint)
        ]
        .height(Fill)
        .width(Fill)
        .into()
    }
}

impl Program<FreeFormEvent, Theme> for FreeFormWindow {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry<Renderer>> {
        let mut frame = Frame::new(renderer, bounds.size());

        let overlay = geometry::Fill::from(Color::from_rgba(0.0, 0.0, 0.0, 0.5));

        if let Some(mut initial_pos) = self.selection_area.initial_pos {
            let mut final_pos = if let Some(final_pos) = self.selection_area.final_pos {
                final_pos
            } else {
                self.cursor_position
            };
            (initial_pos, final_pos) = evaluate_points(initial_pos, final_pos);

            let selection = Path::rectangle(initial_pos, (final_pos - initial_pos).into());
            let stroke = Stroke {
                style: Style::Solid(Color::from_rgba8(255, 255, 255, 0.2)),
                width: 1.0,
                line_cap: LineCap::default(),
                line_join: LineJoin::default(),
                line_dash: LineDash::default(),
            };
            frame.fill_rectangle(
                Point::new(0.0, 0.0),
                Size {
                    height: initial_pos.y,
                    width: bounds.width,
                },
                overlay,
            );
            frame.fill_rectangle(
                Point::new(0.0, final_pos.y),
                Size {
                    height: bounds.height - final_pos.y,
                    width: bounds.width,
                },
                overlay,
            );
            frame.fill_rectangle(
                Point::new(0.0, initial_pos.y),
                Size {
                    height: final_pos.y - initial_pos.y,
                    width: initial_pos.x,
                },
                overlay,
            );
            frame.fill_rectangle(
                Point::new(final_pos.x, initial_pos.y),
                Size {
                    height: final_pos.y - initial_pos.y,
                    width: bounds.width - final_pos.x,
                },
                overlay,
            );

            frame.stroke(&selection, stroke);

            let (width, height) = (final_pos.x - initial_pos.x, final_pos.y - initial_pos.y);

            let horizontal_segment_len = if width > 80.0 { 20.0 } else { width / 4.0 };
            let vertical_segment_len = if height > 80.0 { 20.0 } else { height / 4.0 };

            let edge_stroke = Stroke {
                style: Style::Solid(Color::WHITE),
                width: 4.0,
                line_cap: LineCap::Square,
                line_join: LineJoin::default(),
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
            };
            frame.stroke(&selection, edge_stroke);
        } else {
            frame.fill_rectangle(Point::ORIGIN, bounds.size(), overlay);
        }
        vec![frame.into_geometry()]
    }

    fn mouse_interaction(
        &self,
        _state: &Self::State,
        _bounds: Rectangle,
        _cursor: Cursor,
    ) -> Interaction {
        Interaction::Crosshair
    }
}
