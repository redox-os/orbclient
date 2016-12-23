#![crate_name="orbclient"]
#![crate_type="lib"]
#![feature(asm)]
#![feature(const_fn)]

#![deny(warnings)]

extern crate core;

pub static FONT: &'static [u8] = include_bytes!("../res/unifont.font");

pub use color::Color;
pub use event::*;
pub use imp::*;
pub use renderer::Renderer;
pub use graphicspath::GraphicsPath;

pub mod color;
pub mod event;
pub mod renderer;
pub mod graphicspath;

#[cfg(target_os = "redox")]
#[path="orbital/mod.rs"]
pub mod imp;

#[cfg(not(target_os = "redox"))]
#[path="sdl2/mod.rs"]
pub mod imp;
