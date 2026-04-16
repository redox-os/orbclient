use crate::rect::Rect;
use crate::{Color, Mode, Renderer};
use core::cell::Cell;
use core::num::NonZero;
use core::{cmp, mem, ptr};

#[cfg(not(feature = "std"))]
use alloc::{boxed::Box, vec};
#[cfg(feature = "image")]
pub use image::ImageError;
#[cfg(feature = "image")]
pub use image::ImageFormat;
#[cfg(feature = "image")]
pub use resize::Type as ResizeType;
#[cfg(feature = "image")]
use std::path::Path;

pub struct ImageRoiRows<'a> {
    height: usize,
    top: usize,
    left: usize,
    width: usize,
    stride: usize,
    data: &'a [Color],
    i: usize,
}

impl<'a> Iterator for ImageRoiRows<'a> {
    type Item = &'a [Color];
    fn next(&mut self) -> Option<Self::Item> {
        if self.i < self.height {
            let start = (self.top + self.i) * self.stride + self.left;
            let end = start + self.width;
            self.i += 1;
            Some(&self.data[start..end])
        } else {
            None
        }
    }
}

pub struct ImageRoiRowsMut<'a> {
    height: usize,
    top: usize,
    left: usize,
    width: usize,
    stride: usize,
    data: &'a mut [Color],
    i: usize,
}

impl<'a> Iterator for ImageRoiRowsMut<'a> {
    type Item = &'a mut [Color];
    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.height {
            return None;
        }
        let data = mem::take(&mut self.data);

        // skip section of data above top of rect
        let data = if self.i == 0 {
            let skip = self.top * self.stride;
            &mut data[skip..]
        } else {
            data
        };

        // split after next row
        let (row, tail) = data.split_at_mut(self.stride);
        self.data = tail; // make data point to the remaining rows
        let start = self.left;
        let end = self.left + self.width;
        self.i += 1;
        Some(&mut row[start..end]) // return the rect part of the row
    }
}

// ImageRoi seems to be a "window" onto an image, i.e. a Rectangular part of an image.
// `rect` defined the area within the larger image, we need to know the width of the image (`w`)
// to move through the data by rows, and `data` is a reference to the data in the actual image
pub struct ImageRoi<'a> {
    height: usize,
    top: usize,
    left: usize,
    width: usize,
    stride: usize,
    data: &'a [Color],
}

impl<'a> ImageRoi<'a> {
    pub fn rows(&'a self) -> ImageRoiRows<'a> {
        ImageRoiRows {
            height: self.height,
            top: self.top,
            left: self.left,
            width: self.width,
            stride: self.stride,
            data: self.data,
            i: 0,
        }
    }

    pub fn cells(&self) -> impl Iterator<Item = &Color> {
        self.rows().flatten()
    }
}

// ImageRoiMut seems to be a "window" onto an image, i.e. a Rectangular part of an image.
// `rect` defined the area within the larger image, we need to know the width of the image (`w`)
// to move through the data by rows, and `data` is a reference to the data in the actual image
pub struct ImageRoiMut<'a> {
    height: usize,
    top: usize,
    left: usize,
    width: usize,
    stride: usize,
    data: &'a mut [Color],
}

