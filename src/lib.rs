#![crate_name="orbclient"]
#![crate_type="lib"]
#![feature(alloc)]
#![feature(asm)]
#![feature(const_fn)]
#![feature(const_ptr_null_mut)]
#![cfg_attr(feature="no_std", no_std)]

#![deny(warnings)]

extern crate alloc;
#[cfg(not(feature="no_std"))]
extern crate core;

pub static FONT: &'static [u8] = include_bytes!("../res/unifont.font");

pub use color::Color;
pub use event::*;
#[cfg(not(feature="no_std"))]
pub use sys::{get_display_size, EventIter, Window};
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
#[path="sys/orbital.rs"]
mod sys;

#[cfg(all(not(feature="no_std"), not(target_os = "redox")))]
#[path="sys/sdl2.rs"]
mod sys;
