extern crate sdl2;

use std::{mem, slice};

use super::super::{FONT, Color};
use super::super::event::*;
use super::{init, SDL_CTX, VIDEO_CTX, EVENT_PUMP};

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
    /// The inner renderer
    inner: sdl2::render::Renderer<'static>,
}

impl Window {
    /// Create a new window
    pub fn new(x: i32, y: i32, w: u32, h: u32, title: &str) -> Option<Self> {
        Window::new_flags(x, y, w, h, title, false)
    }

    /// Create a new window with flags
    pub fn new_flags(x: i32, y: i32, w: u32, h: u32, title: &str, async: bool) -> Option<Self> {
        //Insure that init has been called
        unsafe { init() };

        let mut builder = unsafe { & *VIDEO_CTX }.window(title, w, h);

        if x >= 0 || y >= 0 {
            builder.position(x, y);
        }

        match builder.build() {
            Ok(window) => Some(Window {
                x: x,
                y: y,
                w: w,
                h: h,
                t: title.to_string(),
                async: async,
                inner: window.renderer().software().build().unwrap(),
            }),
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
        let window = self.inner.window().unwrap();
        let surface = window.surface(unsafe { & *EVENT_PUMP }).unwrap();
        let bytes = surface.without_lock().unwrap();
        unsafe { slice::from_raw_parts(bytes.as_ptr() as *const Color, bytes.len()/mem::size_of::<Color>()) }
    }

    pub fn data_mut(&mut self) -> &mut [Color] {
        let window = self.inner.window_mut().unwrap();
        let surface = window.surface_mut(unsafe { & *EVENT_PUMP }).unwrap();
        let bytes = surface.without_lock_mut().unwrap();
        unsafe { slice::from_raw_parts_mut(bytes.as_mut_ptr() as *mut Color, bytes.len()/mem::size_of::<Color>()) }
    }

    /// Draw a pixel
    pub fn pixel(&mut self, x: i32, y: i32, color: Color) {
        self.inner.set_blend_mode(sdl2::render::BlendMode::Blend);
        self.inner.set_draw_color(sdl2::pixels::Color::RGBA((color.data >> 16) as u8, (color.data >> 8) as u8, color.data as u8, (color.data >> 24) as u8));
        self.inner.draw_point(sdl2::rect::Point::new(x, y));
    }

    /// Draw a circle. Negative radius will fill in the inside
    pub fn circle(&mut self, x0: i32, y0: i32, radius: i32, color: Color) {
        let mut x = radius.abs();
        let mut y = 0;
        let mut err = 0;

        while x >= y {
            if radius < 0 {
                self.rect(x0 - x, y0 + y, x as u32 * 2, 1, color);
                self.rect(x0 - y, y0 + x, y as u32 * 2, 1, color);
                self.rect(x0 - x, y0 - y, x as u32 * 2, 1, color);
                self.rect(x0 - y, y0 - x, y as u32 * 2, 1, color);
            } else if radius == 0 {
                self.pixel(x0, y0, color);
            } else {
                self.pixel(x0 - x, y0 + y, color);
                self.pixel(x0 + x, y0 + y, color);
                self.pixel(x0 - y, y0 + x, color);
                self.pixel(x0 + y, y0 + x, color);
                self.pixel(x0 - x, y0 - y, color);
                self.pixel(x0 + x, y0 - y, color);
                self.pixel(x0 - y, y0 - x, color);
                self.pixel(x0 + y, y0 - x, color);
            }

            y += 1;
            err += 1 + 2*y;
            if 2*(err-x) + 1 > 0 {
                x -= 1;
                err += 1 - 2*x;
            }
        }
    }

    /// Draw a line
    pub fn line(&mut self, argx1: i32, argy1: i32, argx2: i32, argy2: i32, color: Color) {
        self.inner.set_blend_mode(sdl2::render::BlendMode::Blend);
        self.inner.set_draw_color(sdl2::pixels::Color::RGBA((color.data >> 16) as u8, (color.data >> 8) as u8, color.data as u8, (color.data >> 24) as u8));
        self.inner.draw_line(sdl2::rect::Point::new(argx1, argy1), sdl2::rect::Point::new(argx2, argy2));
    }

    /// Draw multiple lines from point to point.
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
        self.inner.set_blend_mode(sdl2::render::BlendMode::None);
        self.inner.set_draw_color(sdl2::pixels::Color::RGBA((color.data >> 16) as u8, (color.data >> 8) as u8, color.data as u8, (color.data >> 24) as u8));
        self.inner.clear();
    }

    /// Sets the whole window to black
    pub fn clear(&mut self) {
        self.set(Color::rgb(0,0,0));
    }

    /// Draw rectangle
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

