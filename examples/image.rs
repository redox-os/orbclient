extern crate orbclient;
extern crate time;

use orbclient::{Color, Window, Renderer, EventOption, GraphicsPath};

fn main() {
    let (width, height) = orbclient::get_display_size().unwrap();

    let mut window = Window::new((width as i32)/4,
                                 (height as i32)/4,
                                 width/2,
                                 height/2,
                                 "TITLE")
                         .unwrap();

    let (win_w, win_h) = (width/2, height/2);
    
    //create image data : a green square
    let data = vec![Color::rgb(100,200,10);360000];

    //draw image sequentially
    let mut t = time::now();
    for i in 1..100 {
        window.image(10,10,600,600, &data[..]);
    }
    println!("{:?}", time::now()-t);
    //draw image parallelizing
    t = time::now();
    for i in 1..100 {
        window.image_par(30,30,600,600, &data[..]);
    }
    println!("{:?}", time::now()-t);
    
    t = time::now();
    for i in 1..100 {
        window.image_blit(30,30,600,600, &data[..]);
    }
    println!("{:?}", time::now()-t);
    
    window.sync();

    'events: loop {
        for event in window.events() {
            match event.to_option() {
                EventOption::Quit(_quit_event) => break 'events,
                EventOption::Mouse(evt) => println!("At position {:?} pixel color is : {:?}",(evt.x,evt.y), window.getpixel(evt.x,evt.y )),
                event_option => println!("{:?}", event_option)
            }
        }
    }
}
