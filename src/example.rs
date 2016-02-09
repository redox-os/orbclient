extern crate orbclient;

use std::cmp::max;
use std::env;

use orbclient::{BmpFile, Color, EventOption, Window};

fn main() {
    let path = match env::args().nth(1) {
        Some(arg) => arg,
        None => "res/redox.bmp".to_string(),
    };

    let bmp = BmpFile::from_path(&path);
    let mut window = Window::new(-1,
                                 -1,
                                 max(32, bmp.width() as u32),
                                 max(32, bmp.height() as u32),
                                 &path)
                         .unwrap();
    window.set(Color::rgb(0, 0, 0));
    window.image(0, 0, bmp.width() as u32, bmp.height() as u32, &bmp);
    window.sync();

    loop {
        for event in window.events() {
            println!("{:?}", event.to_option());
            if let EventOption::Quit(_) = event.to_option() {
                return;
            }
        }
    }
}
