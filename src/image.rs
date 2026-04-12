use crate::rect::Rect;
use crate::{Color, Mode, Renderer};
use core::cell::Cell;
use core::{cmp, mem, ptr};

pub struct ImageRoiRows<'a> {
    rect: Rect,
    w: usize,
    data: &'a [Color],
    i: usize,
}

impl<'a> Iterator for ImageRoiRows<'a> {
    type Item = &'a [Color];
    fn next(&mut self) -> Option<Self::Item> {
        if self.i < self.rect.height() as usize {
            let start = (self.rect.top() as usize + self.i) * self.w + self.rect.left() as usize;
            let end = start + self.rect.width() as usize;
            self.i += 1;
            Some(&self.data[start..end])
        } else {
            None
        }
    }
}

pub struct ImageRoiRowsMut<'a> {
    rect: Rect,
    w: usize,
    data: &'a mut [Color],
    i: usize,
}

impl<'a> Iterator for ImageRoiRowsMut<'a> {
    type Item = &'a mut [Color];
    fn next(&mut self) -> Option<Self::Item> {
        if self.i < self.rect.height() as usize {
            let mut data = mem::take(&mut self.data);

            // skip section of data above top of rect
            if self.i == 0 {
                data = data.split_at_mut(self.rect.top() as usize * self.w).1
            };

            // split after next row
            let (row, tail) = data.split_at_mut(self.w);
            self.data = tail; // make data point to the remaining rows
            let start = self.rect.left() as usize;
            let end = self.rect.right() as usize;
            self.i += 1;
            Some(&mut row[start..end]) // return the rect part of the row
        } else {
            None
        }
    }
}

// ImageRoi seems to be a "window" onto an image, i.e. a Rectangular part of an image.
// `rect` defined the area within the larger image, we need to know the width of the image (`w`)
// to move through the data by rows, and `data` is a reference to the data in the actual image
pub struct ImageRoi<'a> {
    rect: Rect,
    w: usize,
    data: &'a [Color],
}

impl<'a> ImageRoi<'a> {
    pub fn rows(&'a self) -> ImageRoiRows<'a> {
        ImageRoiRows {
            rect: self.rect,
            w: self.w,
            data: self.data,
            i: 0,
        }
    }
}

// ImageRoiMut seems to be a "window" onto an image, i.e. a Rectangular part of an image.
// `rect` defined the area within the larger image, we need to know the width of the image (`w`)
// to move through the data by rows, and `data` is a reference to the data in the actual image
pub struct ImageRoiMut<'a> {
    pub rect: Rect,
    pub w: usize,
    pub data: &'a mut [Color],
}

impl<'a> ImageRoiMut<'a> {
    pub fn rows(&'a self) -> ImageRoiRows<'a> {
        ImageRoiRows {
            rect: self.rect,
            w: self.w,
            data: self.data,
            i: 0,
        }
    }

    pub fn rows_mut(&'a mut self) -> ImageRoiRowsMut<'a> {
        ImageRoiRowsMut {
            rect: self.rect,
            w: self.w,
            data: self.data,
            i: 0,
        }
    }

    pub fn blend(&'a mut self, other: &ImageRoi) {
        for (self_row, other_row) in self.rows_mut().zip(other.rows()) {
            for (old, new) in self_row.iter_mut().zip(other_row.iter()) {
                let alpha = (new.data >> 24) & 0xFF;
                if alpha >= 255 {
                    old.data = new.data;
                } else if alpha > 0 {
                    let n_r = (((new.data >> 16) & 0xFF) * alpha) >> 8;
                    let n_g = (((new.data >> 8) & 0xFF) * alpha) >> 8;
                    let n_b = ((new.data & 0xFF) * alpha) >> 8;

                    let n_alpha = 255 - alpha;

                    let o_r = (((old.data >> 16) & 0xFF) * n_alpha) >> 8;
                    let o_g = (((old.data >> 8) & 0xFF) * n_alpha) >> 8;
                    let o_b = ((old.data & 0xFF) * n_alpha) >> 8;

                    old.data = ((o_r << 16) | (o_g << 8) | o_b) + ((n_r << 16) | (n_g << 8) | n_b);
                }
            }
        }
    }

    pub fn blit(&'a mut self, other: &ImageRoi) {
        for (self_row, other_row) in self.rows_mut().zip(other.rows()) {
            let len = cmp::min(self_row.len(), other_row.len());
            unsafe {
                ptr::copy(other_row.as_ptr(), self_row.as_mut_ptr(), len);
            }
        }
    }
}

pub struct ImageRef<'a> {
    w: u32,
    h: u32,
    data: &'a mut [Color],
    mode: Cell<Mode>,
}

impl<'a> ImageRef<'a> {
    pub fn from_data(w: u32, h: u32, data: &'a mut [Color]) -> Self {
        ImageRef {
            w,
            h,
            data,
            mode: Cell::new(Mode::Blend),
        }
    }

