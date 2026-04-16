// SPDX-License-Identifier: MIT

use orbclient::{image::ImageRef, rect::Rect, Color, EventOption, Renderer, Window};

const TIMES: usize = 100;

macro_rules! time {
    ($msg:tt, $block: block) => ({
        let _time_instant = ::std::time::Instant::now();
        $block
        let _time_duration = _time_instant.elapsed();
        let _time_fractional = _time_duration.as_secs() as f64
                             + (_time_duration.subsec_nanos() as f64)/1000000000.0;
        println!(
            "{}: {} ms",
            $msg,
            _time_fractional * 1000.0
        );
    });
}

fn main() {
    //let (width, height) = orbclient::get_display_size().unwrap();

    let mut window = Window::new(10, 10, 800, 600, "IMAGE BENCHMARK").unwrap();

    window.set(Color::rgb(255, 255, 255));

    //create image data : a green square
    let data = vec![Color::rgba(100, 200, 10, 3); 412500];
    let mut data2 = vec![Color::rgba(200, 100, 10, 3); 412500];
    let mut data3 = vec![Color::rgba(10, 100, 100, 3); 412500];
    let mut data4 = vec![Color::rgba(10, 100, 200, 3); 800 * 400];

    //draw image benchmarking
    println!("Benchmarking implementations to draw an image on window:");

    time!("image_legacy", {
        for _i in 0..TIMES {
            window.image_legacy(15, 15, 750, 550, &data[..]);
        }
    });

    time!("image_over", {
        for _i in 0..TIMES {
            window.image_over(50, &data4[..]);
        }
    });

    time!("image_fast", {
        for _i in 0..TIMES {
            window.image_fast(20, 20, 750, 550, &data2[..]);
        }
    });

    time!("image_opaque", {
        for _i in 0..TIMES {
            window.image_opaque(30, 30, 750, 550, &data3[..]);
        }
    });

    time!("image_roi_mut_blend", {
        let data2_roi = ImageRef::from_data(750, 550, &mut data2[..]);
        for _i in 0..TIMES {
            ImageRef::from_renderer(&mut window)
                .roi_mut(&Rect::new(40, 40, 750, 550))
                .blend(&data2_roi.roi(&Rect::new(0, 0, 750, 550)));
        }
    });

    time!("image_roi_mut_blit_mask", {
        let data2_roi = ImageRef::from_data(750, 550, &mut data2[..]);
        for _i in 0..TIMES {
            ImageRef::from_renderer(&mut window)
                .roi_mut(&Rect::new(40, 40, 750, 550))
                .blit_mask(&data2_roi.roi(&Rect::new(0, 0, 750, 550)));
        }
    });

    time!("image_roi_mut_blit", {
        let data3_roi = ImageRef::from_data(750, 550, &mut data3[..]);
        for _i in 0..TIMES {
            ImageRef::from_renderer(&mut window)
                .roi_mut(&Rect::new(50, 50, 750, 550))
                .blit(&data3_roi.roi(&Rect::new(0, 0, 750, 550)));
        }
    });

    time!("image_roi_mut_blit_over", {
        let data4_roi = ImageRef::from_data(800, 400, &mut data4[..]);
        for _i in 0..TIMES {
            ImageRef::from_renderer(&mut window)
                // .roi_mut(&Rect::new(10, 120, 790, 400)) // to test blit_over does not trigger
                .roi_mut(&Rect::new(0, 120, 800, 400))
                .blit(&data4_roi.roi(&Rect::new(0, 0, 800, 400)));
        }
    });

    println!("------------------------------------------------");

    window.sync();

    'events: loop {
        for event in window.events() {
            match event.to_option() {
                EventOption::Quit(_quit_event) => break 'events,
                EventOption::Mouse(evt) => println!(
                    "At position {:?} pixel color is : {:?}",
                    (evt.x, evt.y),
                    window.getpixel(evt.x, evt.y)
                ),
                event_option => println!("{:?}", event_option),
            }
        }
    }
}
