extern crate syscall;

use std::cmp;
use std::fs::File;
use std::io::*;
use std::mem;
use std::os::unix::io::{AsRawFd, RawFd};
use std::slice;
use std::thread;

#[cfg(target_arch = "x86")]
#[inline(always)]
#[cold]
pub unsafe fn fast_set32(dst: *mut u32, src: u32, len: usize) {
    asm!("cld
        rep stosd"
        :
        : "{edi}"(dst as usize), "{eax}"(src), "{ecx}"(len)
        : "cc", "memory", "edi", "ecx"
        : "intel", "volatile");
}

#[cfg(target_arch = "x86_64")]
#[inline(always)]
#[cold]
pub unsafe fn fast_set32(dst: *mut u32, src: u32, len: usize) {
    asm!("cld
        rep stosd"
        :
        : "{rdi}"(dst as usize), "{eax}"(src), "{rcx}"(len)
        : "cc", "memory", "rdi", "rcx"
        : "intel", "volatile");
}

use super::super::FONT;
use super::super::Event;
use super::super::Color;

/// A window
pub struct Window {
    /// The x coordinate of the window
    x: i32,
    /// The y coordinate of the window
    y: i32,
    /// The width of the window
    w: u32,
    /// The height of the window
    h: u32,
    /// The title of the window
    t: String,
    /// True if the window should not wait for events
    async: bool,
    /// The input scheme
    file: File,
    /// Window data
    data: &'static mut [Color],
}

impl Window {
    /// Create a new window
    pub fn new(x: i32, y: i32, w: u32, h: u32, title: &str) -> Option<Self> {
        Window::new_flags(x, y, w, h, title, false)
    }

    /// Create a new window with flags
    pub fn new_flags(x: i32, y: i32, w: u32, h: u32, title: &str, async: bool) -> Option<Self> {
        if let Ok(file) = File::open(&format!("orbital:{}/{}/{}/{}/{}/{}", if async { "a" } else { "" }, x, y, w, h, title)) {
            if let Ok(address) = unsafe { syscall::fmap(file.as_raw_fd(), 0, (w * h * 4) as usize) } {
                Some(Window {
                    x: x,
                    y: y,
                    w: w,
                    h: h,
                    t: title.to_string(),
                    async: async,
                    file: file,
                    data: unsafe { slice::from_raw_parts_mut(address as *mut Color, (w * h) as usize) },
                })
            } else {
                None
            }
        } else {
            None
        }
    }

    // TODO: Replace with smarter mechanism, maybe a move event?
    pub fn sync_path(&mut self) {
        if let Ok(path) = self.file.path() {
            // orbital:/x/y/w/h/t
            if let Some(path_str) = path.to_str() {
                let mut parts = path_str.split('/').skip(1);
                if let Some(x) = parts.next() {
                    self.x = x.parse::<i32>().unwrap_or(0);
                }
                if let Some(y) = parts.next() {
                    self.y = y.parse::<i32>().unwrap_or(0);
                }
                if let Some(w) = parts.next() {
                    self.w = w.parse::<u32>().unwrap_or(0);
                }
                if let Some(h) = parts.next() {
                    self.h = h.parse::<u32>().unwrap_or(0);
                }
            }
        }
    }

    /// Get x
    // TODO: Sync with window movements
    pub fn x(&self) -> i32 {
        self.x
    }

    /// Get y
    // TODO: Sync with window movements
    pub fn y(&self) -> i32 {
        self.y
    }

    /// Get width
    pub fn width(&self) -> u32 {
        self.w
    }

    /// Get height
    pub fn height(&self) -> u32 {
        self.h
    }

    /// Get title
    pub fn title(&self) -> String {
        self.t.clone()
    }

    /// Set title
    pub fn set_title(&mut self, _: &str) {
        // TODO
    }

