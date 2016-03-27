use std::fs::File;
use std::io::*;
use std::mem;
use std::slice;
use std::thread;

use super::FONT;
use super::Event;
use super::Color;

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
    data: Box<[Color]>,
}

impl Window {
    /// Create a new window
    pub fn new(x: i32, y: i32, w: u32, h: u32, title: &str) -> Option<Box<Self>> {
        Window::new_flags(x, y, w, h, title, false)
    }

    /// Create a new window with flags
    pub fn new_flags(x: i32, y: i32, w: u32, h: u32, title: &str, async: bool) -> Option<Box<Self>> {
        match File::open(&format!("orbital:{}/{}/{}/{}/{}/{}", if async { "a" } else { "" }, x, y, w, h, title)) {
            Ok(file) => {
                Some(box Window {
                    x: x,
                    y: y,
                    w: w,
                    h: h,
                    t: title.to_string(),
                    async: async,
                    file: file,
                    data: vec![Color::rgb(0, 0, 0); (w * h * 4) as usize].into_boxed_slice(),
                })
            }
            Err(_) => None,
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
    pub fn line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, color: Color) {
        let triangle_x = x2 - x1;
        let triangle_y = y2 - y1;

        if triangle_x > triangle_y {
            let ratio = triangle_y as f32 / triangle_x as f32;
            for pixel in x1..x2 {
                self.pixel(pixel, y1 + (ratio * pixel as f32 - x1 as f32) as i32, color);
            }
        } else if triangle_y >= triangle_x && triangle_y != 0 {
            let ratio = triangle_x as f32 / triangle_y as f32;
            for pixel in y1..y2 {
                self.pixel(x1 + (ratio * pixel as f32 - y1 as f32) as i32, pixel, color);
            }
        } else if triangle_x == 0 && triangle_y == 0 {
            self.pixel(x1, y1, color);
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
        for mut d in self.data.iter_mut() {
            *d = color;
        }
    }

    /// Sets the whole window to black
    pub fn clear(&mut self) {
        self.set(Color::rgb(0,0,0));
    }

    // Allows to draw a left border in the window
    // x_limit is the left limit of the border!!!
    pub fn set_border_left(&mut self, x_limit: i32, color: Color, density: i32) {
        for y in 0..self.h as i32 {
            for x in x_limit..(x_limit + density) {
                self.pixel(x, y, color);
            }
        }
    }

    // Allows to draw a right border in the window
    // x_limit is the left limit of the border!!!
    pub fn set_border_right(&mut self, x_limit: i32, color: Color, density: i32) {
        for y in 0..self.h as i32 {
            for x in x_limit..(x_limit + density) {
                let new_x : i32 = self.w as i32;
                self.pixel((new_x - x), y, color);
            }
        }
    }

    // Allows to draw a top border in the window
    // x_limit is the top limit of the border!!!
    pub fn set_border_top(&mut self, y_limit: i32, color: Color, density: i32) {
        for x in 0..self.w as i32 {
            for y in y_limit..(y_limit + density) {
                self.pixel(x, y, color);
            }
        }
    }

    // Allows to draw a bottom border in the window
    // x_limit is the top limit of the border!!!
    pub fn set_border_bottom(&mut self, y_limit: i32, color: Color, density: i32) {
        for x in 0..self.x {
            for y in 0..(y_limit + density) {
                let new_y : i32 = self.h as i32;
                self.pixel(x, (new_y - y), color);
            }
        }
    }

    /// Draw rectangle
    // TODO: Improve speed
    #[allow(unused_variables)]
    pub fn rect(&mut self, start_x: i32, start_y: i32, w: u32, h: u32, color: Color) {
        for y in start_y..start_y + h as i32 {
            for x in start_x..start_x + w as i32 {
                self.pixel(x, y, color);
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
        self.file.write(unsafe {
            slice::from_raw_parts(self.data.as_ptr() as *const u8, self.data.len() * mem::size_of::<Color>())
        }).is_ok()
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
