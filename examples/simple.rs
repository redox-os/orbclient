extern crate orbclient;

use orbclient::{Color, Window, Renderer, EventOption};

fn main() {
    let (width, height) = orbclient::get_display_size().unwrap();

    let mut window = Window::new((width as i32)/4,
                                 (height as i32)/4,
                                 width/2,
                                 height/2,
                                 "TITLE")
                         .unwrap();

    let (win_w, win_h) = (width/2, height/2);
    // top left -> bottom rigth
    window.linear_gradient(0, 0, win_w/3, win_h, 0, 0,  (win_w/3) as i32, (win_h/2) as i32, Color::rgb(128,128,128), Color::rgb(255,255,255));
    // horizontal gradient
    window.linear_gradient((win_w/3) as i32, 0, win_w/3, win_h, (win_w/3) as i32, 0,  (2*win_w/3) as i32, 0, Color::rgb(128,255,255), Color::rgb(255,255,255));
    // vertical gradient
    window.linear_gradient((2*win_w/3) as i32, 0, win_w/3, win_h, (2*win_w/3) as i32, 0,  (2*win_w/3) as i32, win_h as i32, Color::rgb(0,128,0), Color::rgb(255,255,255));
    window.arc(100, 100, -25, 1 << 0 | 1 << 2, Color::rgb(0, 0, 255));
    window.arc(100, 100, -25, 1 << 1 | 1 << 3, Color::rgb(0, 255, 255));
    window.arc(100, 100, -25, 1 << 4 | 1 << 6, Color::rgb(255, 0, 255));
    window.arc(100, 100, -25, 1 << 5 | 1 << 7, Color::rgb(255, 255, 0));
    window.circle(100, 100, 25, Color::rgb(0, 0, 0));
    window.circle(100, 101, -25, Color::rgb(0, 255, 0));
    window.line(0, 0, 200, 200, Color::rgb(255, 0, 0));
    window.line(0, 200, 200, 0, Color::rgb(0, 255, 0));
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