    pub fn data(&self) -> &[Color] {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut [Color] {
        &mut self.data
    }

    /// Draw a pixel
    pub fn pixel(&mut self, x: i32, y: i32, color: Color) {
        if x >= 0 && y >= 0 && x < self.w as i32 && y < self.h as i32 {
            let new = color.data;

            let alpha = (new >> 24) & 0xFF;
            if alpha > 0 {
                let old = &mut self.data[y as usize * self.w as usize + x as usize].data;
                if alpha >= 255 {
                    *old = new;
                } else {
                    let n_r = (((new >> 16) & 0xFF) * alpha) >> 8;
                    let n_g = (((new >> 8) & 0xFF) * alpha) >> 8;
                    let n_b = ((new & 0xFF) * alpha) >> 8;

                    let n_alpha = 255 - alpha;
                    let o_a = (((*old >> 24) & 0xFF) * n_alpha) >> 8;
                    let o_r = (((*old >> 16) & 0xFF) * n_alpha) >> 8;
                    let o_g = (((*old >> 8) & 0xFF) * n_alpha) >> 8;
                    let o_b = ((*old & 0xFF) * n_alpha) >> 8;

                    *old = ((o_a << 24) | (o_r << 16) | (o_g << 8) | o_b) + ((alpha << 24) | (n_r << 16) | (n_g << 8) | n_b);
                }
            }
        }
    }

    /// Draw a line
    pub fn line(&mut self, argx1: i32, argy1: i32, argx2: i32, argy2: i32, color: Color) {
        let mut x1 = argx1;
        let mut y1 = argy1;
        let mut x2 = argx2;
        let mut y2 = argy2;

        if x2 < x1 {
            x1 = argx2;
            y1 = argy2;
            x2 = argx1;
            y2 = argy1;
        }

        let dx = x2 - x1;
        let dy = y2 - y1;

        //let ratio = dy as f32 / dx as f32;
        for x in x1..x2 {
            let y = y1 + ((dy * (x - x1)) as f32 / dx as f32) as i32;
            self.pixel(x, y, color);
        }
    }

    pub fn lines(&mut self, points: &[[i32; 2]], color: Color) {
        if points.len() == 0 {
            // when no points given, do nothing
        } else if points.len() == 1 {
            self.pixel(points[0][0], points[0][1], color);
        } else {
            for i in 0..points.len() - 1 {
                self.line(points[i][0], points[i][1], points[i+1][0], points[i+1][1], color);
            }
        }
    }

    /// Draw a character, using the loaded font
    pub fn char(&mut self, x: i32, y: i32, c: char, color: Color) {
        let mut offset = (c as usize) * 16;
        for row in 0..16 {
            let row_data;
            if offset < FONT.len() {
                row_data = FONT[offset];
            } else {
                row_data = 0;
            }

            for col in 0..8 {
                let pixel = (row_data >> (7 - col)) & 1;
                if pixel > 0 {
                    self.pixel(x + col as i32, y + row as i32, color);
                }
            }
            offset += 1;
        }
    }

    // TODO move, resize, set_title

    /// Set entire window to a color
    // TODO: Improve speed
    #[allow(unused_variables)]
    pub fn set(&mut self, color: Color) {
        unsafe {
            fast_set32(self.data.as_mut_ptr() as *mut u32, color.data, self.data.len());
        }
    }

    /// Sets the whole window to black
    pub fn clear(&mut self) {
        self.set(Color::rgb(0,0,0));
    }

    /// Draw rectangle
    #[allow(unused_variables)]
    pub fn rect(&mut self, x: i32, y: i32, w: u32, h: u32, color: Color) {
        let data = color.data;

        let start_y = cmp::max(0, cmp::min(self.h as i32 - 1, y));
        let end_y = cmp::max(start_y, cmp::min(self.h as i32, y + h as i32));

        let start_x = cmp::max(0, cmp::min(self.w as i32 - 1, x));
        let len = cmp::max(start_x, cmp::min(self.w as i32, x + w as i32)) - start_x;

        for y in start_y..end_y {
            unsafe {
                fast_set32(self.data.as_mut_ptr().offset((y * self.w as i32 + start_x) as isize) as *mut u32, data, len as usize);
            }
        }
    }

    /// Display an image
    // TODO: Improve speed
    pub fn image(&mut self, start_x: i32, start_y: i32, w: u32, h: u32, data: &[Color]) {
        let mut i = 0;
        for y in start_y..start_y + h as i32 {
            for x in start_x..start_x + w as i32 {
                if i < data.len() {
                    self.pixel(x, y, data[i])
                }
                i += 1;
            }
        }
    }

    /// Blocking iterator over events
    pub fn events(&mut self) -> EventIter {
        let mut iter = EventIter {
            events: [Event::new(); 128],
            i: 0,
            count: 0,
        };

        'blocking: loop {
            //Should it be cleared? iter.events = [Event::new(); 128];
            match self.file.read(unsafe {
                slice::from_raw_parts_mut(iter.events.as_mut_ptr() as *mut u8, iter.events.len() * mem::size_of::<Event>())
            }){
                Ok(0) => if ! self.async {
                    thread::yield_now();
                } else {
                    break 'blocking;
                },
                Ok(count) => {
                    iter.count = count/mem::size_of::<Event>();
                    break 'blocking;
                },
                Err(_) => break 'blocking,
            }
        }

        iter
    }

    /// Flip the window buffer
    pub fn sync(&mut self) -> bool {
        self.file.sync_data().is_ok()
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        let _ = unsafe { syscall::funmap(self.data.as_ptr() as usize) };
    }
}

impl AsRawFd for Window {
    fn as_raw_fd(&self) -> RawFd {
        self.file.as_raw_fd()
    }
}

/// Event iterator
pub struct EventIter {
    events: [Event; 128],
    i: usize,
    count: usize,
}

impl Iterator for EventIter {
    type Item = Event;
    fn next(&mut self) -> Option<Event> {
        if self.i < self.count {
            if let Some(event) = self.events.get(self.i) {
                self.i += 1;
                Some(*event)
            } else {
                None
            }
        } else {
            None
        }
    }
}
