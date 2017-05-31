#![crate_name="orbclient"]
#![crate_type="lib"]
#![feature(asm)]
#![feature(collections)]
#![feature(const_fn)]
#![cfg_attr(feature="no_std", no_std)]

#![deny(warnings)]

#[cfg(not(feature="no_std"))]
extern crate core;
extern crate collections;

pub static FONT: &'static [u8] = include_bytes!("../res/unifont.font");

pub use color::Color;
pub use event::*;
#[cfg(not(feature="no_std"))]
pub use imp::{get_display_size, EventIter, Window};
pub use graphicspath::GraphicsPath;
pub use renderer::Renderer;

pub mod color;
pub mod event;
pub mod graphicspath;
pub mod renderer;

#[derive(Clone, Copy, Debug)]
pub enum WindowFlag {
    Async,
    Back,
    Front,
    Resizable,
    Unclosable
}

#[cfg(all(not(feature="no_std"), target_os = "redox"))]
#[path="imp/orbital.rs"]
mod imp;

#[cfg(all(not(feature="no_std"), not(target_os = "redox")))]
#[path="imp/sdl2.rs"]
mod imp;
