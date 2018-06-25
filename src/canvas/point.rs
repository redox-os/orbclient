#[derive(Copy, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Point {
            x: x,
            y: y,
        }
    }

    pub fn abs2(&self) -> f32 {
        self.x.powi(2) + self.y.powi(2)
    }

    pub fn abs(&self) -> f32 {
        self.abs2().sqrt()
    }

    pub fn arg(&self) -> f32 {
        self.y.atan2(self.x)
    }

    pub fn vector(a: &Point, b: &Point) -> Point {
        Point { x: b.x - a.x, y: b.y - a.y }
    }

    pub fn cross_product(a: &Point, b: &Point) -> f32 {
        a.x * b.y - a.y * b.x
    }
}
