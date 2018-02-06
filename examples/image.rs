extern crate orbclient;
extern crate time;

use orbclient::{Color, Window, Renderer, EventOption};

const TIMES:i32 = 10;

fn main() {
    let (width, height) = orbclient::get_display_size().unwrap();

    let mut window = Window::new((width as i32)/4,
                                 (height as i32)/4,
                                 width/2,
                                 height/2,
                                 "IMAGE BENCHMARK")
                         .unwrap();

    window.set(Color::rgb(255,255,255));
    
    //create image data : a green square
    let data = vec![Color::rgba(100,200,10,2);200000];
    let data2 = vec![Color::rgba(200,100,10,2);200000];

    //draw image sequentially
    let mut t = time::now();
    for _i in 1..TIMES {
        window.image(10,10,500,400, &data[..]);
    }
    println!("image {:?}", time::now()-t);
    //draw image parallelizing
    t = time::now();
    for _i in 1..TIMES {
        window.image_par(20,20,500,400, &data[..]);
    }
    println!("image_par {:?}", time::now()-t);
    
    t = time::now();
    for _i in 1..TIMES {
        window.image_fast(30,30,500,400, &data[..]);
    }
    println!("image_fast {:?}", time::now()-t);
    
    t = time::now();
    
    for _i in 1..TIMES {
        window.image_veryfast(40,40,500,400, &data2[..]);
    }
    println!("image_veryfast{:?}", time::now()-t);

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
