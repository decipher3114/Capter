use iced::Point;

pub mod capture;
pub mod config;
pub mod key_listener;

pub fn evaluate_points(point_a: Point, point_b: Point) -> (Point, Point) {
    let (mut start, mut end) = (point_a, point_b);
    if point_a.x > point_b.x {
        (start.x, end.x) = (point_b.x, point_a.x)
    };
    if point_a.y > point_b.y {
        (start.y, end.y) = (point_b.y, point_a.y)
    };

    (start, end)
}
