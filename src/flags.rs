#[cfg(not(feature = "std"))]
use alloc::string::String;
use core::fmt::Display;
use core::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WindowFlag {
    /// Asynchronous Event
    Async,
    /// Do not use. Reserved for background
    Back,
    /// Always on top, even taskbar
    Front,
    /// Hide the window
    Hidden,
    /// Hide window border, handle your own window dragging API
    Borderless,
    /// Maximize the window
    Maximized,
    /// Full screen the window
    Fullscreen,
    /// Window manager can resize the window
    Resizable,
    /// Window manager can enable window scaling
    Scalable,
    /// Apply blending with window behind (slower)
    Transparent,
    /// Cannot be closed, handle your own window close API
    Unclosable,
}

impl WindowFlag {
    pub const fn try_from_u64(val: u64) -> Option<Self> {
        let val = match val {
            0x0001 => WindowFlag::Async,
            0x0002 => WindowFlag::Back,
            0x0004 => WindowFlag::Front,
            0x0008 => WindowFlag::Resizable,
            0x0010 => WindowFlag::Unclosable,
            0x0020 => WindowFlag::Borderless,
            0x0040 => WindowFlag::Transparent,
            0x0080 => WindowFlag::Maximized,
            0x0100 => WindowFlag::Fullscreen,
            0x0200 => WindowFlag::Hidden,
            0x0400 => WindowFlag::Scalable,
            _ => return None,
        };
        Some(val)
    }
    pub const fn to_u64(&self) -> u64 {
        match self {
            WindowFlag::Async => 0x0001,
            WindowFlag::Back => 0x0002,
            WindowFlag::Front => 0x0004,
            WindowFlag::Resizable => 0x0008,
            WindowFlag::Unclosable => 0x0010,
            WindowFlag::Borderless => 0x0020,
            WindowFlag::Transparent => 0x0040,
            WindowFlag::Maximized => 0x0080,
            WindowFlag::Fullscreen => 0x0100,
            WindowFlag::Hidden => 0x0200,
            WindowFlag::Scalable => 0x0400,
        }
    }
    pub const fn try_from_byte(val: u8) -> Option<Self> {
        match val {
            b'a' => Some(WindowFlag::Async),
            b'b' => Some(WindowFlag::Back),
            b'f' => Some(WindowFlag::Front),
            b'h' => Some(WindowFlag::Hidden),
            b'l' => Some(WindowFlag::Borderless),
            b'm' => Some(WindowFlag::Maximized),
            b'M' => Some(WindowFlag::Fullscreen),
            b'r' => Some(WindowFlag::Resizable),
            b's' => Some(WindowFlag::Scalable),
            b't' => Some(WindowFlag::Transparent),
            b'u' => Some(WindowFlag::Unclosable),
            _ => None,
        }
    }
    const fn max_u64() -> u64 {
        // TODO: use https://github.com/rust-lang/rust/issues/73662
        WindowFlag::Scalable.to_u64()
    }
}

impl Display for WindowFlag {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            WindowFlag::Async => write!(f, "a"),
            WindowFlag::Back => write!(f, "b"),
            WindowFlag::Front => write!(f, "f"),
            WindowFlag::Hidden => write!(f, "h"),
            WindowFlag::Borderless => write!(f, "l"),
            WindowFlag::Maximized => write!(f, "m"),
            WindowFlag::Fullscreen => write!(f, "M"),
            WindowFlag::Resizable => write!(f, "r"),
            WindowFlag::Scalable => write!(f, "s"),
            WindowFlag::Transparent => write!(f, "t"),
            WindowFlag::Unclosable => write!(f, "u"),
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct WindowFlags(u64);

#[derive(Debug, Clone)]
pub struct WindowFlagsIter(u64, u64);

impl WindowFlags {
    /// New flags from array of WindowFlag
    pub const fn new(flags: &[WindowFlag]) -> Self {
        let mut iflags = 0;
        let mut i = 0;
        // Using while loop because of `const fn`
        while i < flags.len() {
            let flag = flags[i];
            i += 1;
            iflags |= flag.to_u64()
        }
        Self(iflags)
    }

    /// New flags from u64. Unknown constants will be saved as it is but not appear in the iterator.
    pub const fn from_u64(flags: u64) -> Self {
        Self(flags)
    }

    /// Encode flags into u64
    pub const fn to_64(&self) -> u64 {
        self.0
    }

    /// Clear flags
    pub fn clear(&mut self) {
        self.0 = 0;
    }