impl<'a> ImageRoiMut<'a> {
    pub fn rows(&'a self) -> ImageRoiRows<'a> {
        ImageRoiRows {
            height: self.height,
            top: self.top,
            left: self.left,
            width: self.width,
            stride: self.stride,
            data: self.data,
            i: 0,
        }
    }

    pub fn rows_mut(&'a mut self) -> ImageRoiRowsMut<'a> {
        ImageRoiRowsMut {
            height: self.height,
            top: self.top,
            left: self.left,
            width: self.width,
            stride: self.stride,
            data: self.data,
            i: 0,
        }
    }

    /// Draw another image on top with alpha blending.
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

    /// Draw another image on top with alpha masking.
    pub fn blit_mask(&'a mut self, other: &ImageRoi) {
        for (self_row, other_row) in self.rows_mut().zip(other.rows()) {
            for (old, new) in self_row.iter_mut().zip(other_row.iter()) {
                if new.data >> 24 >= 128 {
                    old.data = new.data;
                }
            }
        }
    }

    /// Draw another image on top without alpha blending.
    pub fn blit(&'a mut self, other: &ImageRoi) {
        if other.stride == self.stride
            && self.left == 0
            && other.left == 0
            && self.width == self.stride
            && other.width == other.stride
        {
            // very fast blit path which will benefit fullscreen window
            unsafe {
                let len = cmp::min(self.width * self.height, other.width * other.height);
                let other_ptr = other.data.split_at(other.stride * other.top).1.as_ptr();
                let self_ptr = self
                    .data
                    .split_at_mut(other.stride * self.top)
                    .1
                    .as_mut_ptr();
                ptr::copy(other_ptr, self_ptr, len);
            }
        } else {
            for (self_row, other_row) in self.rows_mut().zip(other.rows()) {
                let len = cmp::min(self_row.len(), other_row.len());
                unsafe {
                    ptr::copy(other_row.as_ptr(), self_row.as_mut_ptr(), len);
                }
            }
        }
    }
}

/// A structure to borrow an existing image in software memory.
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

    pub fn from_renderer(renderer: &'a mut impl Renderer) -> Self {
        let mode = renderer.mode().clone();
        ImageRef {
            w: renderer.width(),
            h: renderer.height(),
            data: renderer.data_mut(),
            mode,
        }
    }

    pub fn roi(&self, rect: &Rect) -> ImageRoi<'_> {
        ImageRoi {
            width: rect.width() as usize,
            height: rect.height() as usize,
            left: rect.left() as usize,
            top: rect.top() as usize,
            stride: self.w as usize,
            data: self.data,
        }
    }

    pub fn roi_mut(self, rect: &Rect) -> ImageRoiMut<'a> {
        ImageRoiMut {
            width: rect.width() as usize,
            height: rect.height() as usize,
            left: rect.left() as usize,
            top: rect.top() as usize,
            stride: self.w as usize,
            data: self.data,
        }
    }
}

impl<'a> Renderer for ImageRef<'a> {
    /// Get the width of the image in pixels
    fn width(&self) -> u32 {
        self.w
    }

