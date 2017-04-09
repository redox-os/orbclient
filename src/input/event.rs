use core::{char, mem, slice};
use core::ops::{Deref, DerefMut};

pub const EVENT_NONE: i64 = 0;
pub const EVENT_KEY: i64 = 1;
pub const EVENT_MOUSE: i64 = 2;
pub const EVENT_BUTTON: i64 = 3;
pub const EVENT_SCROLL: i64 = 4;
pub const EVENT_QUIT: i64 = 5;
pub const EVENT_FOCUS: i64 = 6;
pub const EVENT_MOVE: i64 = 7;
pub const EVENT_RESIZE: i64 = 8;
pub const EVENT_SCREEN: i64 = 9;

/// An optional event
#[derive(Copy, Clone, Debug)]
pub enum EventOption {
    /// A key event
    Key(KeyEvent),
    /// A mouse event
    Mouse(MouseEvent),
    /// A mouse button event
    Button(ButtonEvent),
    /// A mouse scroll event
    Scroll(ScrollEvent),
    /// A quit request event
    Quit(QuitEvent),
    /// A focus event
    Focus(FocusEvent),
    /// A move event
    Move(MoveEvent),
    /// A resize event
    Resize(ResizeEvent),
    /// A screen report event
    Screen(ScreenEvent),
    /// An unknown event
    Unknown(Event),
    /// No event
    None,
}

/// An event
#[derive(Copy, Clone, Debug)]
#[repr(packed)]
pub struct Event {
    pub code: i64,
    pub a: i64,
    pub b: i64,
}

impl Event {
    /// Create a null event
    pub fn new() -> Event {
        Event {
            code: 0,
            a: 0,
            b: 0,
        }
    }

    /// Convert the event ot an optional event
    // TODO: Consider doing this via a From trait.
    pub fn to_option(self) -> EventOption {
        match self.code {
            EVENT_NONE => EventOption::None,
            EVENT_KEY => EventOption::Key(KeyEvent::from_event(self)),
            EVENT_MOUSE => EventOption::Mouse(MouseEvent::from_event(self)),
            EVENT_BUTTON => EventOption::Button(ButtonEvent::from_event(self)),
            EVENT_SCROLL => EventOption::Scroll(ScrollEvent::from_event(self)),
            EVENT_QUIT => EventOption::Quit(QuitEvent::from_event(self)),
            EVENT_FOCUS => EventOption::Focus(FocusEvent::from_event(self)),
            EVENT_MOVE => EventOption::Move(MoveEvent::from_event(self)),
            EVENT_RESIZE => EventOption::Resize(ResizeEvent::from_event(self)),
            EVENT_SCREEN => EventOption::Screen(ScreenEvent::from_event(self)),
            _ => EventOption::Unknown(self),
        }
    }
}

impl Deref for Event {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(self as *const Event as *const u8, mem::size_of::<Event>()) as &[u8]
        }
    }
}

impl DerefMut for Event {
    fn deref_mut(&mut self) -> &mut [u8] {
        unsafe {
            slice::from_raw_parts_mut(self as *mut Event as *mut u8, mem::size_of::<Event>()) as &mut [u8]
        }
    }
}

bitflags! {
    pub flags ModKeys: u32 {
        const MOD_LSHIFT    = 1 << 0,
        const MOD_RSHIFT    = 1 << 1,
        const MOD_ALT       = 1 << 2,
        const MOD_ALT_GR    = 1 << 3,
        const MOD_SUPER     = 1 << 4,
    }
}

/// A key event (such as a pressed key)
#[derive(Copy, Clone, Debug)]
pub struct KeyEvent {
    /// The character of the key
    pub character: char,
    /// The keycode of the key.
    pub keycode: u8,
    /// Was it pressed?
    pub pressed: bool,
    /// Modifier keys at the time it was pressed
    pub modifiers: ModKeys,
}

impl KeyEvent {
    /// Convert to an `Event`
    pub fn to_event(&self) -> Event {
        Event {
            code: EVENT_KEY,
            a: self.character as i64,
            b: self.keycode as i64 | (self.pressed as i64) << 8 | (self.modifiers.bits as i64) << 16,
        }
    }

    /// Convert from an `Event`
    pub fn from_event(event: Event) -> KeyEvent {
        KeyEvent {
            character: char::from_u32(event.a as u32).unwrap_or('\0'),
            keycode: event.b as u8,
            pressed: event.b & 1 << 8 == 1 << 8,
            modifiers: ModKeys::from_bits_truncate((event.b as u32) >> 16),
        }
    }
}

