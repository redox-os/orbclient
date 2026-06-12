use core::fmt::Display;
use core::str::FromStr;
#[cfg(not(feature = "std"))]
use alloc::string::String;

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

pub enum WindowDragKind {
    None,
    Move,
    ResizeLeft,
    ResizeRight,
    ResizeBottom,
    ResizeTop,
}

impl Display for WindowDragKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            WindowDragKind::Move => write!(f, "m"),
            WindowDragKind::ResizeLeft => write!(f, "l"),
            WindowDragKind::ResizeRight => write!(f, "r"),
            WindowDragKind::ResizeBottom => write!(f, "b"),
            WindowDragKind::ResizeTop => write!(f, "t"),
            WindowDragKind::None => write!(f, "0"),
        }
    }
}

impl WindowDragKind {
    pub fn from_u8(val: i64) -> Option<Self> {
        match val {
            0 => Some(WindowDragKind::None),
            1 => Some(WindowDragKind::Move),
            2 => Some(WindowDragKind::ResizeLeft),
            3 => Some(WindowDragKind::ResizeRight),
            4 => Some(WindowDragKind::ResizeBottom),
            5 => Some(WindowDragKind::ResizeTop),
            _ => None,
        }
    }
    pub fn to_u8(&self) -> i64 {
        match self {
            WindowDragKind::None => 0,
            WindowDragKind::Move => 1,
            WindowDragKind::ResizeLeft => 2,
            WindowDragKind::ResizeRight => 3,
            WindowDragKind::ResizeBottom => 4,
            WindowDragKind::ResizeTop => 5,
        }
    }
    #[allow(unused)]
    pub(crate) fn to_orbital_cmd(&self) -> &'static [u8] {
        match self {
            WindowDragKind::None => b"D,0",
            WindowDragKind::Move => b"D,m",
            WindowDragKind::ResizeLeft => b"D,l",
            WindowDragKind::ResizeRight => b"D,r",
            WindowDragKind::ResizeBottom => b"D,b",
            WindowDragKind::ResizeTop => b"D,t",
        }
    }
}
impl FromStr for WindowDragKind {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "m" => Ok(WindowDragKind::Move),
            "l" => Ok(WindowDragKind::ResizeLeft),
            "r" => Ok(WindowDragKind::ResizeRight),
            "b" => Ok(WindowDragKind::ResizeBottom),
            "t" => Ok(WindowDragKind::ResizeTop),
            _ => Err(s.into()),
        }
    }
}

/// A type of media that registered into clipboard or DND system.
/// Text, File, Uri is designed for simple implementation.
/// For programs that wants to send custom MIME, use Data.
/// Cannot support multiple kind of data at the moment.
#[derive(Clone, Copy, Debug)]
pub enum MediaKind {
    /// A regular text
    Text,
    /// A file path (can be multiple, \n separated)
    File,
    /// A URI path (HTTP or internal identifier)
    Uri,
    /// A data URI
    Data,
    /// Do not use
    Any,
}

impl MediaKind {
    pub fn from_i64(val: i64) -> Self {
        match val {
            1 => MediaKind::Text,
            2 => MediaKind::File,
            3 => MediaKind::Uri,
            4 => MediaKind::Data,
            _ => MediaKind::Any,
        }
    }
    pub fn to_i64(&self) -> i64 {
        match self {
            MediaKind::Text => 1,
            MediaKind::File => 2,
            MediaKind::Uri => 3,
            MediaKind::Data => 4,
            MediaKind::Any => 1,
        }
    }
    pub fn to_mime(&self, data: &str) -> String {
        const DEFAULT_MIME: &str = "application/octet-stream";
        match self {
            MediaKind::Text => "text/plain".into(),
            MediaKind::File => "text/uri-list".into(),
            MediaKind::Uri => "text/x-uri".into(),
            MediaKind::Data => {
                if let Some(rest) = data.strip_prefix("data:") {
                    if let Some((prefix, _)) = rest.split_once(&[';', ',', '\n']) {
                        if prefix.len() < 100 {
                            return prefix.into();
                        }
                    }
                }
                DEFAULT_MIME.into()
            }
            MediaKind::Any => DEFAULT_MIME.into(),
        }
    }
}

impl Display for MediaKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            MediaKind::Text => write!(f, "t"),
            MediaKind::File => write!(f, "f"),
            MediaKind::Uri => write!(f, "u"),
            MediaKind::Data => write!(f, "d"),
            MediaKind::Any => write!(f, " "),
        }
    }
}

impl FromStr for MediaKind {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "t" => Ok(MediaKind::Text),
            "f" => Ok(MediaKind::File),
            "u" => Ok(MediaKind::Uri),
            "d" => Ok(MediaKind::Data),
            " " => Ok(MediaKind::Any),
            _ => Err(s.into()),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ClipboardAction {
    Copy,
    Cut,
    Paste,
}

impl ClipboardAction {
    pub fn from_u8(val: u8) -> Option<Self> {
        match val {
            1 => Some(ClipboardAction::Copy),
            2 => Some(ClipboardAction::Cut),
            3 => Some(ClipboardAction::Paste),
            _ => None,
        }
    }
    pub fn to_u8(&self) -> u8 {
        match self {
            ClipboardAction::Copy => 1,
            ClipboardAction::Cut => 2,
            ClipboardAction::Paste => 3,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum DragAction {
    Copy,
    Move,
    Link,
    None,
}

impl DragAction {
    pub fn from_u8(val: u8) -> Option<Self> {
        match val {
            1 => Some(DragAction::Copy),
            2 => Some(DragAction::Move),
            4 => Some(DragAction::Link),
            0 => Some(DragAction::None),
            _ => None,
        }
    }
    pub fn to_u8(&self) -> u8 {
        match self {
            DragAction::Copy => 1,
            DragAction::Move => 2,
            DragAction::Link => 4,
            DragAction::None => 0,
        }
    }
}
