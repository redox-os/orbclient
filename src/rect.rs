use core::cmp::{max, min};
use core::convert::TryInto;

/// Utilies for working with Image type.
/// As with other APIs, the (0, 0) position is left top of the screen, then it grows downward and rightward.
#[derive(Copy, Clone, Debug, Default)]
pub struct Rect {
    x: i32,
    y: i32,
    w: u32,
    h: u32,
}

pub enum RectAlignment {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

pub enum RectEdge {
    Top,
    Left,
    Right,
    Bottom,
}

impl Rect {
    pub fn new(x: i32, y: i32, w: u32, h: u32) -> Rect {
        Rect { x, y, w, h }
    }

    /// Return (width * height) as usize, which you can use it as a buffer length in an array.
    pub fn area(&self) -> usize {
        self.w as usize * self.h as usize
    }

    pub fn left(&self) -> i32 {
        self.x
    }

    pub fn right(&self) -> i32 {
        self.x + self.iwidth()
    }

    pub fn top(&self) -> i32 {
        self.y
    }

    pub fn bottom(&self) -> i32 {
        self.y + self.iheight()
    }

    pub fn width(&self) -> u32 {
        self.w
    }

    pub fn height(&self) -> u32 {
        self.h
    }

    /// A convenient type for width() with i32 for rect calculations.
    /// Consider to use other utilities functions if it suitable.
    pub fn iwidth(&self) -> i32 {
        self.w.try_into().unwrap_or(0)
    }

    /// A convenient type for height() with i32 for rect calculations.
    /// Consider to use other utilities functions if it suitable.
    pub fn iheight(&self) -> i32 {
        self.h.try_into().unwrap_or(0)
    }

    /// Create a union between two Rectangles.
    /// If the two does not overlap, the returned Rect will be larger than two rectangle areas.
    pub fn container(&self, other: &Rect) -> Rect {
        let left = min(self.left(), other.left());
        let right = max(self.right(), other.right());
        let top = min(self.top(), other.top());
        let bottom = max(self.bottom(), other.bottom());
        let width = (right - left).try_into().unwrap_or(0);
        let height = (bottom - top).try_into().unwrap_or(0);

        Rect::new(left, top, width, height)
    }

    /// Check if a point is in or at the edge of Rectangle.
    /// To check if two Rectangle overlaps, use `!a.intersection(b).is_empty()`.
    pub fn contains(&self, x: i32, y: i32) -> bool {
        !self.is_empty()
            && self.left() <= x
            && self.right() >= x
            && self.top() <= y
            && self.bottom() >= y
    }

    /// Check if this rectangle is not empty.
    pub fn is_empty(&self) -> bool {
        self.w == 0 || self.h == 0
    }

    /// Create an intersection between two Rectangles.
    /// If the two does not overlap, the returned Rect will be empty.
    pub fn intersection(&self, other: &Rect) -> Rect {
        let left = max(self.left(), other.left());
        let right = min(self.right(), other.right());
        let top = max(self.top(), other.top());
        let bottom = min(self.bottom(), other.bottom());
        let width = (right - left).try_into().unwrap_or(0);
        let height = (bottom - top).try_into().unwrap_or(0);

        Rect::new(left, top, width, height)
    }

    /// Return a new Rect with new moved position
    pub fn translate(self, x: i32, y: i32) -> Rect {
        Rect::new(self.x + x, self.y + y, self.w, self.h)
    }

    /// Return a new Rect with new size. The position will be adjusted depending to alignment.
    /// If you want to leave the position unchanged, use RectAlignment::TopLeft
    pub fn resize(self, w: u32, h: u32, align: RectAlignment) -> Rect {
        let (x, y) = match align {
            RectAlignment::TopLeft => (self.left(), self.top()),
            RectAlignment::TopRight => (self.right().saturating_sub_unsigned(w), self.top()),
            RectAlignment::BottomLeft => (self.left(), self.bottom().saturating_sub_unsigned(h)),
            RectAlignment::BottomRight => (
                self.right().saturating_sub_unsigned(w),
                self.bottom().saturating_sub_unsigned(h),
            ),
        };
        Rect::new(x, y, w, h)
    }

    /// Return a new Rect with new width or height depending on alignment edge.
    /// The new width or height is calculated as (inset + outset),
    /// the difference between the two is whether you want the rectangle grows inward or outward.
    pub fn edge(self, inset: u32, outset: u32, align: RectEdge) -> Rect {
        let start = match align {
            RectEdge::Top => self.top().saturating_sub_unsigned(outset),
            RectEdge::Left => self.left().saturating_sub_unsigned(outset),
            RectEdge::Right => self.right().saturating_sub_unsigned(inset),
            RectEdge::Bottom => self.bottom().saturating_sub_unsigned(inset),
        };
        match align {
            RectEdge::Top | RectEdge::Bottom => Rect::new(self.x, start, self.w, inset + outset),
            RectEdge::Left | RectEdge::Right => Rect::new(start, self.y, inset + outset, self.h),
        }
    }
}
