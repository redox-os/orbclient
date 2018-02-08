use core::cmp;

use FONT;
use color::Color;
use graphicspath::GraphicsPath;
use graphicspath::PointType;

#[cfg(target_arch = "x86")]
#[inline(always)]
#[cold]
pub unsafe fn fast_set32(dst: *mut u32, src: u32, len: usize) {
    asm!("cld
        rep stosd"
        :
        : "{edi}"(dst as usize), "{eax}"(src), "{ecx}"(len)
        : "cc", "memory", "edi", "ecx"
        : "intel", "volatile");
}

#[cfg(target_arch = "x86_64")]
#[inline(always)]
#[cold]
pub unsafe fn fast_set32(dst: *mut u32, src: u32, len: usize) {
    asm!("cld
        rep stosd"
        :
        : "{rdi}"(dst as usize), "{eax}"(src), "{rcx}"(len)
        : "cc", "memory", "rdi", "rcx"
        : "intel", "volatile");
}

pub trait Renderer {
    /// Get width
    fn width(&self) -> u32;

    /// Get height
    fn height(&self) -> u32;

    /// Access the pixel buffer
    fn data(&self) -> &[Color];

    /// Access the pixel buffer mutably
    fn data_mut(&mut self) -> &mut [Color];

    /// Flip the buffer
    fn sync(&mut self) -> bool;

    /// Draw a pixel
    fn pixel(&mut self, x: i32, y: i32, color: Color) {
        let w = self.width();
        let h = self.height();
        let data = self.data_mut();

        if x >= 0 && y >= 0 && x < w as i32 && y < h as i32 {
            let new = color.data;

            let alpha = (new >> 24) & 0xFF;
            if alpha > 0 {
                let old = unsafe{ &mut data[y as usize * w as usize + x as usize].data};
                if alpha >= 255 {
                    *old = new;
                } else {
                    let n_r = (((new >> 16) & 0xFF) * alpha) >> 8;
                    let n_g = (((new >> 8) & 0xFF) * alpha) >> 8;
                    let n_b = ((new & 0xFF) * alpha) >> 8;

                    let n_alpha = 255 - alpha;
                    let o_a = (((*old >> 24) & 0xFF) * n_alpha) >> 8;
                    let o_r = (((*old >> 16) & 0xFF) * n_alpha) >> 8;
                    let o_g = (((*old >> 8) & 0xFF) * n_alpha) >> 8;
                    let o_b = ((*old & 0xFF) * n_alpha) >> 8;

                    *old = ((o_a << 24) | (o_r << 16) | (o_g << 8) | o_b) + ((alpha << 24) | (n_r << 16) | (n_g << 8) | n_b);
                }
            }
        }
    }

    /// Draw a piece of an arc. Negative radius will fill in the inside
    fn arc(&mut self, x0: i32, y0: i32, radius: i32, parts: u8, color: Color) {
        let mut x = radius.abs();
        let mut y = 0;
        let mut err = 0;

        while x >= y {
            if radius < 0 {
                if parts & 1 << 0 != 0 { self.rect(x0 - x, y0 + y, x as u32, 1, color); }
                if parts & 1 << 1 != 0 { self.rect(x0, y0 + y, x as u32 + 1, 1, color); }
                if parts & 1 << 2 != 0 { self.rect(x0 - y, y0 + x, y as u32, 1, color); }
                if parts & 1 << 3 != 0 { self.rect(x0, y0 + x, y as u32 + 1, 1, color); }
                if parts & 1 << 4 != 0 { self.rect(x0 - x, y0 - y, x as u32, 1, color); }
                if parts & 1 << 5 != 0 { self.rect(x0, y0 - y, x as u32 + 1, 1, color); }
                if parts & 1 << 6 != 0 { self.rect(x0 - y, y0 - x, y as u32, 1, color); }
                if parts & 1 << 7 != 0 { self.rect(x0, y0 - x, y as u32 + 1, 1, color); }
            } else if radius == 0 {
                self.pixel(x0, y0, color);
            } else {
                if parts & 1 << 0 != 0 { self.pixel(x0 - x, y0 + y, color); }
                if parts & 1 << 1 != 0 { self.pixel(x0 + x, y0 + y, color); }
                if parts & 1 << 2 != 0 { self.pixel(x0 - y, y0 + x, color); }
                if parts & 1 << 3 != 0 { self.pixel(x0 + y, y0 + x, color); }
                if parts & 1 << 4 != 0 { self.pixel(x0 - x, y0 - y, color); }
                if parts & 1 << 5 != 0 { self.pixel(x0 + x, y0 - y, color); }
                if parts & 1 << 6 != 0 { self.pixel(x0 - y, y0 - x, color); }
                if parts & 1 << 7 != 0 { self.pixel(x0 + y, y0 - x, color); }
            }

            y += 1;
            err += 1 + 2*y;
            if 2*(err-x) + 1 > 0 {
                x -= 1;
                err += 1 - 2*x;
            }
        }
    }

