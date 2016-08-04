extern crate sdl2;

use std::ptr;
use std::sync::atomic::{AtomicUsize, ATOMIC_USIZE_INIT, Ordering};

pub use self::display::*;
pub use self::window::*;

mod display;
mod window;

pub static SDL_USAGES: AtomicUsize = ATOMIC_USIZE_INIT;
/// SDL2 Context
pub static mut SDL_CTX: *mut sdl2::Sdl = ptr::null_mut();
/// Video Context
pub static mut VIDEO_CTX: *mut sdl2::VideoSubsystem = ptr::null_mut();
/// Event Pump
pub static mut EVENT_PUMP: *mut sdl2::EventPump = ptr::null_mut();

//Call this when the CTX needs to be used is created
#[inline]
unsafe fn init() {
    if SDL_USAGES.fetch_add(1, Ordering::Relaxed) == 0 {
        SDL_CTX = Box::into_raw(Box::new(sdl2::init().unwrap()));
        VIDEO_CTX = Box::into_raw(Box::new((&mut *SDL_CTX).video().unwrap()));
        EVENT_PUMP = Box::into_raw(Box::new((&mut *SDL_CTX).event_pump().unwrap()));
    }
}
