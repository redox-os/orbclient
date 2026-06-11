use core::fmt::Display;
use core::str::FromStr;

#[derive(Clone, Copy, Debug)]
pub enum WindowFlag {
    /// Asynchronous Event
    Async,
    /// Do not use. Reserved for background
    Back,
    /// Always on top, even taskbar
    Front,
    /// Hide window border, handle your own window dragging API
    Borderless,
    /// Window manager can resize the window
    Resizable,
    /// Apply blending with window behind (slower)
    Transparent,
    /// Cannot be closed, handle your own window close API
    Unclosable,
}

pub struct WindowFlags(String, usize);

impl Display for WindowFlag {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            WindowFlag::Async => write!(f, "a"),
            WindowFlag::Back => write!(f, "b"),
            WindowFlag::Front => write!(f, "f"),
            WindowFlag::Borderless => write!(f, "l"),
            WindowFlag::Resizable => write!(f, "r"),
            WindowFlag::Transparent => write!(f, "t"),
            WindowFlag::Unclosable => write!(f, "u"),
        }
    }
}

impl FromStr for WindowFlags {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.into(), 0))
    }
}

impl Iterator for WindowFlags {
    type Item = WindowFlag;

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.1;
        if self.0.len() >= i {
            return None;
        }
        self.1 += 1;
        match self.0.as_bytes()[i] {
            b'a' => Some(WindowFlag::Async),
            b'b' => Some(WindowFlag::Borderless),
            b'f' => Some(WindowFlag::Front),
            b'l' => Some(WindowFlag::Borderless),
            b'r' => Some(WindowFlag::Resizable),
            b't' => Some(WindowFlag::Transparent),
            b'u' => Some(WindowFlag::Unclosable),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum SurfaceFlag {}

#[derive(Clone, Copy, Debug)]
pub enum Mode {
    Blend,     //Composite
    Overwrite, //Replace
}
