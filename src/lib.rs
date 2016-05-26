#![crate_name="orbclient"]
#![crate_type="lib"]
#![feature(const_fn)]

#![deny(warnings)]

extern crate core;

pub static FONT: &'static [u8] = include_bytes!("../res/unifont.font");

pub use color::Color;
pub use event::*;
pub use window::Window;

pub mod color;
pub mod event;

#[cfg(target_os = "redox")]
#[path="orbital/window.rs"]
pub mod window;

#[cfg(not(target_os = "redox"))]
#[path="sdl2/window.rs"]
pub mod window;
