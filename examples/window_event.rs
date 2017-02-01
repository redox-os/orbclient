extern crate orbclient;

use orbclient::{Color, Window, Renderer, EventOption};

fn main() {
    let (width, height) = orbclient::get_display_size().unwrap();

    let mut window = Window::new((width as i32)/4,
                                 (height as i32)/4,
                                 width/2,
                                 height/2,
                                 "Window related events")
                         .unwrap();

    draw_things(&mut window);

    'events: loop {
        for event in window.events() {
            match event.to_option() {
                EventOption::Quit(_quit_event) => break 'events,
                EventOption::Resized(resized_event) => {
                    draw_things(&mut window);
                    println!("{:?}", resized_event);
                }
                _ => (),
            }
        }
    }
}

fn draw_things(window: &mut Window) {
    use std::cmp::max;

    let w = max(window.width(), 50);
    let h = max(window.height(), 50);

    window.rect(0, 0, w, h, Color::rgb(0, 0, 255));

    window.rect(0, 0, 25, 25, Color::rgb(0, 255, 0));
    window.rect((w - 25) as i32, 0, 25, 25, Color::rgb(0, 255, 255));
    window.rect(0, (h - 25) as i32, 25, 25, Color::rgb(255, 255, 0));
    window.rect((w - 25) as i32, (h - 25) as i32, 25, 25, Color::rgb(255, 0, 255));

    window.rect((w/2 - 12) as i32, (h/2 - 3) as i32, 25, 6, Color::rgb(255, 0, 0));
    window.rect((w/2 - 3) as i32, (h/2 - 12) as i32, 6, 25, Color::rgb(255, 0, 0));

    window.sync();
}