    /// Draw a circle. Negative radius will fill in the inside
    fn circle(&mut self, x0: i32, y0: i32, radius: i32, color: Color) {
        let mut x = radius.abs();
        let mut y = 0;
        let mut err = -radius.abs();
        
        match radius {
            radius if radius > 0 => {
                err = 0;
                while x >= y {
                    self.pixel(x0 - x, y0 + y, color);
                    self.pixel(x0 + x, y0 + y, color);
                    self.pixel(x0 - y, y0 + x, color);
                    self.pixel(x0 + y, y0 + x, color);
                    self.pixel(x0 - x, y0 - y, color);
                    self.pixel(x0 + x, y0 - y, color);
                    self.pixel(x0 - y, y0 - x, color);
                    self.pixel(x0 + y, y0 - x, color);
                
                    y += 1;
                    err += 1 + 2*y;
                    if 2*(err-x) + 1 > 0 {
                        x -= 1;
                        err += 1 - 2*x;
                    }
                }      
            },
            
            radius if radius < 0 => {
                while x >= y {
                    let lasty = y;
                    err +=y;
                    y +=1;
                    err += y;
                    self.line4points(x0,y0,x,lasty,color);
                    if err >=0 {
                        if x != lasty{
                           self.line4points(x0,y0,lasty,x,color);
                        }
                        err -= x;
                        x -= 1;
                        err -= x;
                    }
                }

                },
                     _ => {
                            self.pixel(x0, y0, color);
                            
                        },
        }
    }
    
    fn line4points(&mut self, x0: i32, y0: i32, x: i32, y: i32, color: Color){
        //self.line(x0 - x, y0 + y, (x+x0), y0 + y, color);
        self.rect(x0 - x, y0 + y, x as u32 * 2 + 1, 1, color);
        if y != 0 {
            //self.line(x0 - x, y0 - y, (x+x0), y0-y , color);
            self.rect(x0 - x, y0 - y, x as u32 * 2 + 1, 1, color);
        }
    }

    /// Draw a line
    fn line(&mut self, argx1: i32, argy1: i32, argx2: i32, argy2: i32, color: Color) {
        let mut x = argx1;
        let mut y = argy1;

        let dx = if argx1 > argx2 { argx1 - argx2 } else { argx2 - argx1 };
        let dy = if argy1 > argy2 { argy1 - argy2 } else { argy2 - argy1 };

        let sx = if argx1 < argx2 { 1 } else { -1 };
        let sy = if argy1 < argy2 { 1 } else { -1 };

        let mut err = if dx > dy { dx } else {-dy} / 2;
        let mut err_tolerance;

        loop {
            self.pixel(x, y, color);

            if x == argx2 && y == argy2 { break };

            err_tolerance = 2 * err;

            if err_tolerance > -dx { err -= dy; x += sx; }
            if err_tolerance < dy { err += dx; y += sy; }
        }
    }

