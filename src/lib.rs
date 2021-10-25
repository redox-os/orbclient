// SPDX-License-Identifier: MIT

#![crate_name = "orbclient"]
#![crate_type = "lib"]
#![cfg_attr(feature = "no_std", feature(alloc))]
#![cfg_attr(feature = "no_std", no_std)]

#[cfg(feature = "no_std")]
extern crate alloc;
#[cfg(not(feature = "no_std"))]
extern crate core;

pub static FONT: &[u8] = include_bytes!("../res/unifont.font");

pub use color::Color;
pub use event::*;
pub use graphicspath::GraphicsPath;
pub use renderer::Renderer;
#[cfg(not(feature = "no_std"))]
pub use sys::{get_display_size, EventIter, Window};

#[cfg(target_arch = "wasm32")]
pub use sys::{animation_loop, log};

#[cfg(not(feature = "no_std"))]
mod blur;
pub mod color;
pub mod event;
pub mod graphicspath;
pub mod renderer;

#[derive(Clone, Copy, Debug)]
pub enum WindowFlag {
    Async,
    Back,
    Front,
    Borderless,
    Resizable,
    Transparent,
    Unclosable,
}

#[derive(Clone, Copy, Debug)]
pub enum Mode {
    Blend,     //Composite
    Overwrite, //Replace
}

#[cfg(all(
    not(feature = "no_std"),
    not(target_arch = "wasm32"),
    target_os = "redox"
))]
#[path = "sys/orbital.rs"]
mod sys;

#[cfg(all(
    not(feature = "no_std"),
    not(target_arch = "wasm32"),
    not(target_os = "redox")
))]
#[path = "sys/sdl2.rs"]
mod sys;

#[cfg(target_arch = "wasm32")]
#[path = "sys/web.rs"]
mod sys;