    /// Get the height of the image in pixels
    fn height(&self) -> u32 {
        self.h
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

/// A structure to hold an image in owned software memory.
/// If `image` feature enabled, it allows loading from file and more dynamic resize algorithm.
#[derive(Clone)]
pub struct Image {
    w: u32,
    h: u32,
    data: Box<[Color]>,
    mode: Cell<Mode>,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Self {
        Self::from_color(width, height, Color::rgb(0, 0, 0))
    }

    pub fn from_color(width: u32, height: u32, color: Color) -> Self {
        Self::from_data_unchecked(
            width,
            height,
            vec![color; width as usize * height as usize].into_boxed_slice(),
        )
    }

    pub fn from_data(w: u32, h: u32, data: Box<[Color]>) -> Option<Self> {
        if (w as usize * h as usize) != data.len() {
            return None;
        }
        Some(Self::from_data_unchecked(w, h, data))
    }

    fn from_data_unchecked(w: u32, h: u32, data: Box<[Color]>) -> Self {
        Self {
            w,
            h,
            data,
            mode: Cell::new(Mode::Blend),
        }
    }

    #[cfg(feature = "image")]
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, ImageError> {
        let img = image::open(path);
        Self::from_dynamic_image(img)
    }

    #[cfg(feature = "image")]
    pub fn from_memory_with_format(data: &[u8], format: ImageFormat) -> Result<Self, ImageError> {
        let img = image::load_from_memory_with_format(data, format);
        Self::from_dynamic_image(img)
    }

    #[cfg(feature = "image")]
    fn from_dynamic_image(
        d_img: image::ImageResult<image::DynamicImage>,
    ) -> Result<Self, ImageError> {
        let img = d_img?.to_rgba();
        let data: Vec<_> = img
            .pixels()
            .map(|p| Color::rgba(p.data[0], p.data[1], p.data[2], p.data[3]))
            .collect();
        Self::from_data(img.width(), img.height(), data.into_boxed_slice())
            .ok_or(ImageError::DimensionError)
    }

    #[cfg(feature = "image")]
    pub fn resize(&self, w: u32, h: u32, resize_type: ResizeType) -> Self {
        let mut dst_color = vec![Color { data: 0 }; w as usize * h as usize].into_boxed_slice();

        let src = unsafe {
            core::slice::from_raw_parts(self.data.as_ptr() as *const u8, self.data.len() * 4)
        };

        let mut dst = unsafe {
            core::slice::from_raw_parts_mut(dst_color.as_mut_ptr() as *mut u8, dst_color.len() * 4)
        };

        let mut resizer = resize::new(
            self.w as usize,
            self.h as usize,
            w as usize,
            h as usize,
            resize::Pixel::RGBA,
            resize_type,
        );
        resizer.resize(&src, &mut dst);

        Self::from_data_unchecked(w, h, dst_color)
    }

    pub fn resize_exact(&self, scale: NonZero<u32>) -> Self {
        let scale = scale.get();
        if scale == 1 {
            return Self::from_data_unchecked(self.w, self.h, self.data.clone());
        }
        let uscale = scale as usize;
        let mut new_data =
            vec![Color::rgb(0, 0, 0); self.data.len() * (uscale * uscale)].into_boxed_slice();
        let w = self.w as usize;
        for y in 0..self.h as usize {
            for x in 0..w {
                let i = y * w + x;
                let value = self.data[i].data;
                for y_s in 0..uscale {
                    for x_s in 0..uscale {
                        let new_i = (y * uscale + y_s) * w * uscale + x * uscale + x_s;
                        new_data[new_i].data = value;
                    }
                }
            }
        }

        Self::from_data_unchecked(self.w * scale, self.h * scale, new_data)
    }

    pub fn width(&self) -> u32 {
        self.w
    }

    pub fn height(&self) -> u32 {
        self.h
    }

    /// Read a specified rect of the image
    pub fn roi(&self, rect: &Rect) -> ImageRoi<'_> {
        ImageRoi {
            width: rect.width() as usize,
            height: rect.height() as usize,
            left: rect.left() as usize,
            top: rect.top() as usize,
            stride: self.w as usize,
            data: &self.data,
        }
    }

    /// Read or write a specified rect of the image
    pub fn roi_mut(&mut self, rect: &Rect) -> ImageRoiMut<'_> {
        ImageRoiMut {
            width: rect.width() as usize,
            height: rect.height() as usize,
            left: rect.left() as usize,
            top: rect.top() as usize,
            stride: self.w as usize,
            data: &mut self.data,
        }
    }
}

impl Renderer for Image {
    /// Get the width of the image in pixels
    fn width(&self) -> u32 {
        self.w
    }

    /// Get the height of the image in pixels
    fn height(&self) -> u32 {
        self.h
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
    w: u32,
    h: u32,
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
    pub fn new(w: u32, h: u32, align: usize) -> ImageAligned {
        let size = (w * h) as usize;
        let size_bytes = size * mem::size_of::<Color>();
        let size_alignments = (size_bytes + align - 1) / align;
        let size_aligned = size_alignments * align;
        let data;
        unsafe {
            let ptr = libc::memalign(align, size_aligned);
            libc::memset(ptr, 0, size_aligned);
            data = core::slice::from_raw_parts_mut(
                ptr as *mut Color,
                size_aligned / mem::size_of::<Color>(),
            );
        }
        ImageAligned { w, h, data }
    }

    pub fn width(&self) -> u32 {
        self.w
    }

    pub fn height(&self) -> u32 {
        self.h
    }

    pub fn roi(&self, rect: &Rect) -> ImageRoi<'_> {
        ImageRoi {
            width: rect.width() as usize,
            height: rect.height() as usize,
            left: rect.left() as usize,
            top: rect.top() as usize,
            stride: self.w as usize,
            data: self.data,
        }
    }

    pub fn roi_mut(&mut self, rect: &Rect) -> ImageRoiMut<'_> {
        ImageRoiMut {
            width: rect.width() as usize,
            height: rect.height() as usize,
            left: rect.left() as usize,
            top: rect.top() as usize,
            stride: self.w as usize,
            data: self.data,
        }
    }

    pub fn data_mut(&mut self) -> &mut [Color] {
        &mut self.data
    }
}
