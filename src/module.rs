use crate::segment::Segment;

pub struct Module {
    segments: Vec<Segment>
}

impl Module {
    pub fn new(name: String) {
        Module {
            segments: Vec::new(),
        }
    }

    pub fn add(&self, segment: Segment) {
        self.segments.append(segment);
    }
}