    fn lines(&mut self, points: &[[i32; 2]], color: Color) {
        if points.len() == 0 {
            // when no points given, do nothing
        } else if points.len() == 1 {
            self.pixel(points[0][0], points[0][1], color);
        } else {
            for i in 0..points.len() - 1 {
                self.line(points[i][0], points[i][1], points[i+1][0], points[i+1][1], color);
            }
        }
    }

    /// Draw a path (GraphicsPath)
    fn draw_path_stroke(&mut self, graphicspath: GraphicsPath, color: Color) {
        let mut x: i32 = 0;
        let mut y: i32 = 0;

        for point in graphicspath.points {
            match point.2 {
                PointType::Connect => {self.line(x,y, point.0, point.1, color)},
                _ => {},
            }
            x = point.0;
            y = point.1;
        }
    }

    /// Draw a character, using the loaded font
    fn char(&mut self, x: i32, y: i32, c: char, color: Color) {
        let mut offset = (c as usize) * 16;
        for row in 0..16 {
            let row_data;
            if offset < FONT.len() {
                row_data = FONT[offset];
            } else {
                row_data = 0;
            }

            for col in 0..8 {
                let pixel = (row_data >> (7 - col)) & 1;
                if pixel > 0 {
                    self.pixel(x + col as i32, y + row as i32, color);
                }
            }
            offset += 1;
        }
    }

    /// Set entire window to a color
    fn set(&mut self, color: Color) {
        let data = self.data_mut();
        unsafe {
            fast_set32(data.as_mut_ptr() as *mut u32, color.data, data.len());
        }
    }

    /// Sets the whole window to black
    fn clear(&mut self) {
        self.set(Color::rgb(0,0,0));
    }

    /// Draw rectangle
    fn rect(&mut self, x: i32, y: i32, w: u32, h: u32, color: Color) {
        let self_w = self.width();
        let self_h = self.height();

        let start_y = cmp::max(0, cmp::min(self_h as i32 - 1, y));
        let end_y = cmp::max(start_y, cmp::min(self_h as i32, y + h as i32));

        let start_x = cmp::max(0, cmp::min(self_w as i32 - 1, x));
        let len = cmp::max(start_x, cmp::min(self_w as i32, x + w as i32)) - start_x;

        let alpha = (color.data >> 24) & 0xFF;
        if alpha > 0 {
            if alpha >= 255 {
                let data = self.data_mut();
                for y in start_y..end_y {
                    unsafe {
                        fast_set32(data.as_mut_ptr().offset((y * self_w as i32 + start_x) as isize) as *mut u32, color.data, len as usize);
                    }
                }
            } else {
                for y in start_y..end_y {
                    for x in start_x..start_x + len {
                        self.pixel(x, y, color);
                    }
                }
            }
        }
    }

    /// Display an image
    fn image(&mut self, start_x: i32, start_y: i32, w: u32, h: u32, data: &[Color]) {
        //check if image is inside window 
        if (w + start_x as u32) > self.width() && h > 100 {
            self.image_legacy(start_x, start_y, w, h, data);
        }else{
            self.image_fast(start_x, start_y, w, h, data);
        }
    }


    // TODO: Improve speed
    fn image_legacy(&mut self, start_x: i32, start_y: i32, w: u32, h: u32, data: &[Color]) {
        let mut i = 0;
        for y in start_y..start_y + h as i32 {
            for x in start_x..start_x + w as i32 {
                if i < data.len() {
                    self.pixel(x, y, data[i])
                }
                i += 1;
            }
        }
    }

