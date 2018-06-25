use point::Point;

#[derive(Copy, Clone)]
pub enum EdgeType {
    Visible,
    Hidden,
}

#[derive(Copy, Clone)]
pub struct Edge {
    pub start: Point,
    pub end: Point,
}

impl Edge {
    pub fn new(start: Point, end: Point) -> Self {
        Edge {
            start: start,
            end: end,
        }
    }
}
