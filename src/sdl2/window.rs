extern crate num;
extern crate sdl2;

use self::num::traits::ToPrimitive;

use super::{FONT, Color, Event, KeyEvent, MouseEvent, QuitEvent};

/// A window
#[allow(dead_code)]
pub struct Window {
    /// The x coordinate of the window
    x: i32,
    /// The y coordinate of the window
    y: i32,
    /// The width of the window
    w: u32,
    /// The height of the window
    h: u32,
    /// The title of the window
    t: String,
    /// SDL2 Context
    ctx: sdl2::Sdl,
    /// Video Context
    video_ctx: sdl2::VideoSubsystem,
    /// Event Pump
    event_pump: sdl2::EventPump,
    /// The inner renderer
    inner: sdl2::render::Renderer<'static>,
}

impl Window {
    /// Create a new window
    pub fn new(x: i32, y: i32, w: u32, h: u32, title: &str) -> Option<Box<Self>> {
        let ctx = sdl2::init().unwrap();
        let video_ctx = ctx.video().unwrap();
        let event_pump = ctx.event_pump().unwrap();

        let mut builder = video_ctx.window(title, w, h);

        if x >= 0 || y >= 0 {
            builder.position(x, y);
        }

        match builder.build() {
            Ok(window) => Some(Box::new(Window {
                x: x,
                y: y,
                w: w,
                h: h,
                t: title.to_string(),
                ctx: ctx,
                video_ctx: video_ctx,
                event_pump: event_pump,
                inner: window.renderer().build().unwrap(),
            })),
            Err(_) => None
        }
    }

    pub fn sync_path(&mut self) {
        if let Some(window) = self.inner.window() {
            self.x = window.position().0;
            self.x = window.position().1;
            self.w = window.size().0;
            self.h = window.size().1;
            self.t = window.title().to_string();
        }
    }

    /// Get x
    // TODO: Sync with window movements
    pub fn x(&self) -> i32 {
        self.x
    }

    /// Get y
    // TODO: Sync with window movements
    pub fn y(&self) -> i32 {
        self.y
    }

    /// Get width
    pub fn width(&self) -> u32 {
        self.w
    }

    /// Get height
    pub fn height(&self) -> u32 {
        self.h
    }

    /// Get title
    pub fn title(&self) -> String {
        self.t.clone()
    }

    /// Set title
    pub fn set_title(&mut self, _: &str) {
        // TODO
    }

    /// Draw a pixel
    pub fn pixel(&mut self, x: i32, y: i32, color: Color) {
        self.inner.set_draw_color(sdl2::pixels::Color::RGBA(color.data as u8, (color.data >> 8) as u8, (color.data >> 16) as u8, (color.data >> 24) as u8));
        self.inner.draw_point(sdl2::rect::Point::new(x, y));
    }

    /// Draw a character, using the loaded font
    pub fn char(&mut self, x: i32, y: i32, c: char, color: Color) {
        self.inner.set_draw_color(sdl2::pixels::Color::RGBA(color.data as u8, (color.data >> 8) as u8, (color.data >> 16) as u8, (color.data >> 24) as u8));

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
                    self.inner.draw_point(sdl2::rect::Point::new(x + col as i32, y + row as i32));
                }
            }
            offset += 1;
        }
    }

    // TODO move, resize, set_title

    /// Set entire window to a color
    // TODO: Improve speed
    #[allow(unused_variables)]
    pub fn set(&mut self, color: Color) {
        let w = self.w;
        let h = self.h;
        self.rect(0, 0, w, h, color);
    }

    /// Draw rectangle
    // TODO: Improve speed
    #[allow(unused_variables)]
    pub fn rect(&mut self, start_x: i32, start_y: i32, w: u32, h: u32, color: Color) {
        if let Some(rect) = sdl2::rect::Rect::new(start_x, start_y, w, h).unwrap_or(None) {
            self.inner.set_draw_color(sdl2::pixels::Color::RGBA(color.data as u8, (color.data >> 8) as u8, (color.data >> 16) as u8, (color.data >> 24) as u8));
            self.inner.fill_rect(rect);
        }
    }

    /// Display an image
    // TODO: Improve speed
    pub fn image(&mut self, start_x: i32, start_y: i32, w: u32, h: u32, data: &[Color]) {
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

    fn convert_event(&self, event: sdl2::event::Event) -> Event {
        match event {
            sdl2::event::Event::MouseMotion { .. } => {
               let mouse = self.ctx.mouse().mouse_state();
               MouseEvent {
                x: mouse.1,
                y: mouse.2,
                left_button: mouse.0.left(),
                middle_button: mouse.0.middle(),
                right_button: mouse.0.right()
                }.to_event()
            },
            sdl2::event::Event::MouseButtonDown { .. } => {
               let mouse = self.ctx.mouse().mouse_state();
               MouseEvent {
                x: mouse.1,
                y: mouse.2,
                left_button: mouse.0.left(),
                middle_button: mouse.0.middle(),
                right_button: mouse.0.right()
                }.to_event()
            },
            sdl2::event::Event::MouseButtonUp { .. } => {
               let mouse = self.ctx.mouse().mouse_state();
               MouseEvent {
                x: mouse.1,
                y: mouse.2,
                left_button: mouse.0.left(),
                middle_button: mouse.0.middle(),
                right_button: mouse.0.right()
                }.to_event()
            },
            sdl2::event::Event::KeyDown { scancode, .. } => KeyEvent {
                character: if let Some(sc) = scancode {
                    if let Some(c) = sc.name().chars().next() {
                        c
                    } else {
                        '\0'
                    }
                } else {
                    '\0'
                },
                scancode: if let Some(sc) = scancode {
                    sc.to_u8().unwrap_or(0)
                } else {
                    0
                },
                pressed: true,
            }.to_event(),
            sdl2::event::Event::KeyUp { scancode, .. } => KeyEvent {
                character: if let Some(sc) = scancode {
                    if let Some(c) = sc.name().chars().next() {
                        c
                    } else {
                        '\0'
                    }
                } else {
                    '\0'
                },
                scancode: if let Some(sc) = scancode {
                    sc.to_u8().unwrap_or(0)
                } else {
                    0
                },
                pressed: false,
            }.to_event(),
            sdl2::event::Event::Quit { .. } => QuitEvent.to_event(),
            _ => Event::new(),
        }
    }

    /// Return a iterator over events
    pub fn events(&mut self) -> EventIter {
        let mut iter = EventIter {
            events: [Event::new(); 128],
            i: 0,
            count: 0,
        };

        while let Some(event) = self.event_pump.poll_event() {
            if iter.count < iter.events.len() {
                iter.events[iter.count] = self.convert_event(event);
                iter.count += 1;
            } else {
                break;
            }
        }

        if iter.count == 0 {
            let event = self.event_pump.wait_event();
            iter.events[iter.count] = self.convert_event(event);
            iter.count += 1;
        }

        iter
    }

    /// Poll for an event
    // TODO: Replace with events()
    #[deprecated]
    pub fn poll(&mut self) -> Option<Event> {
        let event = self.event_pump.wait_event();
        Some(self.convert_event(event))
    }

    /// Flip the window buffer
    pub fn sync(&mut self) -> bool {
        self.inner.present();
        true
    }
}

/// Event iterator
pub struct EventIter {
    events: [Event; 128],
    i: usize,
    count: usize,
}

impl Iterator for EventIter {
    type Item = Event;
    fn next(&mut self) -> Option<Event> {
        if self.i < self.count {
            if let Some(event) = self.events.get(self.i) {
                self.i += 1;
                Some(*event)
            } else {
                None
            }
        } else {
            None
        }
    }
}
