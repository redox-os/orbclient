use core::cmp::{max, min};
use core::convert::TryInto;

#[derive(Copy, Clone, Debug, Default)]
pub struct Rect {
    x: i32,
    y: i32,
    w: u32,
    h: u32,
}

impl Rect {
    pub fn new(x: i32, y: i32, w: u32, h: u32) -> Rect {
        Rect { x, y, w, h }
    }

    pub fn area(&self) -> u32 {
        self.w * self.h
    }

    pub fn left(&self) -> i32 {
        self.x
    }

    pub fn right(&self) -> i32 {
        self.left_offset(self.w)
    }

    pub fn top(&self) -> i32 {
        self.y
    }

    pub fn bottom(&self) -> i32 {
        self.top_offset(self.h)
    }

    pub fn width(&self) -> u32 {
        self.w
    }

    pub fn height(&self) -> u32 {
        self.h
    }

    pub fn left_offset(&self, xoffset: u32) -> i32 {
        self.x + xoffset.try_into().unwrap_or(0)
    }

    pub fn top_offset(&self, yoffset: u32) -> i32 {
        self.y + yoffset.try_into().unwrap_or(0)
    }

    pub fn container(&self, other: &Rect) -> Rect {
        let left = min(self.left(), other.left());
        let right = max(self.right(), other.right());
        let top = min(self.top(), other.top());
        let bottom = max(self.bottom(), other.bottom());
        let width = (right - left).try_into().unwrap_or(0);
        let height = (bottom - top).try_into().unwrap_or(0);

        Rect::new(left, top, width, height)
    }

    pub fn contains(&self, x: i32, y: i32) -> bool {
        self.left() <= x && self.right() >= x && self.top() <= y && self.bottom() >= y
    }

    pub fn is_empty(&self) -> bool {
        self.w == 0 || self.h == 0
    }

    pub fn intersection(&self, other: &Rect) -> Rect {
        let left = max(self.left(), other.left());
        let right = min(self.right(), other.right());
        let top = max(self.top(), other.top());
        let bottom = min(self.bottom(), other.bottom());
        let width = (right - left).try_into().unwrap_or(0);
        let height = (bottom - top).try_into().unwrap_or(0);

        Rect::new(left, top, width, height)
    }

    pub fn offset(&self, x: i32, y: i32) -> Rect {
        Rect::new(self.x + x, self.y + y, self.w, self.h)
    }
}