    fn convert_scancode(&self, scancode_option: Option<sdl2::keyboard::Scancode>, shift: bool) -> Option<(char, u8)> {
        if let Some(scancode) = scancode_option {
            match scancode {
                sdl2::keyboard::Scancode::A => Some((if shift { 'A' } else { 'a' }, K_A)),
                sdl2::keyboard::Scancode::B => Some((if shift { 'B' } else { 'b' }, K_B)),
                sdl2::keyboard::Scancode::C => Some((if shift { 'C' } else { 'c' }, K_C)),
                sdl2::keyboard::Scancode::D => Some((if shift { 'D' } else { 'd' }, K_D)),
                sdl2::keyboard::Scancode::E => Some((if shift { 'E' } else { 'e' }, K_E)),
                sdl2::keyboard::Scancode::F => Some((if shift { 'F' } else { 'f' }, K_F)),
                sdl2::keyboard::Scancode::G => Some((if shift { 'G' } else { 'g' }, K_G)),
                sdl2::keyboard::Scancode::H => Some((if shift { 'H' } else { 'h' }, K_H)),
                sdl2::keyboard::Scancode::I => Some((if shift { 'I' } else { 'i' }, K_I)),
                sdl2::keyboard::Scancode::J => Some((if shift { 'J' } else { 'j' }, K_J)),
                sdl2::keyboard::Scancode::K => Some((if shift { 'K' } else { 'k' }, K_K)),
                sdl2::keyboard::Scancode::L => Some((if shift { 'L' } else { 'l' }, K_L)),
                sdl2::keyboard::Scancode::M => Some((if shift { 'M' } else { 'm' }, K_M)),
                sdl2::keyboard::Scancode::N => Some((if shift { 'N' } else { 'n' }, K_N)),
                sdl2::keyboard::Scancode::O => Some((if shift { 'O' } else { 'o' }, K_O)),
                sdl2::keyboard::Scancode::P => Some((if shift { 'P' } else { 'p' }, K_P)),
                sdl2::keyboard::Scancode::Q => Some((if shift { 'Q' } else { 'q' }, K_Q)),
                sdl2::keyboard::Scancode::R => Some((if shift { 'R' } else { 'r' }, K_R)),
                sdl2::keyboard::Scancode::S => Some((if shift { 'S' } else { 's' }, K_S)),
                sdl2::keyboard::Scancode::T => Some((if shift { 'T' } else { 't' }, K_T)),
                sdl2::keyboard::Scancode::U => Some((if shift { 'U' } else { 'u' }, K_U)),
                sdl2::keyboard::Scancode::V => Some((if shift { 'V' } else { 'v' }, K_V)),
                sdl2::keyboard::Scancode::W => Some((if shift { 'W' } else { 'w' }, K_W)),
                sdl2::keyboard::Scancode::X => Some((if shift { 'X' } else { 'x' }, K_X)),
                sdl2::keyboard::Scancode::Y => Some((if shift { 'Y' } else { 'y' }, K_Y)),
                sdl2::keyboard::Scancode::Z => Some((if shift { 'Z' } else { 'z' }, K_Z)),
                sdl2::keyboard::Scancode::Num0 => Some((if shift { ')' } else { '0' }, K_0)),
                sdl2::keyboard::Scancode::Num1 => Some((if shift { '!' } else { '1' }, K_1)),
                sdl2::keyboard::Scancode::Num2 => Some((if shift { '@' } else { '2' }, K_2)),
                sdl2::keyboard::Scancode::Num3 => Some((if shift { '#' } else { '3' }, K_3)),
                sdl2::keyboard::Scancode::Num4 => Some((if shift { '$' } else { '4' }, K_4)),
                sdl2::keyboard::Scancode::Num5 => Some((if shift { '%' } else { '5' }, K_5)),
                sdl2::keyboard::Scancode::Num6 => Some((if shift { '^' } else { '6' }, K_6)),
                sdl2::keyboard::Scancode::Num7 => Some((if shift { '&' } else { '7' }, K_7)),
                sdl2::keyboard::Scancode::Num8 => Some((if shift { '*' } else { '8' }, K_8)),
                sdl2::keyboard::Scancode::Num9 => Some((if shift { '(' } else { '9' }, K_9)),
                sdl2::keyboard::Scancode::Grave => Some((if shift { '~' } else { '`' }, K_TICK)),
                sdl2::keyboard::Scancode::Minus => Some((if shift { '_' } else { '-' }, K_MINUS)),
                sdl2::keyboard::Scancode::Equals => Some((if shift { '+' } else { '=' }, K_EQUALS)),
                sdl2::keyboard::Scancode::LeftBracket => Some((if shift { '{' } else { '[' }, K_BRACE_OPEN)),
                sdl2::keyboard::Scancode::RightBracket => Some((if shift { '}' } else { ']' }, K_BRACE_CLOSE)),
                sdl2::keyboard::Scancode::Backslash => Some((if shift { '|' } else { '\\' }, K_BACKSLASH)),
                sdl2::keyboard::Scancode::Semicolon => Some((if shift { ':' } else { ';' }, K_SEMICOLON)),
                sdl2::keyboard::Scancode::Apostrophe => Some((if shift { '"' } else { '\'' }, K_QUOTE)),
                sdl2::keyboard::Scancode::Comma => Some((if shift { '<' } else { ',' }, K_COMMA)),
                sdl2::keyboard::Scancode::Period => Some((if shift { '>' } else { '.' }, K_PERIOD)),
                sdl2::keyboard::Scancode::Slash => Some((if shift { '?' } else { '/' }, K_SLASH)),
                sdl2::keyboard::Scancode::Space => Some((' ', K_SPACE)),
                sdl2::keyboard::Scancode::Backspace => Some(('\0', K_BKSP)),
                sdl2::keyboard::Scancode::Tab => Some(('\t', K_TAB)),
                sdl2::keyboard::Scancode::LCtrl => Some(('\0', K_CTRL)),
                sdl2::keyboard::Scancode::RCtrl => Some(('\0', K_CTRL)),
                sdl2::keyboard::Scancode::LAlt => Some(('\0', K_ALT)),
                sdl2::keyboard::Scancode::RAlt => Some(('\0', K_ALT)),
                sdl2::keyboard::Scancode::Return => Some(('\n', K_ENTER)),
                sdl2::keyboard::Scancode::Escape => Some(('\x1B', K_ESC)),
                sdl2::keyboard::Scancode::F1 => Some(('\0', K_F1)),
                sdl2::keyboard::Scancode::F2 => Some(('\0', K_F2)),
                sdl2::keyboard::Scancode::F3 => Some(('\0', K_F3)),
                sdl2::keyboard::Scancode::F4 => Some(('\0', K_F4)),
                sdl2::keyboard::Scancode::F5 => Some(('\0', K_F5)),
                sdl2::keyboard::Scancode::F6 => Some(('\0', K_F6)),
                sdl2::keyboard::Scancode::F7 => Some(('\0', K_F7)),
                sdl2::keyboard::Scancode::F8 => Some(('\0', K_F8)),
                sdl2::keyboard::Scancode::F9 => Some(('\0', K_F9)),
                sdl2::keyboard::Scancode::F10 => Some(('\0', K_F10)),
                sdl2::keyboard::Scancode::Home => Some(('\0', K_HOME)),
                sdl2::keyboard::Scancode::Up => Some(('\0', K_UP)),
                sdl2::keyboard::Scancode::PageUp => Some(('\0', K_PGUP)),
                sdl2::keyboard::Scancode::Left => Some(('\0', K_LEFT)),
                sdl2::keyboard::Scancode::Right => Some(('\0', K_RIGHT)),
                sdl2::keyboard::Scancode::End => Some(('\0', K_END)),
                sdl2::keyboard::Scancode::Down => Some(('\0', K_DOWN)),
                sdl2::keyboard::Scancode::PageDown => Some(('\0', K_PGDN)),
                sdl2::keyboard::Scancode::Delete => Some(('\0', K_DEL)),
                sdl2::keyboard::Scancode::F11 => Some(('\0', K_F11)),
                sdl2::keyboard::Scancode::F12 => Some(('\0', K_F12)),
                sdl2::keyboard::Scancode::LShift => Some(('\0', K_LEFT_SHIFT)),
                sdl2::keyboard::Scancode::RShift => Some(('\0', K_RIGHT_SHIFT)),
                _ => None
            }
        } else {
            None
        }
    }

