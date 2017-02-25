extern crate sdl2;

use std::{mem, slice};

use color::Color;
use event::*;
use super::{init, SDL_CTX, VIDEO_CTX, EVENT_PUMP};
use renderer::Renderer;

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
        let window = self.inner.window().unwrap();
        let surface = window.surface(unsafe { & *EVENT_PUMP }).unwrap();
        let bytes = surface.without_lock().unwrap();
        unsafe { slice::from_raw_parts(bytes.as_ptr() as *const Color, bytes.len()/mem::size_of::<Color>()) }
    }

    /// Access pixel buffer mutably
    fn data_mut(&mut self) -> &mut [Color] {
        let window = self.inner.window_mut().unwrap();
        let surface = window.surface_mut(unsafe { & *EVENT_PUMP }).unwrap();
        let bytes = surface.without_lock_mut().unwrap();
        unsafe { slice::from_raw_parts_mut(bytes.as_mut_ptr() as *mut Color, bytes.len()/mem::size_of::<Color>()) }
    }

    /// Flip the window buffer
    fn sync(&mut self) -> bool {
        self.inner.present();
        true
    }
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

        if title.is_empty() {
            builder.borderless();
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
            let pos = window.position();
            let size = window.size();
            let title = window.title();
            self.x = pos.0;
            self.y = pos.1;
            self.w = size.0;
            self.h = size.1;
            self.t = title.to_string();
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
        if let Some(mut window) = self.inner.window_mut() {
            let _ = window.set_position(sdl2::video::WindowPos::Positioned(x),
                                        sdl2::video::WindowPos::Positioned(y));
        }
        self.sync_path();
    }

    // Set size
    pub fn set_size(&mut self, width: u32, height: u32) {
        if let Some(mut window) = self.inner.window_mut() {
            let _ = window.set_size(width, height);
        }
        self.sync_path();
    }

    /// Set title
    pub fn set_title(&mut self, title: &str) {
        if let Some(mut window) = self.inner.window_mut() {
            let _ = window.set_title(title);
        }
        self.sync_path();
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
            let mouse = unsafe { &mut *EVENT_PUMP }.mouse_state();
            MouseEvent {
                x: mouse.x(),
                y: mouse.y(),
                left_button: mouse.left(),
                middle_button: mouse.middle(),
                right_button: mouse.right()
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
