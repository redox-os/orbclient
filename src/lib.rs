#![crate_name="orbclient"]
#![crate_type="lib"]
#![feature(asm)]
#![cfg_attr(feature="no_std", feature(alloc))]
#![cfg_attr(feature="no_std", no_std)]

#![deny(warnings)]

#[cfg(feature="no_std")]
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
#[cfg(not(feature = "no_std"))]
mod blur;

#[derive(Clone, Copy, Debug)]
pub enum WindowFlag {
    Async,
    Back,
    Front,
    Borderless,
    Resizable,
    Unclosable
}

#[derive(Clone, Copy, Debug)]
pub enum Mode {
    Blend,      //Composite
    Overwrite   //Replace
}

#[cfg(all(not(feature="no_std"), target_os = "redox"))]
#[path="sys/orbital.rs"]
mod sys;

#[cfg(all(not(feature="no_std"), not(target_os = "redox")))]
#[path="sys/sdl2.rs"]
mod sys;