    pub fn roi(&self, rect: &Rect) -> ImageRoi<'_> {
        ImageRoi {
            rect: *rect,
            w: self.w as usize,
            data: self.data,
        }
    }

    pub fn roi_mut(self, rect: &Rect) -> ImageRoiMut<'a> {
        ImageRoiMut {
            rect: *rect,
            w: self.w as usize,
            data: self.data,
        }
    }
}

impl<'a> Renderer for ImageRef<'a> {
    /// Get the width of the image in pixels
    fn width(&self) -> u32 {
        self.w as u32
    }

    /// Get the height of the image in pixels
    fn height(&self) -> u32 {
        self.h as u32
    }

    /// Return a reference to a slice of colors making up the image
    fn data(&self) -> &[Color] {
        self.data
    }

    /// Return a mutable reference to a slice of colors making up the image
    fn data_mut(&mut self) -> &mut [Color] {
        self.data
    }

    fn mode(&self) -> &Cell<Mode> {
        &self.mode
    }

    fn sync(&mut self) -> bool {
        true
    }

    fn update(&mut self) -> bool {
        true
    }

    fn update_rects(&mut self, _rects: &[(i32, i32, u32, u32)]) -> bool {
        true
    }
}

#[derive(Clone)]
pub struct Image {
    w: u32,
    h: u32,
    data: Box<[Color]>,
    mode: Cell<Mode>,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Image {
        Image::from_color(width, height, Color::rgb(0, 0, 0))
    }

    pub fn from_color(width: u32, height: u32, color: Color) -> Image {
        Image::from_data(
            width,
            height,
            vec![color; width as usize * height as usize].into_boxed_slice(),
        )
    }

    pub fn from_data(w: u32, h: u32, data: Box<[Color]>) -> Image {
        Image {
            w,
            h,
            data,
            mode: Cell::new(Mode::Blend),
        }
    }

    pub fn width(&self) -> u32 {
        self.w
    }

    pub fn height(&self) -> u32 {
        self.h
    }

    pub fn roi(&self, rect: &Rect) -> ImageRoi<'_> {
        ImageRoi {
            rect: *rect,
            w: self.w as usize,
            data: &self.data,
        }
    }

    pub fn roi_mut(&mut self, rect: &Rect) -> ImageRoiMut<'_> {
        ImageRoiMut {
            rect: *rect,
            w: self.w as usize,
            data: &mut self.data,
        }
    }
}

impl Renderer for Image {
    /// Get the width of the image in pixels
    fn width(&self) -> u32 {
        self.w as u32
    }

    /// Get the height of the image in pixels
    fn height(&self) -> u32 {
        self.h as u32
    }

    /// Return a reference to a slice of colors making up the image
    fn data(&self) -> &[Color] {
        &self.data
    }

    /// Return a mutable reference to a slice of colors making up the image
    fn data_mut(&mut self) -> &mut [Color] {
        &mut self.data
    }

    fn mode(&self) -> &Cell<Mode> {
        &self.mode
    }

    fn sync(&mut self) -> bool {
        true
    }

    fn update(&mut self) -> bool {
        true
    }

    fn update_rects(&mut self, _rects: &[(i32, i32, u32, u32)]) -> bool {
        true
    }
}

#[cfg(target_os = "redox")]
pub struct ImageAligned {
    w: i32,
    h: i32,
    data: &'static mut [Color],
}

#[cfg(target_os = "redox")]
impl Drop for ImageAligned {
    fn drop(&mut self) {
        unsafe {
            libc::free(self.data.as_mut_ptr() as *mut libc::c_void);
        }
    }
}

#[cfg(target_os = "redox")]
impl ImageAligned {
    pub fn new(w: i32, h: i32, align: usize) -> ImageAligned {
        let size = (w * h) as usize;
        let size_bytes = size * mem::size_of::<Color>();
        let size_alignments = (size_bytes + align - 1) / align;
        let size_aligned = size_alignments * align;
        let data;
        unsafe {
            let ptr = libc::memalign(align, size_aligned);
            libc::memset(ptr, 0, size_aligned);
            data = slice::from_raw_parts_mut(
                ptr as *mut Color,
                size_aligned / mem::size_of::<Color>(),
            );
        }
        ImageAligned { w, h, data }
    }

    pub fn width(&self) -> i32 {
        self.w
    }

    pub fn height(&self) -> i32 {
        self.h
    }

    pub fn roi(&self, rect: &Rect) -> ImageRoi<'_> {
        ImageRoi {
            rect: *rect,
            w: self.w,
            data: self.data,
        }
    }

    pub fn roi_mut(&mut self, rect: &Rect) -> ImageRoiMut<'_> {
        ImageRoiMut {
            rect: *rect,
            w: self.w,
            data: self.data,
        }
    }

    pub fn data_mut(&mut self) -> &mut [Color] {
        &mut self.data
    }
}