    /// Check if flags contains a flag
    pub const fn contains(&self, flag: WindowFlag) -> bool {
        self.0 & flag.to_u64() > 0
    }

    /// Add a flag into flags
    pub fn push(&mut self, flag: WindowFlag) {
        self.0 |= flag.to_u64();
    }

    /// Remove a flag from flags
    pub fn remove(&mut self, flag: WindowFlag) {
        self.0 &= !(flag.to_u64());
    }
}

impl FromStr for WindowFlags {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iflags = 0;
        for (i, c) in s.bytes().enumerate() {
            let Some(ch) = WindowFlag::try_from_byte(c) else {
                // return remaining flags
                return Err(s.get(i..).unwrap_or("").into());
            };
            iflags |= ch.to_u64()
        }
        Ok(Self(iflags))
    }
}

impl<'a, const N: usize> From<&[WindowFlag; N]> for WindowFlags {
    fn from(value: &[WindowFlag; N]) -> Self {
        Self::new(value)
    }
}

impl Display for WindowFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut i = 1;
        while i <= WindowFlag::max_u64() {
            let Some(flag) = WindowFlag::try_from_u64(i) else {
                continue;
            };
            if self.contains(flag) {
                write!(f, "{}", flag)?;
            }
            i *= 2;
        }
        Ok(())
    }
}

impl IntoIterator for WindowFlags {
    type Item = WindowFlag;
    type IntoIter = WindowFlagsIter;

    fn into_iter(self) -> WindowFlagsIter {
        WindowFlagsIter(self.0, 0)
    }
}

impl Iterator for WindowFlagsIter {
    type Item = WindowFlag;

    fn next(&mut self) -> Option<Self::Item> {
        while 1 << self.1 <= WindowFlag::max_u64() {
            let r = self.0 & (1 << self.1);
            self.1 += 1;
            if r == 0 {
                continue;
            }
            if let Some(flag) = WindowFlag::try_from_u64(r) {
                return Some(flag);
            }
        }
        return None;
    }
}

#[derive(Clone, Copy, Debug)]
pub enum SurfaceFlag {}

#[derive(Clone, Copy, Debug)]
pub enum Mode {
    Blend,     //Composite
    Overwrite, //Replace
}

#[derive(Clone, Copy, Debug)]
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
            WindowDragKind::Move => write!(f, "M"),
            WindowDragKind::ResizeLeft => write!(f, "L"),
            WindowDragKind::ResizeRight => write!(f, "R"),
            WindowDragKind::ResizeBottom => write!(f, "B"),
            WindowDragKind::ResizeTop => write!(f, "T"),
            WindowDragKind::None => write!(f, "0"),
        }
    }
}

impl WindowDragKind {
    pub const fn try_from_u8(val: i64) -> Option<Self> {
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
    pub const fn to_u8(&self) -> i64 {
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
            WindowDragKind::Move => b"D,M",
            WindowDragKind::ResizeLeft => b"D,L",
            WindowDragKind::ResizeRight => b"D,R",
            WindowDragKind::ResizeBottom => b"D,B",
            WindowDragKind::ResizeTop => b"D,T",
        }
    }
}
impl FromStr for WindowDragKind {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" | "M" => Ok(WindowDragKind::Move),
            "L" => Ok(WindowDragKind::ResizeLeft),
            "R" => Ok(WindowDragKind::ResizeRight),
            "B" => Ok(WindowDragKind::ResizeBottom),
            "T" => Ok(WindowDragKind::ResizeTop),
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
    pub const fn from_i64(val: i64) -> Self {
        match val {
            1 => MediaKind::Text,
            2 => MediaKind::File,
            3 => MediaKind::Uri,
            4 => MediaKind::Data,
            _ => MediaKind::Any,
        }
    }
    pub const fn to_i64(&self) -> i64 {
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
    pub const fn try_from_u8(val: u8) -> Option<Self> {
        match val {
            1 => Some(ClipboardAction::Copy),
            2 => Some(ClipboardAction::Cut),
            3 => Some(ClipboardAction::Paste),
            _ => None,
        }
    }
    pub const fn to_u8(&self) -> u8 {
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
    pub const fn try_from_u8(val: u8) -> Option<Self> {
        match val {
            1 => Some(DragAction::Copy),
            2 => Some(DragAction::Move),
            4 => Some(DragAction::Link),
            0 => Some(DragAction::None),
            _ => None,
        }
    }
    pub const fn to_u8(&self) -> u8 {
        match self {
            DragAction::Copy => 1,
            DragAction::Move => 2,
            DragAction::Link => 4,
            DragAction::None => 0,
        }
    }
}
