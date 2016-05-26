extern crate orbclient;

use orbclient::{Color, Window, EventOption};

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
    window.sync();

    println!("Data: {}", window.data().len());
    println!("Mut Data: {}", window.data_mut().len());

    'events: loop {
        for event in window.events() {
            match event.to_option() {
                EventOption::Quit(_quit_event) => break 'events,
                event_option => println!("{:?}", event_option)
            }
        }
    }
}
