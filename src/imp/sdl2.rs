extern crate sdl2;

use std::{mem, ptr, slice};
use std::sync::atomic::{AtomicUsize, ATOMIC_USIZE_INIT, Ordering};

use color::Color;
use event::*;
use input::scancode::*;
use renderer::Renderer;
use WindowFlag;

static SDL_USAGES: AtomicUsize = ATOMIC_USIZE_INIT;
/// SDL2 Context
static mut SDL_CTX: *mut sdl2::Sdl = ptr::null_mut();
/// Video Context
static mut VIDEO_CTX: *mut sdl2::VideoSubsystem = ptr::null_mut();
/// Event Pump
static mut EVENT_PUMP: *mut sdl2::EventPump = ptr::null_mut();

//Call this when the CTX needs to be used is created
#[inline]
unsafe fn init() {
    if SDL_USAGES.fetch_add(1, Ordering::Relaxed) == 0 {
        SDL_CTX = Box::into_raw(Box::new(sdl2::init().unwrap()));
        VIDEO_CTX = Box::into_raw(Box::new((&mut *SDL_CTX).video().unwrap()));
        EVENT_PUMP = Box::into_raw(Box::new((&mut *SDL_CTX).event_pump().unwrap()));
    }
}

pub fn get_display_size() -> Result<(u32, u32), String> {
    unsafe { init() };
    unsafe { & *VIDEO_CTX }.display_bounds(0)
        .map(|rect| (rect.width(), rect.height()))
        .map_err(|err| format!("{}", err))
}

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
        Window::new_flags(x, y, w, h, title, &[])
    }

    /// Create a new window with flags
    pub fn new_flags(x: i32, y: i32, w: u32, h: u32, title: &str, flags: &[WindowFlag]) -> Option<Self> {
        //Insure that init has been called
        unsafe { init() };

        let mut async = false;
        let mut resizable = false;
        //TODO: Hide exit button
        let mut _unclosable = false;
        for &flag in flags.iter() {
            match flag {
                WindowFlag::Async => async = true,
                WindowFlag::Resizable => resizable = true,
                WindowFlag::Unclosable => _unclosable = true,
            }
        }

        let mut builder = unsafe { & *VIDEO_CTX }.window(title, w, h);

        if resizable {
            builder.resizable();
        }

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
                sdl2::keyboard::Scancode::A => Some((if shift { 'A' } else { 'a' }, SC_A)),
                sdl2::keyboard::Scancode::B => Some((if shift { 'B' } else { 'b' }, SC_B)),
                sdl2::keyboard::Scancode::C => Some((if shift { 'C' } else { 'c' }, SC_C)),
                sdl2::keyboard::Scancode::D => Some((if shift { 'D' } else { 'd' }, SC_D)),
                sdl2::keyboard::Scancode::E => Some((if shift { 'E' } else { 'e' }, SC_E)),
                sdl2::keyboard::Scancode::F => Some((if shift { 'F' } else { 'f' }, SC_F)),
                sdl2::keyboard::Scancode::G => Some((if shift { 'G' } else { 'g' }, SC_G)),
                sdl2::keyboard::Scancode::H => Some((if shift { 'H' } else { 'h' }, SC_H)),
                sdl2::keyboard::Scancode::I => Some((if shift { 'I' } else { 'i' }, SC_I)),
                sdl2::keyboard::Scancode::J => Some((if shift { 'J' } else { 'j' }, SC_J)),
                sdl2::keyboard::Scancode::K => Some((if shift { 'K' } else { 'k' }, SC_K)),
                sdl2::keyboard::Scancode::L => Some((if shift { 'L' } else { 'l' }, SC_L)),
                sdl2::keyboard::Scancode::M => Some((if shift { 'M' } else { 'm' }, SC_M)),
                sdl2::keyboard::Scancode::N => Some((if shift { 'N' } else { 'n' }, SC_N)),
                sdl2::keyboard::Scancode::O => Some((if shift { 'O' } else { 'o' }, SC_O)),
                sdl2::keyboard::Scancode::P => Some((if shift { 'P' } else { 'p' }, SC_P)),
                sdl2::keyboard::Scancode::Q => Some((if shift { 'Q' } else { 'q' }, SC_Q)),
                sdl2::keyboard::Scancode::R => Some((if shift { 'R' } else { 'r' }, SC_R)),
                sdl2::keyboard::Scancode::S => Some((if shift { 'S' } else { 's' }, SC_S)),
                sdl2::keyboard::Scancode::T => Some((if shift { 'T' } else { 't' }, SC_T)),
                sdl2::keyboard::Scancode::U => Some((if shift { 'U' } else { 'u' }, SC_U)),
                sdl2::keyboard::Scancode::V => Some((if shift { 'V' } else { 'v' }, SC_V)),
                sdl2::keyboard::Scancode::W => Some((if shift { 'W' } else { 'w' }, SC_W)),
                sdl2::keyboard::Scancode::X => Some((if shift { 'X' } else { 'x' }, SC_X)),
                sdl2::keyboard::Scancode::Y => Some((if shift { 'Y' } else { 'y' }, SC_Y)),
                sdl2::keyboard::Scancode::Z => Some((if shift { 'Z' } else { 'z' }, SC_Z)),
                sdl2::keyboard::Scancode::Num0 => Some((if shift { ')' } else { '0' }, SC_0)),
                sdl2::keyboard::Scancode::Num1 => Some((if shift { '!' } else { '1' }, SC_1)),
                sdl2::keyboard::Scancode::Num2 => Some((if shift { '@' } else { '2' }, SC_2)),
                sdl2::keyboard::Scancode::Num3 => Some((if shift { '#' } else { '3' }, SC_3)),
                sdl2::keyboard::Scancode::Num4 => Some((if shift { '$' } else { '4' }, SC_4)),
                sdl2::keyboard::Scancode::Num5 => Some((if shift { '%' } else { '5' }, SC_5)),
                sdl2::keyboard::Scancode::Num6 => Some((if shift { '^' } else { '6' }, SC_6)),
                sdl2::keyboard::Scancode::Num7 => Some((if shift { '&' } else { '7' }, SC_7)),
                sdl2::keyboard::Scancode::Num8 => Some((if shift { '*' } else { '8' }, SC_8)),
                sdl2::keyboard::Scancode::Num9 => Some((if shift { '(' } else { '9' }, SC_9)),
                sdl2::keyboard::Scancode::Grave => Some((if shift { '~' } else { '`' }, SC_TICK)),
                sdl2::keyboard::Scancode::Minus => Some((if shift { '_' } else { '-' }, SC_MINUS)),
                sdl2::keyboard::Scancode::Equals => Some((if shift { '+' } else { '=' }, SC_EQUALS)),
                sdl2::keyboard::Scancode::LeftBracket => Some((if shift { '{' } else { '[' }, SC_BRACE_OPEN)),
                sdl2::keyboard::Scancode::RightBracket => Some((if shift { '}' } else { ']' }, SC_BRACE_CLOSE)),
                sdl2::keyboard::Scancode::Backslash => Some((if shift { '|' } else { '\\' }, SC_BACKSLASH)),
                sdl2::keyboard::Scancode::Semicolon => Some((if shift { ':' } else { ';' }, SC_SEMICOLON)),
                sdl2::keyboard::Scancode::Apostrophe => Some((if shift { '"' } else { '\'' }, SC_QUOTE)),
                sdl2::keyboard::Scancode::Comma => Some((if shift { '<' } else { ',' }, SC_COMMA)),
                sdl2::keyboard::Scancode::Period => Some((if shift { '>' } else { '.' }, SC_PERIOD)),
                sdl2::keyboard::Scancode::Slash => Some((if shift { '?' } else { '/' }, SC_SLASH)),
                sdl2::keyboard::Scancode::Space => Some((' ', SC_SPACE)),
                sdl2::keyboard::Scancode::Backspace => Some(('\0', SC_BKSP)),
                sdl2::keyboard::Scancode::Tab => Some(('\t', SC_TAB)),
                sdl2::keyboard::Scancode::LCtrl => Some(('\0', SC_CTRL)),
                sdl2::keyboard::Scancode::RCtrl => Some(('\0', SC_CTRL)),
                sdl2::keyboard::Scancode::LAlt => Some(('\0', SC_ALT)),
                sdl2::keyboard::Scancode::RAlt => Some(('\0', SC_ALT)),
                sdl2::keyboard::Scancode::Return => Some(('\n', SC_ENTER)),
                sdl2::keyboard::Scancode::Escape => Some(('\x1B', SC_ESC)),
                sdl2::keyboard::Scancode::F1 => Some(('\0', SC_F1)),
                sdl2::keyboard::Scancode::F2 => Some(('\0', SC_F2)),
                sdl2::keyboard::Scancode::F3 => Some(('\0', SC_F3)),
                sdl2::keyboard::Scancode::F4 => Some(('\0', SC_F4)),
                sdl2::keyboard::Scancode::F5 => Some(('\0', SC_F5)),
                sdl2::keyboard::Scancode::F6 => Some(('\0', SC_F6)),
                sdl2::keyboard::Scancode::F7 => Some(('\0', SC_F7)),
                sdl2::keyboard::Scancode::F8 => Some(('\0', SC_F8)),
                sdl2::keyboard::Scancode::F9 => Some(('\0', SC_F9)),
                sdl2::keyboard::Scancode::F10 => Some(('\0', SC_F10)),
                sdl2::keyboard::Scancode::Home => Some(('\0', SC_HOME)),
                sdl2::keyboard::Scancode::Up => Some(('\0', SC_UP)),
                sdl2::keyboard::Scancode::PageUp => Some(('\0', SC_PGUP)),
                sdl2::keyboard::Scancode::Left => Some(('\0', SC_LEFT)),
                sdl2::keyboard::Scancode::Right => Some(('\0', SC_RIGHT)),
                sdl2::keyboard::Scancode::End => Some(('\0', SC_END)),
                sdl2::keyboard::Scancode::Down => Some(('\0', SC_DOWN)),
                sdl2::keyboard::Scancode::PageDown => Some(('\0', SC_PGDN)),
                sdl2::keyboard::Scancode::Delete => Some(('\0', SC_DEL)),
                sdl2::keyboard::Scancode::F11 => Some(('\0', SC_F11)),
                sdl2::keyboard::Scancode::F12 => Some(('\0', SC_F12)),
                sdl2::keyboard::Scancode::LShift => Some(('\0', SC_LEFT_SHIFT)),
                sdl2::keyboard::Scancode::RShift => Some(('\0', SC_RIGHT_SHIFT)),
                _ => None
            }
        } else {
            None
        }
    }

    fn convert_event(&self, event: sdl2::event::Event) -> Vec<Event> {
        let mut events = Vec::new();

        let button_event = || -> Event {
            let mouse = unsafe { &mut *EVENT_PUMP }.mouse_state();
            ButtonEvent {
                left: mouse.left(),
                middle: mouse.middle(),
                right: mouse.right()
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
            sdl2::event::Event::Window { win_event, .. } => match win_event {
                sdl2::event::WindowEvent::Moved(x, y) => events.push(MoveEvent {
                        x: x,
                        y: y
                    }.to_event()),
                sdl2::event::WindowEvent::Resized(w, h) => events.push(ResizeEvent {
                        width: w as u32,
                        height: h as u32
                    }.to_event()),
                sdl2::event::WindowEvent::FocusGained => events.push(FocusEvent {
                        focused: true
                    }.to_event()),
                sdl2::event::WindowEvent::FocusLost => events.push(FocusEvent {
                        focused: false
                    }.to_event()),
                _ => ()
            },
            sdl2::event::Event::MouseMotion { x, y, .. } => events.push(MouseEvent {
                    x: x,
                    y: y
                }.to_event()),
            sdl2::event::Event::MouseButtonDown { .. } => events.push(button_event()),
            sdl2::event::Event::MouseButtonUp { .. } => events.push(button_event()),
            sdl2::event::Event::MouseWheel { x, y, .. } => events.push(ScrollEvent {
                    x: x,
                    y: y
                }.to_event()),
            sdl2::event::Event::KeyDown { scancode, .. } => if let Some(code) = self.convert_scancode(scancode, shift) {
                    /* TODO - figure out what to do with `keycode` (which atm is scancode) and `modifiers` */
                    events.push(KeyEvent {
                        character: code.0,
                        keycode: code.1,
                        pressed: true,
                        modifiers: ModKeys::empty(),
                    }.to_event());
                },
            sdl2::event::Event::KeyUp { scancode, .. } => if let Some(code) = self.convert_scancode(scancode, shift) {
                    /* TODO - figure out what to do with `keycode` and `modifiers` */
                    events.push(KeyEvent {
                        character: code.0,
                        keycode: code.1,
                        pressed: false,
                        modifiers: ModKeys::empty(),
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
