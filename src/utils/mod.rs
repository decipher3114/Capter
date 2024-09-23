use std::env::var_os;

use iced::Point;

pub mod capture;
pub mod config;
pub mod ipc;
pub mod key_listener;
pub mod tray_icon;

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

pub fn shorten_path(path: String) -> String {

    #[cfg(target_os = "windows")]
    let home_path = format!(
            "{}{}",
            var_os("HOMEDRIVE").unwrap().to_string_lossy(),
            var_os("HOMEPATH").unwrap().to_string_lossy()
        );

    #[cfg(not(target_os = "windows"))]
    let home_path = format!("{}", var_os("HOME").unwrap().to_string_lossy());

    let replaced_path = path.replace(&home_path, "~");

    if replaced_path.len() > 20 {
        format!("...{}", &replaced_path[replaced_path.len() - 17..])
    } else {
        replaced_path
    }
}
