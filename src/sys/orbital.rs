extern crate syscall;

use std::{env, mem, slice, thread};
use std::fs::File;
use std::io::{Read, Write};
use std::os::unix::io::{AsRawFd, RawFd};
use std::cell::Cell;

use color::Color;
use event::{Event, EVENT_RESIZE};
use renderer::Renderer;
use WindowFlag;
use Mode;

pub fn get_display_size() -> Result<(u32, u32), String> {
    let display_path = try!(env::var("DISPLAY").or(Err("DISPLAY not set")));
    match File::open(&display_path) {
        Ok(display) => {
            let mut buf: [u8; 4096] = [0; 4096];
            let count = syscall::fpath(display.as_raw_fd() as usize, &mut buf).map_err(|err| format!("{}", err))?;
            let path = unsafe { String::from_utf8_unchecked(Vec::from(&buf[..count])) };
            let res = path.split(":").nth(1).unwrap_or("");
            let width = res.split("/").nth(1).unwrap_or("").parse::<u32>().unwrap_or(0);
            let height = res.split("/").nth(2).unwrap_or("").parse::<u32>().unwrap_or(0);
            Ok((width, height))
        },
        Err(err) => Err(format!("{}", err))
    }
}

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
    /// True if the window can be resized
    resizable: bool,
    /// Drawing mode
    mode: Cell<Mode>,
    /// The input scheme
    file: File,
    /// Window data
    data: &'static mut [Color],
}

impl Renderer for Window {
    /// Get width
    fn width(&self) -> u32 {
        self.w
    }

    /// Get height
    fn height(&self) -> u32 {
        self.h
    }

    /// Access pixel buffer
    fn data(&self) -> &[Color] {
        &self.data
    }

    /// Access pixel buffer mutably
    fn data_mut(&mut self) -> &mut [Color] {
        &mut self.data
    }

    /// Flip the buffer
    fn sync(&mut self) -> bool {
        self.file.sync_data().is_ok()
    }

    /// Set/get mode
    fn mode(&self) -> &Cell<Mode> {
        &self.mode
    }
}

impl Window {
    /// Create a new window
    pub fn new(x: i32, y: i32, w: u32, h: u32, title: &str) -> Option<Self> {
        Window::new_flags(x, y, w, h, title, &[])
    }

    /// Create a new window with flags
    pub fn new_flags(x: i32, y: i32, w: u32, h: u32, title: &str, flags: &[WindowFlag]) -> Option<Self> {
        let mut flag_str = String::new();

        let mut async = false;
        let mut resizable = false;
        for &flag in flags.iter() {
            match flag {
                WindowFlag::Async => {
                    async = true;
                    flag_str.push('a');
                },
                WindowFlag::Back => flag_str.push('b'),
                WindowFlag::Front => flag_str.push('f'),
                WindowFlag::Resizable => {
                    resizable = true;
                    flag_str.push('r');
                },
                WindowFlag::Unclosable => flag_str.push('u'),
            }
        }

        if let Ok(file) = File::open(&format!(
            "orbital:{}/{}/{}/{}/{}/{}",
            flag_str, x, y, w, h, title
        )) {
            if let Ok(address) = unsafe { syscall::fmap(file.as_raw_fd(), 0, (w * h * 4) as usize) } {
                Some(Window {
                    x: x,
                    y: y,
                    w: w,
                    h: h,
                    t: title.to_string(),
                    async: async,
                    resizable: resizable,
                    mode: Cell::new(Mode::Blend),
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
        let mut buf: [u8; 4096] = [0; 4096];
        if let Ok(count) = syscall::fpath(self.file.as_raw_fd() as usize, &mut buf) {
            let path = unsafe { String::from_utf8_unchecked(Vec::from(&buf[..count])) };
            // orbital:/x/y/w/h/t
            let mut parts = path.split('/').skip(1);
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

    /// Get title
    pub fn title(&self) -> String {
        self.t.clone()
    }

    // Set position
    pub fn set_pos(&mut self, x: i32, y: i32) {
        let _ = self.file.write(&format!("P,{},{}", x, y).as_bytes());
        self.sync_path();
    }

    // Set size
    pub fn set_size(&mut self, width: u32, height: u32) {
        //TODO: Improve safety and reliability
        unsafe {
            syscall::funmap(self.data.as_ptr() as usize).expect("orbclient: failed to unmap memory in resize");
        }
        let _ = self.file.write(&format!("S,{},{}", width, height).as_bytes());
        self.sync_path();
        unsafe {
            let address = syscall::fmap(self.file.as_raw_fd(), 0, (self.w * self.h * 4) as usize).expect("orbclient: failed to map memory in resize");
            self.data = slice::from_raw_parts_mut(address as *mut Color, (self.w * self.h) as usize);
        }
    }

    /// Set title
    pub fn set_title(&mut self, title: &str) {
        let _ = self.file.write(&format!("T,{}", title).as_bytes());
        self.sync_path();
    }

    /// Blocking iterator over events
    pub fn events(&mut self) -> EventIter {
        let mut iter = EventIter {
            events: [Event::new(); 16],
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
                    if self.resizable {
                        let mut resize = None;
                        for i in 0..iter.count {
                            let event = &iter.events[i];
                            if event.code == EVENT_RESIZE {
                                resize = Some((event.a as u32, event.b as u32));
                            }
                        }
                        if let Some((w, h)) = resize {
                            self.set_size(w, h);
                        }
                    }
                    break 'blocking;
                },
                Err(_) => break 'blocking,
            }
        }

        iter
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
    events: [Event; 16],
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
