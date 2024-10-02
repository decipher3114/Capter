use iced::Point;

use crate::entities::capture::Endpoints;

impl Endpoints {
    pub fn normalize(self) -> (Point, Point) {
        let (initial_pt, final_pt) = (self.initial_pt.clone(), self.final_pt.clone());  
        let (mut start, mut end) = (initial_pt, final_pt);
        if initial_pt.x > final_pt.x {
            (start.x, end.x) = (final_pt.x, initial_pt.x)
        };
        if initial_pt.y > final_pt.y {
            (start.y, end.y) = (final_pt.y, initial_pt.y)
        };

        (start, end)
    }
}