    // Speed improved, but image has to be all inside of window boundary
    fn image_fast (&mut self, start_x: i32, start_y: i32, w: u32, _h: u32, image_data: &[Color]) {
        let window_w = self.width() as usize;
        let window_len = self.data().len();
        let data = self.data_mut();
        let w = w as usize;
        let start_x = start_x as usize;
        let start_y = start_y as usize;

        for i in 0..image_data.len() {
            let y0 = i/w;
            let y = y0 + start_y;
            let x = start_x + i-(y0*w) ;
            let window_index = y * window_w + x;
            if window_index < window_len {
                let new = image_data[i].data;
                let alpha = (new >> 24) & 0xFF;
                if alpha > 0 {
                    let old = unsafe{ &mut data[window_index].data};
                    if alpha >= 255 {
                        *old = new;
                    } else {
                        let n_r = (((new >> 16) & 0xFF) * alpha) >> 8;
                        let n_g = (((new >> 8) & 0xFF) * alpha) >> 8;
                        let n_b = ((new & 0xFF) * alpha) >> 8;

                        let n_alpha = 255 - alpha;
                        let o_a = (((*old >> 24) & 0xFF) * n_alpha) >> 8;
                        let o_r = (((*old >> 16) & 0xFF) * n_alpha) >> 8;
                        let o_g = (((*old >> 8) & 0xFF) * n_alpha) >> 8;
                        let o_b = ((*old & 0xFF) * n_alpha) >> 8;

                        *old = ((o_a << 24) | (o_r << 16) | (o_g << 8) | o_b) + ((alpha << 24) | (n_r << 16) | (n_g << 8) | n_b);
                    }
                }
            }
            
        }
    }

    /// Draw a linear gradient in a rectangular region
    #[cfg(not(feature="no_std"))]
    fn linear_gradient(&mut self, rect_x: i32, rect_y: i32, rect_width: u32, rect_height:u32, start_x: i32, start_y: i32, end_x: i32, end_y: i32, start_color: Color, end_color: Color) {
        if (start_x == end_x) && (start_y == end_y) {
            // Degenerate gradient
            self.rect(rect_x, rect_y, rect_width, rect_height, start_color);
        } else if start_x == end_x {
            // Vertical gradient
            for y in rect_y..(rect_y + rect_height as i32) {
                let proj = (y as f64 - start_y as f64) / (end_y as f64 - start_y as f64);
                let scale = if proj < 0.0 { 0.0 } else if proj > 1.0 { 1.0 } else { proj };
                let color = Color::interpolate(start_color, end_color, scale);
                self.line(rect_x, y, rect_x + rect_width as i32 - 1, y, color);
            }
        } else if start_y == end_y {
            // Horizontal gradient
            for x in rect_x..(rect_x + rect_width as i32) {
                let proj = (x as f64 - start_x as f64) / (end_x as f64 - start_x as f64);
                let scale = if proj < 0.0 { 0.0 } else if proj > 1.0 { 1.0 } else { proj };
                let color = Color::interpolate(start_color, end_color, scale);
                self.line(x, rect_y, x, rect_y + rect_height as i32 - 1, color);
            }
        } else {
            // Non axis-aligned gradient
            // Gradient vector
            let grad_x = end_x as f64 - start_x as f64;
            let grad_y = end_y as f64 - start_y as f64;
            let grad_len = grad_x * grad_x + grad_y * grad_y;

            for y in rect_y..(rect_y + rect_height as i32) {
                for x in rect_x..(rect_x + rect_width as i32) {
                    // Pixel vector
                    let pix_x = x as f64 - start_x as f64;
                    let pix_y = y as f64 - start_y as f64;
                    // Scalar projection
                    let proj = (pix_x * grad_x + pix_y * grad_y) / grad_len;
                    // Saturation
                    let scale = if proj < 0.0 { 0.0 } else if proj > 1.0 { 1.0 } else { proj };
                    // Interpolation
                    let color = Color::interpolate(start_color, end_color, scale);
                    self.pixel(x, y, color);
                }
            }
        }
    }

