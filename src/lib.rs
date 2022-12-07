// SPDX-License-Identifier: MIT

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

cfg_if::cfg_if! {
    if #[cfg(all(feature = "std", feature = "sdl", not(target_arch = "wasm32"), not(target_os = "redox")))] {
        #[path = "sys/sdl2.rs"]
        mod sys;
        pub use sys::{get_display_size, EventIter, Window};
    } else if #[cfg(all(feature = "std", not(target_arch = "wasm32"), target_os = "redox"))] {
        #[path = "sys/orbital.rs"]
        mod sys;
        pub use sys::{get_display_size, EventIter, Window};
    } else if #[cfg(target_arch = "wasm32")] {
        #[path = "sys/web.rs"]
        mod sys;
        pub use sys::{animation_loop, log, get_display_size, EventIter, Window};
    }
}

#[cfg(feature = "unifont")]
pub static FONT: &[u8] = include_bytes!("../res/unifont.font");

pub use color::Color;
pub use event::*;
pub use graphicspath::GraphicsPath;
pub use renderer::Renderer;

#[cfg(feature = "std")]
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
