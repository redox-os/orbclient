#![crate_name="orbclient"]
#![crate_type="lib"]
#![cfg_attr(target_os = "redox", feature(asm))]
#![cfg_attr(target_os = "redox", feature(file_path))]
#![feature(const_fn)]

#![deny(warnings)]

extern crate core;

pub static FONT: &'static [u8] = include_bytes!("../res/unifont.font");

pub use color::Color;
pub use event::*;
pub use imp::*;

pub mod color;
pub mod event;

#[cfg(target_os = "redox")]
#[path="orbital/mod.rs"]
pub mod imp;

#[cfg(not(target_os = "redox"))]
#[path="sdl2/mod.rs"]
pub mod imp;