    /// Draw a rect with rounded corners
    fn rounded_rect(&mut self, x: i32, y: i32, w: u32, h: u32, radius: u32, filled: bool, color: Color) {
        let w = w as i32;
        let h = h as i32;
        let r = radius as i32;


        if filled {
            //Draw inside corners
            self.arc(x + r, y + r, -r, 1 << 4 | 1 << 6, color);
            self.arc(x + w - 1 - r, y + r, -r, 1 << 5 | 1 << 7, color);
            self.arc(x + r, y + h - 1 - r,- r, 1 << 0 | 1 << 2, color);
            self.arc(x + w - 1 - r, y + h - 1 - r, -r, 1 << 1 | 1 << 3, color);

            // Draw inside rectangles
            self.rect(x + r, y, (w - 1 - r * 2) as u32, r as u32 + 1, color);
            self.rect(x + r, y + h - 1 - r, (w - 1 - r * 2) as u32, r as u32 + 1, color);
            self.rect(x, y + r + 1, w as u32, (h - 2 - r * 2) as u32, color);
        } else {
            //Draw outside corners
            self.arc(x + r, y + r, r, 1 << 4 | 1 << 6, color);
            self.arc(x + w - 1 - r, y + r, r, 1 << 5 | 1 << 7, color);
            self.arc(x + r, y + h - 1 - r, r, 1 << 0 | 1 << 2, color);
            self.arc(x + w - 1 - r, y + h - 1 - r, r, 1 << 1 | 1 << 3, color);

            // Draw outside rectangles
            self.rect(x + r + 1, y, (w - 2 - r * 2) as u32, 1, color);
            self.rect(x + r + 1, y + h - 1, (w - 2 - r * 2) as u32, 1, color);
            self.rect(x, y + r + 1, 1, (h - 2 - r * 2) as u32, color);
            self.rect(x + w - 1, y + r + 1, 1, (h - 2 - r * 2) as u32, color);
        }
    }

