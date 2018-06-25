extern crate orbclient;

use orbclient::{Color, Window, Renderer, EventOption, Canvas, Mode};


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
    window.circle(220, 220, -100, Color::rgba(128, 128,128,80));
    window.wu_circle(150, 220, 100, Color::rgba(255,0,0,255));
    window.line(0, 0, 200, 200, Color::rgb(255, 0, 0));
    window.line(0, 200, 200, 0, Color::rgb(128, 255, 0));
    // vertical and horizontal line test
    window.line(100, 0, 100, 200, Color::rgb(0, 0, 255));
    window.line(0, 100, 200, 100, Color::rgb(255, 255, 0));
    window.wu_line(100, 220, 400, 250, Color::rgba(255,0,0,255));
    window.line(100, 230, 400, 260, Color::rgba(255,0,0,255));

    //Init a new canvas
    let mut canvas = Canvas::new(500.0,500.0);

    //Transform the canvas
    canvas.transform(3.0,0.0,0.0, 3.0,0.0,200.0);
    //canvas.transform(0.71,-0.71,0.71, 0.71,0.0,0.0);

    //Set canvas fill and stroke style
    canvas.set_fill_style(Color::rgba(255,255,255,200));
    canvas.set_stroke_style(Color::rgba(200,200,200,255));

    //Create the polygon
    canvas.begin_path();
    canvas.move_to(48.355,17.922);
    canvas.bezier_curve_to(52.06,20.245,54.658,24.176,55.131,28.739);
    canvas.bezier_curve_to(56.642,29.445,58.319,29.851,60.097,29.851);
    canvas.bezier_curve_to(66.588,29.851,71.849,24.59,71.849,18.1);
    canvas.bezier_curve_to(71.849,11.609,66.588,6.348,60.097,6.348);
    canvas.bezier_curve_to(53.668,6.35,48.453,11.517,48.355,17.922);
    canvas.close_path();
    canvas.move_to(40.656,41.984);
    canvas.bezier_curve_to(47.147,41.984,52.408,36.722,52.408,30.232);
    canvas.bezier_curve_to(52.408,23.742,47.146,18.481,40.656,18.481);
    canvas.bezier_curve_to(34.166,18.481,28.902,23.743,28.902,30.233);
    canvas.bezier_curve_to(28.902,36.723,34.166,41.984,40.656,41.984);
    canvas.close_path();
    canvas.move_to(45.641,42.785);
    canvas.line_to(35.669,42.785);
    canvas.bezier_curve_to(27.372,42.785,20.622,49.536,20.622,57.833);
    canvas.line_to(20.622,70.028);
    canvas.line_to(20.653,70.219);
    canvas.line_to(21.493,70.482);
    canvas.bezier_curve_to(29.411,72.956,36.290,73.781,41.952,73.781);
    canvas.bezier_curve_to(53.011,73.781,59.421,70.628,59.816,70.427);
    canvas.line_to(60.601,70.03);
    canvas.line_to(60.685,70.03);
    canvas.line_to(60.685,57.833);
    canvas.bezier_curve_to(60.688,49.536,53.938,42.785,45.641,42.785);
    canvas.close_path();
    canvas.move_to(65.084,30.653);
    canvas.line_to(55.189,30.653);
    canvas.bezier_curve_to(55.082,34.612,53.392,38.177,50.719,40.741);
    canvas.bezier_curve_to(58.094,42.934,63.49,49.773,63.49,57.851);
    canvas.line_to(63.49,61.609);
    canvas.bezier_curve_to(73.26,61.251,78.89,58.482,79.261,58.296);
    canvas.line_to(80.046,57.898);
    canvas.line_to(80.13,57.898);
    canvas.line_to(80.13,45.699);
    canvas.bezier_curve_to(80.13,37.403,73.38,30.653,65.084,30.653);
    canvas.close_path();
    canvas.move_to(20.035,29.853);
    canvas.bezier_curve_to(22.334,29.853,24.473,29.182,26.285,28.039);
    canvas.bezier_curve_to(26.861,24.282,28.875,20.999,31.752,18.763);
    canvas.bezier_curve_to(31.764,18.543,31.785,18.325,31.785,18.103);
    canvas.bezier_curve_to(31.785,11.612,26.523,6.351,20.035,6.351);
    canvas.bezier_curve_to(13.543,6.351,8.283,11.612,8.283,18.103);
    canvas.bezier_curve_to(8.283,24.591,13.543,29.853,20.035,29.853);
    canvas.close_path();
    canvas.move_to(30.589,40.741);
    canvas.bezier_curve_to(27.929,38.19,26.245,34.644,26.122,30.709);
    canvas.bezier_curve_to(25.755,30.682,25.392,30.653,25.018,30.653);
    canvas.line_to(15.047,30.653);
    canvas.bezier_curve_to(6.75,30.653,0.0,37.403,0.0,45.699);
    canvas.line_to(0.0,57.896);
    canvas.line_to(0.031,58.084);
    canvas.line_to(0.871,58.349);
    canvas.bezier_curve_to(7.223,60.332,12.892,61.246,17.816,61.534);
    canvas.line_to(17.816,57.851);
    canvas.bezier_curve_to(17.818,49.773,23.212,42.936,30.589,40.74);

    //Fill the polygon and draw a stroke
    canvas.fill();
    canvas.stroke();



    window.image_fast(500,0,500,500, unsafe { &canvas.data });

    window.char(200, 200, '═', Color::rgb(0, 0, 0));
    window.char(208, 200, '═', Color::rgb(0, 0, 0));

    // testing for non existent x,y position : does not panic but returns Color(0,0,0,0)
    let _non_existent_pixel = window.getpixel(width as i32 +10,height as i32 +10);

    // testing PartialEq for Color
    if Color::rgb(11,2,3) == Color::rgba(1,2,3,100) {
        println!("Testing colors: they are the same!")
    }else{
        println!("Testing colors: they are NOT the same!")
    }

    //Draw a transparent rectangle over window content
    // default mode is Blend 
    window.rect(250, 200, 80, 80, Color::rgba(100,100,100,100));
    
    //Draw an opaque rectangle replacing window content
    window.mode().set(Mode::Overwrite); // set window drawing mode to Overwrite from now on 
    window.rect(300, 220, 80, 80, Color::rgb(100,100,100));
    
    //Draw a hole in the window replacing alpha channel (Only in Orbital, not in SDL2)
    window.rect(300, 100, 80, 80, Color::rgba(10,10,10,1));

    //Draw a transparent rectangle over window content
    window.mode().set(Mode::Blend); //set mode to Blend fron now on
    window.rect(200, 230, 80, 80, Color::rgba(100,100,100,100));

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
