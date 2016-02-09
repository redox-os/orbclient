extern crate orbclient;

use orbclient::{Color, EventOption, Window};

use std::thread;

fn main() {
    let mut window = Window::new(-1, -1, 400, 300, "Test").unwrap();
    window.set(Color::rgb(0, 255, 0));
    window.char(16, 16, 'A', Color::rgb(255, 255, 255));
    window.sync();

    'events: loop {
        for event in window.events() {
            println!("{:?}", event.to_option());
            match event.to_option() {
                EventOption::Quit(_) => break 'events,
                _ => ()
            }
        }
    }
}