    /// Draws antialiased line
    fn wu_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: Color) {
        //adapted from https://rosettacode.org/wiki/Xiaolin_Wu's_line_algorithm#C.23
        let mut x0 = x0 as f64;
        let mut y0 = y0 as f64;
        let mut x1 = x1 as f64;
        let mut y1 = y1 as f64;
        let r = color.r();
        let g = color.g();
        let b = color.b();
        let a = color.a() as f64;
        
        fn ipart (x: f64) -> i32 {
            x.trunc() as i32
        }

        fn fpart (x: f64) -> f64 {
            if x <0.0 { return 1.0-(x-x.floor());}
            x-x.floor() 
        }
        
        fn rfpart(x: f64) -> f64 {
            1.0-fpart(x)
        }
        
        fn chkalpha (mut alpha :f64) -> u8 {
             if alpha > 255.0 { alpha = 255.0};
             if alpha < 0.0 {alpha = 0.0};
             alpha as u8
        }
        
        let steep :bool = (y1-y0).abs() > (x1-x0).abs();
        let mut temp;
        if steep {
            temp = x0; x0 = y0; y0 = temp;
            temp = x1; x1 = y1; y1 = temp;
        }
        if x0 > x1 {
            temp = x0; x0 = x1; x1 = temp;
            temp = y0; y0 = y1; y1 = temp;
        }
        let dx = x1 -x0;
        let dy = y1- y0;
        let gradient = dy/dx;
        
        let mut xend: f64 = (x0 as f64).round() ;
        let mut yend: f64 = y0 + gradient * (xend - x0);
        let mut xgap: f64 = rfpart(x0+0.5);
        let xpixel1 = xend as i32;
        let ypixel1 = (ipart (yend)) as i32;
        
        if steep {
            self.pixel(ypixel1, xpixel1, Color::rgba(r,g,b,chkalpha(rfpart(yend)*xgap*a)));
            self.pixel(ypixel1+1, xpixel1, Color::rgba(r,g,b,chkalpha(fpart(yend)*xgap*a)));
        }else{
            self.pixel(xpixel1, ypixel1, Color::rgba(r,g,b,chkalpha(rfpart(yend)*xgap*a)));
            self.pixel(xpixel1+1, ypixel1, Color::rgba(r,g,b,chkalpha(fpart(yend)*xgap*a)));
        }
        let mut intery :f64 = yend + gradient;
        xend = x1.round();
        yend = y1 + gradient * (xend-x1);
        xgap = fpart(x1 + 0.5);
        let xpixel2 = xend as i32;
        let ypixel2 = ipart(yend) as i32;
        if steep {
            self.pixel(ypixel2, xpixel2, Color::rgba(r,g,b,chkalpha(rfpart(yend)*xgap*a)));
            self.pixel(ypixel2+1, xpixel2, Color::rgba(r,g,b,chkalpha(fpart(yend)*xgap*a)));
        }else{
            self.pixel(xpixel2, ypixel2, Color::rgba(r,g,b,chkalpha(rfpart(yend)*xgap*a)));
            self.pixel(xpixel2+1, ypixel2, Color::rgba(r,g,b,chkalpha(fpart(yend)*xgap*a)));
        }
        if steep {
            for x in (xpixel1+1)..(xpixel2) {
                self.pixel(ipart(intery) as i32 , x, Color::rgba(r,g,b,chkalpha(a*rfpart(intery))));
                self.pixel(ipart(intery) as i32 + 1, x, Color::rgba(r,g,b,chkalpha(a*fpart(intery))));
                intery += gradient;
            }
        }else{
            for x in (xpixel1+1)..(xpixel2) {
                self.pixel(x, ipart(intery) as i32, Color::rgba(r,g,b,chkalpha(a*rfpart(intery))));
                self.pixel(x, ipart(intery) as i32 + 1, Color::rgba(r,g,b,chkalpha(a*fpart(intery))));
                intery += gradient;
            } 
        }           
    }

    ///Draws antialiased circle
    fn wu_circle(&mut self, x0: i32, y0: i32, radius: i32, color: Color) {
        let r = color.r();
        let g = color.g();
        let b = color.b();
        let a = color.a();
        let mut y = 0;
        let mut x = radius;
        let mut d = 0_f64;
        
        self.pixel (x0+x,y0+y,color);
        self.pixel (x0-x,y0-y,color);
        self.pixel (x0+y,y0-x,color);
        self.pixel (x0-y,y0+x,color);
        
        while x > y {
            let di = dist(radius,y);
            if di < d { x -= 1;}
            let col = Color::rgba(r,g,b,(a as f64*(1.0-di)) as u8);
            let col2 = Color::rgba(r,g,b,(a as f64*di) as u8);
            
            self.pixel(x0+x, y0+y, col);
            self.pixel(x0+x-1, y0+y, col2);//-
            self.pixel(x0-x, y0+y, col);
            self.pixel(x0-x+1, y0+y, col2);//+
            self.pixel(x0+x, y0-y, col);
            self.pixel(x0+x-1, y0-y, col2);//-
            self.pixel(x0-x, y0-y, col);
            self.pixel(x0-x+1, y0-y, col2);//+
            
            self.pixel(x0+y, y0+x, col);
            self.pixel(x0+y, y0+x-1, col2);
            self.pixel(x0-y, y0+x, col);
            self.pixel(x0-y, y0+x-1, col2);
            self.pixel(x0+y, y0-x, col);
            self.pixel(x0+y, y0-x+1, col2);
            self.pixel(x0-y, y0-x, col);
            self.pixel(x0-y, y0-x+1, col2);
            d = di;
            y += 1;
        }
        
        fn dist(r: i32, y: i32) -> f64 {
            let x :f64 = ((r*r-y*y)as f64).sqrt();
            x.ceil()-x
        }
    }

    ///Gets pixel color at x,y position
    fn getpixel(&self, x:i32, y:i32) -> Color {
        let p = (self.width()as i32 * y + x) as usize;
        if p > self.data().len() {
            return Color::rgba(0,0,0,0) 
        }
        self.data()[p]
    }
}
