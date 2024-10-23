use std::f32::consts::PI;

use iced::Point;

pub fn resolve_arrow_points(initial_pt: Point, final_pt: Point) -> (Point, Point) {
    let line = final_pt - initial_pt;
    let length = f32::sqrt(line.x.powf(2.0) + line.y.powf(2.0));
    let size = if length < 60.0 { length / 2.0 } else { 30.0 };
    let rad = line.y.atan2(line.x);
    let right_pt = Point::new(
        final_pt.x - size * (rad - PI / 5.0).cos(),
        final_pt.y - size * (rad - PI / 5.0).sin(),
    );
    let left_pt = Point::new(
        final_pt.x - size * (rad + PI / 5.0).cos(),
        final_pt.y - size * (rad + PI / 5.0).sin(),
    );
    (right_pt , left_pt)
}