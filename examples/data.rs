extern crate orbclient;

use orbclient::{Color, Window};

use std::thread;

fn main() {

    let width = 600;
    let height = 600;

    let mut window = Window::new(-1,
                                 -1,
                                 width,
                                 height,
                                 "TITLE")
                         .unwrap();

    window.set(Color::rgb(255, 255, 255));

    println!("Data: {}", window.data().len());
    println!("Mut Data: {}", window.data_mut().len());
}
