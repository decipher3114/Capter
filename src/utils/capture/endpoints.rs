use crate::entities::capture::Endpoints;

impl Endpoints {
    pub fn clear(&mut self) {
        self.initial_pt = None;
        self.final_pt = None;
    }
}