/// A event related to the mouse
#[derive(Copy, Clone, Debug)]
pub struct MouseEvent {
    /// The x coordinate of the mouse
    pub x: i32,
    /// The y coordinate of the mouse
    pub y: i32,
}

impl MouseEvent {
    /// Convert to an `Event`
    pub fn to_event(&self) -> Event {
        Event {
            code: EVENT_MOUSE,
            a: self.x as i64,
            b: self.y as i64,
        }
    }

    /// Convert an `Event` to a `MouseEvent`
    pub fn from_event(event: Event) -> MouseEvent {
        MouseEvent {
            x: event.a as i32,
            y: event.b as i32,
        }
    }
}

/// A event for clicking the mouse
#[derive(Copy, Clone, Debug)]
pub struct ButtonEvent {
    /// Was the left button pressed?
    pub left: bool,
    /// Was the middle button pressed?
    pub middle: bool,
    /// Was the right button pressed?
    pub right: bool,
}

impl ButtonEvent {
    /// Convert to an `Event`
    pub fn to_event(&self) -> Event {
        Event {
            code: EVENT_BUTTON,
            a: self.left as i64 | (self.middle as i64) << 1 |
               (self.right as i64) << 2,
            b: 0,
        }
    }

    /// Convert an `Event` to a `ButtonEvent`
    pub fn from_event(event: Event) -> ButtonEvent {
        ButtonEvent {
            left: event.a & 1 == 1,
            middle: event.a & 2 == 2,
            right: event.a & 4 == 4,
        }
    }
}

/// A event for scrolling the mouse
#[derive(Copy, Clone, Debug)]
pub struct ScrollEvent {
    /// The x distance of the scroll
    pub x: i32,
    /// The y distance of the scroll
    pub y: i32,
}

impl ScrollEvent {
    /// Convert to an `Event`
    pub fn to_event(&self) -> Event {
        Event {
            code: EVENT_SCROLL,
            a: self.x as i64,
            b: self.y as i64,
        }
    }

    /// Convert an `Event` to a `ScrollEvent`
    pub fn from_event(event: Event) -> ScrollEvent {
        ScrollEvent {
            x: event.a as i32,
            y: event.b as i32,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct QuitEvent;

impl QuitEvent {
    pub fn to_event(&self) -> Event {
        Event {
            code: EVENT_QUIT,
            a: 0,
            b: 0,
        }
    }

    pub fn from_event(_: Event) -> QuitEvent {
        QuitEvent
    }
}

/// A focus event
#[derive(Copy, Clone, Debug)]
pub struct FocusEvent {
    /// True if window has been focused, false if not
    pub focused: bool
}

impl FocusEvent {
    pub fn to_event(&self) -> Event {
        Event {
            code: EVENT_FOCUS,
            a: self.focused as i64,
            b: 0,
        }
    }

    pub fn from_event(event: Event) -> FocusEvent {
        FocusEvent {
            focused: event.a > 0
        }
    }
}

/// A move event
#[derive(Copy, Clone, Debug)]
pub struct MoveEvent {
    pub x: i32,
    pub y: i32
}

impl MoveEvent {
    pub fn to_event(&self) -> Event {
        Event {
            code: EVENT_MOVE,
            a: self.x as i64,
            b: self.y as i64,
        }
    }

    pub fn from_event(event: Event) -> MoveEvent {
        MoveEvent {
            x: event.a as i32,
            y: event.b as i32
        }
    }
}

/// A resize event
#[derive(Copy, Clone, Debug)]
pub struct ResizeEvent {
    pub width: u32,
    pub height: u32
}

impl ResizeEvent {
    pub fn to_event(&self) -> Event {
        Event {
            code: EVENT_RESIZE,
            a: self.width as i64,
            b: self.height as i64,
        }
    }

    pub fn from_event(event: Event) -> ResizeEvent {
        ResizeEvent {
            width: event.a as u32,
            height: event.b as u32
        }
    }
}

/// A screen report event
#[derive(Copy, Clone, Debug)]
pub struct ScreenEvent {
    pub width: u32,
    pub height: u32
}

impl ScreenEvent {
    pub fn to_event(&self) -> Event {
        Event {
            code: EVENT_SCREEN,
            a: self.width as i64,
            b: self.height as i64,
        }
    }

    pub fn from_event(event: Event) -> ScreenEvent {
        ScreenEvent {
            width: event.a as u32,
            height: event.b as u32
        }
    }
}
