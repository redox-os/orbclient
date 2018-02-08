extern crate orbclient;
extern crate time;

use orbclient::{Color, Window, Renderer, EventOption};

const TIMES:i32 = 10;

fn main() {
    //let (width, height) = orbclient::get_display_size().unwrap();

    let mut window = Window::new(10,
                                 10,
                                 800,
                                 600,
                                 "IMAGE BENCHMARK")
                         .unwrap();

    window.set(Color::rgb(255,255,255));
    
    //create image data : a green square
    let data = vec![Color::rgba(100,200,10,20);412500];
    let data2 = vec![Color::rgba(200,100,10,20);412500];

    //draw image benchmarking 
    println!("Benchmarking implementations to draw an image on window:");
    let mut t = time::now();
    
    for _i in 0..TIMES {
        window.image_legacy(10,10,750,550, &data[..]);
    }
    let mut t2 = time::now();
    let dt = (t2-t)/TIMES;
    println!("     image {:?}",dt );

    t = time::now();
    
    for _i in 0..TIMES {
        window.image_fast(40,40,750,550, &data2[..]);
    }
    t2 = time::now();
    let dt2 = (t2-t)/TIMES;
    println!("image_fast {:?}",dt2);
    println!("-------------------------");
    println!("difference {:?}", dt-dt2);
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
