use crate::components::GridPosition;

#[derive(Clone, Copy, Debug)]
pub struct Segment {
    pub pos: GridPosition,
}

impl Segment {
    pub fn new(pos: GridPosition) -> Self {
        Segment { pos }
    }
}