extern crate orbclient;

use orbclient::{Color, Window, EventOption};

fn main() {
    let (width, height) = orbclient::get_display_size().unwrap();

    let mut window = Window::new((width as i32)/4,
                                 (height as i32)/4,
                                 width/2,
                                 height/2,
                                 "TITLE")
                         .unwrap();

    window.set(Color::rgb(255, 255, 255));
    window.sync();

    'events: loop {
        for event in window.events() {
            match event.to_option() {
                EventOption::Quit(_quit_event) => break 'events,
                event_option => println!("{:?}", event_option)
            }
        }
    }
}
