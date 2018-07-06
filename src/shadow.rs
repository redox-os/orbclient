use Renderer;
use Color;
use blur;
use Mode;
use std::cell::Cell;

#[allow(unused)]
pub fn box_shadow<T: Renderer + ?Sized>(renderer: &mut T, x: i32, y: i32, w: u32, h: u32, offset_x: i32, offset_y: i32, blur_radius: u32, box_radius: u32, invert: bool, color: Color) {
    let real_w = w as i32 + (blur_radius * 2) as i32;
    let real_h = h as i32 + (blur_radius * 2) as i32;

    let mut image_mask = ImageMask::new(real_w as u32, real_h as u32);
    image_mask.rounded_rect(blur_radius as i32,blur_radius as i32 ,w,h, box_radius as u32, true, Color::rgb(0,0,0));

    let mut blur_data = image_mask.data.clone();
    blur::gauss_blur(&mut blur_data, real_w as u32, real_h as u32, blur_radius as f32 / 3.0);


    let mut real_start_y = y - blur_radius as i32 + offset_y;
    let mut real_start_x = x - blur_radius as i32 + offset_x;
    let real_end_y = y + h as i32 + blur_radius as i32 + offset_y;
    let real_end_x = x + w as i32 + blur_radius as i32 + offset_x;


    for yb in real_start_y .. real_end_y{
        for xb in real_start_x .. real_end_x {
            let real_x = xb - real_start_x;
            let real_y = yb - real_start_y;

            let mut draw = true;
            if !(xb < x || yb < y || yb >= y + h as i32 || xb >= x + w as i32) {
                let data_index = ((real_y + offset_y) * real_w + real_x + offset_x);
                if data_index > 0 && data_index < image_mask.data.len() as i32 {
                    let mask_color = image_mask.data[data_index as usize];
                    if mask_color.r() == 0 {
                        draw = false;
                    }
                }
            }

            if (invert) { draw = !draw; }
            if draw {
                let data_index = ((real_y) * real_w + real_x);
                let c = blur_data[data_index as usize];
                let mut alpha: u8 = if color.a() < 255 - c.r() { color.a() } else { 255 - c.r() };
                if (invert) { alpha = if color.a() < c.r() { color.a() } else { c.r() }; }
                let col = Color::rgba(color.r(), color.g(), color.b(), alpha);
                renderer.pixel(xb, yb, col);
            }
        }
    }
}

pub struct ImageMask {
    pub width: u32,
    pub height: u32,
    pub data: Vec<Color>,
    mode: Cell<Mode>,
}

impl ImageMask {
    pub fn new(width: u32, height: u32) -> Self {
        let size: u64 = (width * height) as u64;
        ImageMask {
            width: width,
            height: height,
            data: vec![Color::rgb(255,255,255); size as usize],
            mode: Cell::new(Mode::Overwrite)
        }
    }
}

impl Renderer for ImageMask {
    /// Get width
    fn width(&self) -> u32 {
        self.width
    }

    /// Get height
    fn height(&self) -> u32 {
        self.height
    }

    /// Access pixel buffer
    fn data(&self) -> &[Color] {
        &self.data
    }

    /// Access pixel buffer mutably
    fn data_mut(&mut self) -> &mut [Color] {
        &mut self.data
    }

    /// Flip the window buffer
    fn sync(&mut self) -> bool {
        true
    }

    /// Set/get mode
    fn mode(&self) -> &Cell<Mode> {
        &self.mode
    }
}
