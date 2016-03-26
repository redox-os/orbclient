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

    let border_limit = (width / 2) as i32;

    for i in 0..border_limit {
        window.set_border_top(i, Color::rgb(0,0,0), 1);
        window.set_border_left(i, Color::rgb(0,0,0), 1);
        window.set_border_bottom(i, Color::rgb(0,0,0), 1);
        window.set_border_right(i, Color::rgb(0,0,0), 1);
        window.sync();
        thread::sleep_ms(5);
    }

}
