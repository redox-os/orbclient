extern crate sdl2;

use std::{mem, slice};

use super::{FONT, Color};
use super::event::*;

/// A window
#[allow(dead_code)]
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
    /// SDL2 Context
    ctx: sdl2::Sdl,
    /// Video Context
    video_ctx: sdl2::VideoSubsystem,
    /// Event Pump
    event_pump: sdl2::EventPump,
    /// The inner renderer
    inner: sdl2::render::Renderer<'static>,
}

impl Window {
    /// Create a new window
    pub fn new(x: i32, y: i32, w: u32, h: u32, title: &str) -> Option<Box<Self>> {
        Window::new_flags(x, y, w, h, title, false)
    }

    /// Create a new window with flags
    pub fn new_flags(x: i32, y: i32, w: u32, h: u32, title: &str, async: bool) -> Option<Box<Self>> {
        let ctx = sdl2::init().unwrap();
        let video_ctx = ctx.video().unwrap();
        let event_pump = ctx.event_pump().unwrap();

        let mut builder = video_ctx.window(title, w, h);

        if x >= 0 || y >= 0 {
            builder.position(x, y);
        }

        match builder.build() {
            Ok(window) => Some(Box::new(Window {
                x: x,
                y: y,
                w: w,
                h: h,
                t: title.to_string(),
                async: async,
                ctx: ctx,
                video_ctx: video_ctx,
                event_pump: event_pump,
                inner: window.renderer().build().unwrap(),
            })),
            Err(_) => None
        }
    }

