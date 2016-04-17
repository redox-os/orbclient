#![crate_name="orbclient"]
#![crate_type="lib"]
#![feature(associated_consts)]
#![feature(const_fn)]
#![feature(box_syntax)]

#![deny(warnings)]

extern crate core;

pub static FONT: &'static [u8] = include_bytes!("../res/unifont.font");

pub use bmp::BmpFile;
pub use color::Color;
pub use event::*;
pub use window::Window;

pub mod bmp;
pub mod color;
pub mod event;

#[cfg(target_os = "redox")]
#[path="orbital/window.rs"]
pub mod window;

#[cfg(not(target_os = "redox"))]
#[path="sdl2/window.rs"]
pub mod window;