    fn convert_event(&self, event: sdl2::event::Event) -> Vec<Event> {
        let mut events = Vec::new();

        let mouse_event = || -> Event {
            let mouse = unsafe { &mut *SDL_CTX }.mouse().mouse_state();
            MouseEvent {
                x: mouse.1,
                y: mouse.2,
                left_button: mouse.0.left(),
                middle_button: mouse.0.middle(),
                right_button: mouse.0.right()
            }.to_event()
        };

        let mods = unsafe { &mut *SDL_CTX }.keyboard().mod_state();
        let shift = if mods.contains(sdl2::keyboard::CAPSMOD)
                    || mods.contains(sdl2::keyboard::LSHIFTMOD)
                    || mods.contains(sdl2::keyboard::RSHIFTMOD)
        {
            true
        } else {
            false
        };

        match event {
            sdl2::event::Event::MouseMotion { .. } => events.push(mouse_event()),
            sdl2::event::Event::MouseButtonDown { .. } => events.push(mouse_event()),
            sdl2::event::Event::MouseButtonUp { .. } => events.push(mouse_event()),
            sdl2::event::Event::KeyDown { scancode, .. } => if let Some(code) = self.convert_scancode(scancode, shift) {
                events.push(KeyEvent {
                    character: code.0,
                    scancode: code.1,
                    pressed: true
                }.to_event());
            },
            sdl2::event::Event::KeyUp { scancode, .. } => if let Some(code) = self.convert_scancode(scancode, shift) {
                events.push(KeyEvent {
                    character: code.0,
                    scancode: code.1,
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
            let event = unsafe { &mut *EVENT_PUMP }.wait_event();
            if let sdl2::event::Event::Window{..} = event {
                self.sync_path();
            }
            for converted_event in self.convert_event(event) {
                if iter.count < iter.events.len() {
                    iter.events[iter.count] = converted_event;
                    iter.count += 1;
                } else {
                    break;
                }
            }
        }

        while let Some(event) = unsafe { &mut *EVENT_PUMP }.poll_event() {
            if let sdl2::event::Event::Window{..} = event {
                self.sync_path();
            }
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
