#![crate_name="orbclient"]
#![crate_type="lib"]
#![feature(alloc)]
#![feature(asm)]
#![feature(const_fn)]
#![cfg_attr(all(not(feature="no_std"), not(target_os = "redox")), feature(const_ptr_null_mut))]
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
pub use renderer::Renderer;

pub mod color;
pub mod event;
pub mod renderer;

/// Canvas components
pub use point::Point;
pub use edge::Edge;
pub use pathbuilder::PathBuilder;
pub use matrix::Matrix;
pub use canvas::Canvas;
pub use canvaspaintstate::CanvasPaintState;

#[path="canvas/point.rs"]
pub mod point;
#[path="canvas/edge.rs"]
pub mod edge;
#[path="canvas/pathbuilder.rs"]
pub mod pathbuilder;
#[path="canvas/matrix.rs"]
pub mod matrix;
#[path="canvas/canvas.rs"]
pub mod canvas;
#[path="canvas/canvaspaintstate.rs"]
pub mod canvaspaintstate;


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