    pub fn sync_path(&mut self) {
        if let Some(window) = self.inner.window() {
            self.x = window.position().0;
            self.x = window.position().1;
            self.w = window.size().0;
            self.h = window.size().1;
            self.t = window.title().to_string();
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
        let bytes = self.inner.surface().unwrap().without_lock().unwrap();
        unsafe { slice::from_raw_parts(bytes.as_ptr() as *const Color, bytes.len()/mem::size_of::<Color>()) }
    }

    pub fn data_mut(&mut self) -> &mut [Color] {
        let bytes = self.inner.surface_mut().unwrap().without_lock_mut().unwrap();
        unsafe { slice::from_raw_parts_mut(bytes.as_mut_ptr() as *mut Color, bytes.len()/mem::size_of::<Color>()) }
    }

    /// Draw a pixel
    pub fn pixel(&mut self, x: i32, y: i32, color: Color) {
        self.inner.set_blend_mode(sdl2::render::BlendMode::Blend);
        self.inner.set_draw_color(sdl2::pixels::Color::RGBA((color.data >> 16) as u8, (color.data >> 8) as u8, color.data as u8, (color.data >> 24) as u8));
        self.inner.draw_point(sdl2::rect::Point::new(x, y));
    }

    /// Draw a character, using the loaded font
    pub fn char(&mut self, x: i32, y: i32, c: char, color: Color) {
        self.inner.set_blend_mode(sdl2::render::BlendMode::Blend);
        self.inner.set_draw_color(sdl2::pixels::Color::RGBA((color.data >> 16) as u8, (color.data >> 8) as u8, color.data as u8, (color.data >> 24) as u8));

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
                    self.inner.draw_point(sdl2::rect::Point::new(x + col as i32, y + row as i32));
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
        if let Some(rect) = sdl2::rect::Rect::new(0, 0, self.w, self.h).unwrap_or(None) {
            self.inner.set_blend_mode(sdl2::render::BlendMode::None);
            self.inner.set_draw_color(sdl2::pixels::Color::RGBA((color.data >> 16) as u8, (color.data >> 8) as u8, color.data as u8, (color.data >> 24) as u8));
            self.inner.fill_rect(rect);
        }
    }

    /// Draw rectangle
    // TODO: Improve speed
    #[allow(unused_variables)]
    pub fn rect(&mut self, start_x: i32, start_y: i32, w: u32, h: u32, color: Color) {
        if let Some(rect) = sdl2::rect::Rect::new(start_x, start_y, w, h).unwrap_or(None) {
            self.inner.set_blend_mode(sdl2::render::BlendMode::Blend);
            self.inner.set_draw_color(sdl2::pixels::Color::RGBA((color.data >> 16) as u8, (color.data >> 8) as u8, color.data as u8, (color.data >> 24) as u8));
            self.inner.fill_rect(rect);
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

    fn convert_keycode(&self, keycode_option: Option<sdl2::keyboard::Keycode>) -> Option<(char, u8)> {
        if let Some(keycode) = keycode_option {
            match keycode {
                sdl2::keyboard::Keycode::Return => Some(('\n', 0)),

                sdl2::keyboard::Keycode::Escape => Some(('\x1B', K_ESC)),
                sdl2::keyboard::Keycode::Backspace => Some(('\0', K_BKSP)),
                sdl2::keyboard::Keycode::Tab => Some(('\t', K_TAB)),
                sdl2::keyboard::Keycode::LCtrl => Some(('\0', K_CTRL)),
                sdl2::keyboard::Keycode::RCtrl => Some(('\0', K_CTRL)),
                sdl2::keyboard::Keycode::LAlt => Some(('\0', K_ALT)),
                sdl2::keyboard::Keycode::RAlt => Some(('\0', K_ALT)),
                sdl2::keyboard::Keycode::F1 => Some(('\0', K_F1)),
                sdl2::keyboard::Keycode::F2 => Some(('\0', K_F2)),
                sdl2::keyboard::Keycode::F3 => Some(('\0', K_F3)),
                sdl2::keyboard::Keycode::F4 => Some(('\0', K_F4)),
                sdl2::keyboard::Keycode::F5 => Some(('\0', K_F5)),
                sdl2::keyboard::Keycode::F6 => Some(('\0', K_F6)),
                sdl2::keyboard::Keycode::F7 => Some(('\0', K_F7)),
                sdl2::keyboard::Keycode::F8 => Some(('\0', K_F8)),
                sdl2::keyboard::Keycode::F9 => Some(('\0', K_F9)),
                sdl2::keyboard::Keycode::F10 => Some(('\0', K_F10)),
                sdl2::keyboard::Keycode::Home => Some(('\0', K_HOME)),
                sdl2::keyboard::Keycode::Up => Some(('\0', K_UP)),
                sdl2::keyboard::Keycode::PageUp => Some(('\0', K_PGUP)),
                sdl2::keyboard::Keycode::Left => Some(('\0', K_LEFT)),
                sdl2::keyboard::Keycode::Right => Some(('\0', K_RIGHT)),
                sdl2::keyboard::Keycode::End => Some(('\0', K_END)),
                sdl2::keyboard::Keycode::Down => Some(('\0', K_DOWN)),
                sdl2::keyboard::Keycode::PageDown => Some(('\0', K_PGDN)),
                sdl2::keyboard::Keycode::Delete => Some(('\0', K_DEL)),
                sdl2::keyboard::Keycode::F11 => Some(('\0', K_F11)),
                sdl2::keyboard::Keycode::F12 => Some(('\0', K_F12)),
                sdl2::keyboard::Keycode::LShift => Some(('\0', K_LEFT_SHIFT)),
                sdl2::keyboard::Keycode::RShift => Some(('\0', K_RIGHT_SHIFT)),
                _ => None
            }
        } else {
            None
        }
    }

    fn convert_event(&self, event: sdl2::event::Event) -> Vec<Event> {
        let mut events = Vec::new();

        let mouse_event = || -> Event {
            let mouse = self.ctx.mouse().mouse_state();
            MouseEvent {
                x: mouse.1,
                y: mouse.2,
                left_button: mouse.0.left(),
                middle_button: mouse.0.middle(),
                right_button: mouse.0.right()
            }.to_event()
        };

        match event {
            sdl2::event::Event::MouseMotion { .. } => events.push(mouse_event()),
            sdl2::event::Event::MouseButtonDown { .. } => events.push(mouse_event()),
            sdl2::event::Event::MouseButtonUp { .. } => events.push(mouse_event()),
            sdl2::event::Event::KeyDown { keycode, .. } => if let Some(code) = self.convert_keycode(keycode) {
                events.push(KeyEvent {
                    character: code.0,
                    scancode: code.1,
                    pressed: true
                }.to_event());
            },
            sdl2::event::Event::KeyUp { keycode, .. } => if let Some(code) = self.convert_keycode(keycode) {
                events.push(KeyEvent {
                    character: code.0,
                    scancode: code.1,
                    pressed: false
                }.to_event());
            },
            sdl2::event::Event::TextInput { text, .. } => for c in text.chars() {
                events.push(KeyEvent {
                    character: c,
                    scancode: 0,
                    pressed: true
                }.to_event());
                events.push(KeyEvent {
                    character: c,
                    scancode: 0,
                    pressed: false
                }.to_event());
            },
            sdl2::event::Event::Quit { .. } => events.push(QuitEvent.to_event()),
            _ => (),
        }

        events
    }

    /// Blocking iterator over events
    pub fn events(&mut self) -> EventIter {
        let mut iter = EventIter {
            events: [Event::new(); 128],
            i: 0,
            count: 0,
        };

        if ! self.async {
            let event = self.event_pump.wait_event();
            for converted_event in self.convert_event(event) {
                if iter.count < iter.events.len() {
                    iter.events[iter.count] = converted_event;
                    iter.count += 1;
                } else {
                    break;
                }
            }
        }

        while let Some(event) = self.event_pump.poll_event() {
            for converted_event in self.convert_event(event) {
                if iter.count < iter.events.len() {
                    iter.events[iter.count] = converted_event;
                    iter.count += 1;
                } else {
                    break;
                }
            }
            if iter.count + 2 < iter.events.len() {
                break;
            }
        }

        iter
    }

    /// Flip the window buffer
    pub fn sync(&mut self) -> bool {
        self.inner.present();
        true
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
