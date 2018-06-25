use color::Color;
use matrix::Matrix;

#[derive(Copy, Clone)]
pub struct CanvasPaintState {
    pub fill_style: Color,
    pub stroke_style: Color,
    pub line_width: f32,
    pub transform: Matrix,
}

impl CanvasPaintState {
    pub fn new() -> Self {
        CanvasPaintState {
            fill_style: Color::rgba(0,0,0,0),
            stroke_style: Color::rgba(0,0,0,0),
            line_width: 1.0,
            transform: Matrix::new(),
        }
    }
}