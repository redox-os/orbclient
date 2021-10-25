// SPDX-License-Identifier: MIT

use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

use wasm_bindgen::{prelude::*, Clamped, JsCast};
use web_sys::{Document, HtmlCanvasElement, HtmlElement, Window as WebWindow};

use crate::color::Color;
use crate::event::*;
use crate::renderer::Renderer;
use crate::Mode;
use crate::WindowFlag;

pub fn get_display_size() -> Result<(u32, u32), String> {
    let width = window()?
        .inner_width()
        .map_err(|_| "Cannot read display width.".to_string())?
        .as_f64()
        .unwrap_or(0.0) as u32;
    let height = window()?
        .inner_height()
        .map_err(|_| "Cannot read display height.".to_string())?
        .as_f64()
        .unwrap_or(0.0) as u32;
    Ok((width, height))
}

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
    /// True if the window should not wait for events
    window_async: bool,
    /// Drawing mode
    mode: Cell<Mode>,
    /// Mouse in relative mode
    mouse_relative: Rc<RefCell<bool>>,
    /// Content of the last drop (file | text) operation
    drop_content: Rc<RefCell<Option<String>>>,
    /// inner frame buffer
    data: Vec<Color>,
    /// html canvas that represents the window
    canvas: web_sys::HtmlCanvasElement,
    /// the 2d render context of the html canvas
    context: web_sys::CanvasRenderingContext2d,
    /// list of current events.
    events: Rc<RefCell<Vec<Event>>>,
    /// Current mouse button states (pressed | released) (0 => left, 1 => middle, 2 => right)
    button_state: Rc<RefCell<(bool, bool, bool)>>,
}

impl Renderer for Window {
    /// Get width
    fn width(&self) -> u32 {
        self.w
    }

    /// Get height
    fn height(&self) -> u32 {
        self.h
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
        let bytes = self.data_mut();
        let len = bytes.len() * std::mem::size_of::<Color>();

        // converts the `Color` data to u8
        let color_data =
            unsafe { std::slice::from_raw_parts_mut(bytes.as_mut_ptr() as *mut u8, len) };

        // crates a `ImageData` object from the window buffer and put it to the canvas.
        if let Ok(image_data) =
            web_sys::ImageData::new_with_u8_clamped_array(Clamped(color_data), self.canvas.width())
        {
            self.context.put_image_data(&image_data, 0.0, 0.0).unwrap();
        }

        true
    }

    /// Set/get mode
    fn mode(&self) -> &Cell<Mode> {
        &self.mode
    }
}

impl Window {
    /// Create a new window
    pub fn new(x: i32, y: i32, w: u32, h: u32, title: &str) -> Option<Self> {
        Window::new_flags(x, y, w, h, title, &vec![])
    }

    /// Create a new window with flags
    pub fn new_flags(
        x: i32,
        y: i32,
        w: u32,
        h: u32,
        title: &str,
        _flags: &[WindowFlag],
    ) -> Option<Self> {
        // todo resizable.

        // set the window title to document title (browser tab)
        if let Ok(document) = document() {
            document.set_title(title);
        }

        // used to fix initial margin around the canvas
        if let Ok(body) = body() {
            body.style().set_property("padding", "0").unwrap();
            body.style().set_property("margin", "0").unwrap();
        }

        if let Ok(canvas) = canvas("canvas") {
            // used to fix initial margin around the canvas
            canvas.style().set_property("display", "block").unwrap();
            canvas.style().set_property("margin", "0").unwrap();

            // set x and y of canvas inside of body
            canvas.style().set_property("position", "absolute").unwrap();
            canvas
                .style()
                .set_property("left", x.to_string().as_str())
                .unwrap();
            canvas
                .style()
                .set_property("top", y.to_string().as_str())
                .unwrap();

            canvas.set_width(w);
            canvas.set_height(h);

            let events = Rc::new(RefCell::new(vec![]));
            let button_state = Rc::new(RefCell::new((false, false, false)));
            let mouse_relative = Rc::new(RefCell::new(false));
            let drop_content = Rc::new(RefCell::new(None));

            connect_event_handlers(
                mouse_relative.clone(),
                &canvas,
                &events,
                &button_state,
                &drop_content,
            );

            if let Ok(context) = context_2d(&canvas) {
                return Some(Window {
                    x,
                    y,
                    w,
                    h,
                    t: title.to_string(),
                    window_async: false,
                    mode: Cell::new(Mode::Blend),
                    mouse_relative,
                    drop_content,
                    data: vec![Color::rgb(255, 255, 255); (w * h) as usize],
                    canvas,
                    context,
                    events,
                    button_state,
                });
            }
        }

        None
    }

    pub fn clipboard(&self) -> String {
        log("clipboard not yet implemented for web");
        String::default()
    }

    pub fn set_clipboard(&mut self, _text: &str) {
        log("set_clipboard not yet implemented for web");
    }

    /// Pops the content of the last drop event from the window.
    pub fn pop_drop_content(&self) -> Option<String> {
        let result = self.drop_content.borrow().clone();
        *self.drop_content.borrow_mut() = None;

        result
    }

    pub fn sync_path(&mut self) {
        self.x = self.canvas.offset_left();
        self.y = self.canvas.offset_top();
        self.w = self.canvas.width();
        self.h = self.canvas.height();
        if let Ok(document) = document() {
            self.t = document.title();
        }
    }

    /// Get x
    pub fn x(&self) -> i32 {
        self.x
    }

    /// Get y
    pub fn y(&self) -> i32 {
        self.y
    }

    /// Get title
    pub fn title(&self) -> String {
        self.t.clone()
    }

    /// Set cursor visibility
    pub fn set_mouse_cursor(&mut self, visible: bool) {
        if let Ok(body) = body() {
            if visible {
                body.style().set_property("cursor", "auto").unwrap();
            } else {
                body.style().set_property("cursor", "none").unwrap();
            }
        }
    }

    /// Set mouse grabbing
    pub fn set_mouse_grab(&mut self, grab: bool) {
        if grab {
            self.canvas.set_capture();
        } else {
            self.canvas.release_capture();
        }
    }

    /// Set mouse relative mode
    pub fn set_mouse_relative(&mut self, relative: bool) {
        *self.mouse_relative.borrow_mut() = relative;
    }

    /// Set position
    pub fn set_pos(&mut self, x: i32, y: i32) {
        self.canvas
            .style()
            .set_property("left", x.to_string().as_str())
            .unwrap();
        self.canvas
            .style()
            .set_property("top", y.to_string().as_str())
            .unwrap();

        self.sync_path();
    }

    /// Set size
    pub fn set_size(&mut self, width: u32, height: u32) {
        self.canvas.set_width(width);
        self.canvas.set_height(height);

        self.sync_path();
    }

    /// Set title
    pub fn set_title(&mut self, title: &str) {
        if let Ok(document) = document() {
            document.set_title(title);
        }
        self.sync_path();
    }

    /// Blocking iterator over events
    pub fn events(&mut self) -> EventIter {
        let mut iter = EventIter {
            events: [Event::new(); 16],
            i: 0,
            count: 0,
        };

        let mut sync_path = false;
        for event in self.events.borrow_mut().pop() {
            match event.code {
                EVENT_RESIZE | EVENT_MOVE => sync_path = true,
                _ => {}
            }
            if iter.count < iter.events.len() {
                iter.events[iter.count] = event;
                iter.count += 1;
            } else {
                break;
            }
        }

        if sync_path {
            self.sync_path();
        }

        iter
    }

    // Returns the id
    // pub fn id(&self) -> u32 {
    //     self.inner.window().id()
    // }
}

// unsafe impl raw_window_handle::HasRawWindowHandle for Window {
//     fn raw_window_handle(&self) -> raw_window_handle::RawWindowHandle {
//         self.inner.window().raw_window_handle()
//     }
// }

/// Event iterator
pub struct EventIter {
    events: [Event; 16],
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

// Web Helpers

fn convert_code(code: String, shift: bool) -> Option<(char, u8)> {
    match code.as_str() {
        "KeyA" => Some((if shift { 'A' } else { 'a' }, K_A)),
        "KeyB" => Some((if shift { 'B' } else { 'b' }, K_B)),
        "KeyC" => Some((if shift { 'C' } else { 'c' }, K_C)),
        "KeyD" => Some((if shift { 'D' } else { 'd' }, K_D)),
        "KeyE" => Some((if shift { 'E' } else { 'e' }, K_E)),
        "KeyF" => Some((if shift { 'F' } else { 'f' }, K_F)),
        "KeyG" => Some((if shift { 'G' } else { 'g' }, K_G)),
        "KeyH" => Some((if shift { 'H' } else { 'h' }, K_H)),
        "KeyI" => Some((if shift { 'I' } else { 'i' }, K_I)),
        "KeyJ" => Some((if shift { 'J' } else { 'j' }, K_J)),
        "KeyK" => Some((if shift { 'K' } else { 'k' }, K_K)),
        "KeyL" => Some((if shift { 'L' } else { 'l' }, K_L)),
        "KeyM" => Some((if shift { 'M' } else { 'm' }, K_M)),
        "KeyN" => Some((if shift { 'N' } else { 'n' }, K_N)),
        "KeyO" => Some((if shift { 'O' } else { 'o' }, K_O)),
        "KeyP" => Some((if shift { 'P' } else { 'p' }, K_P)),
        "KeyQ" => Some((if shift { 'Q' } else { 'q' }, K_Q)),
        "KeyR" => Some((if shift { 'R' } else { 'r' }, K_R)),
        "KeyS" => Some((if shift { 'S' } else { 's' }, K_S)),
        "KeyT" => Some((if shift { 'T' } else { 't' }, K_T)),
        "KeyU" => Some((if shift { 'U' } else { 'u' }, K_U)),
        "KeyV" => Some((if shift { 'V' } else { 'v' }, K_V)),
        "KeyW" => Some((if shift { 'W' } else { 'w' }, K_W)),
        "KeyX" => Some((if shift { 'X' } else { 'x' }, K_X)),
        "KeyY" => Some((if shift { 'Y' } else { 'y' }, K_Y)),
        "KeyZ" => Some((if shift { 'Z' } else { 'z' }, K_Z)),
        "Digit0" => Some((if shift { ')' } else { '0' }, K_0)),
        "Digit1" => Some((if shift { '!' } else { '1' }, K_1)),
        "Digit2" => Some((if shift { '@' } else { '2' }, K_2)),
        "Digit3" => Some((if shift { '#' } else { '3' }, K_3)),
        "Digit4" => Some((if shift { '$' } else { '4' }, K_4)),
        "Digit5" => Some((if shift { '%' } else { '5' }, K_5)),
        "Digit6" => Some((if shift { '^' } else { '6' }, K_6)),
        "Digit7" => Some((if shift { '&' } else { '7' }, K_7)),
        "Digit8" => Some((if shift { '*' } else { '8' }, K_8)),
        "Digit9" => Some((if shift { '(' } else { '9' }, K_9)),
        "Grave" => Some((if shift { '~' } else { '`' }, K_TICK)),
        "Subtract" => Some((if shift { '_' } else { '-' }, K_MINUS)),
        "Equals" => Some((if shift { '+' } else { '=' }, K_EQUALS)),
        "BracketLeft" => Some((if shift { '{' } else { '[' }, K_BRACE_OPEN)),
        "BracketRight" => Some((if shift { '}' } else { ']' }, K_BRACE_CLOSE)),
        "Backslash" => Some((if shift { '|' } else { '\\' }, K_BACKSLASH)),
        "Semicolon" => Some((if shift { ':' } else { ';' }, K_SEMICOLON)),
        "Apostrophe" => Some((if shift { '"' } else { '\'' }, K_QUOTE)),
        "Comma" => Some((if shift { '<' } else { ',' }, K_COMMA)),
        "Period" => Some((if shift { '>' } else { '.' }, K_PERIOD)),
        "Slash" => Some((if shift { '?' } else { '/' }, K_SLASH)),
        "Space" => Some((' ', K_SPACE)),
        "Backspace" => Some(('\0', K_BKSP)),
        "Tab" => Some(('\t', K_TAB)),
        "ControlLeft" => Some(('\0', K_CTRL)),
        "ControlRight" => Some(('\0', K_CTRL)),
        "AltLeft" => Some(('\0', K_ALT)),
        "AltRight" => Some(('\0', K_ALT)),
        "Enter" => Some(('\n', K_ENTER)),
        "Escape" => Some(('\x1B', K_ESC)),
        "F1" => Some(('\0', K_F1)),
        "F2" => Some(('\0', K_F2)),
        "F3" => Some(('\0', K_F3)),
        "F4" => Some(('\0', K_F4)),
        "F5" => Some(('\0', K_F5)),
        "F6" => Some(('\0', K_F6)),
        "F7" => Some(('\0', K_F7)),
        "F8" => Some(('\0', K_F8)),
        "F9" => Some(('\0', K_F9)),
        "F10" => Some(('\0', K_F10)),
        "OSLeft" => Some(('\0', K_HOME)),
        "OSRight" => Some(('\0', K_HOME)),
        "ArrowUp" => Some(('\0', K_UP)),
        "PageUp" => Some(('\0', K_PGUP)),
        "ArrowLeft" => Some(('\0', K_LEFT)),
        "ArrowRight" => Some(('\0', K_RIGHT)),
        "End" => Some(('\0', K_END)),
        "ArrowDown" => Some(('\0', K_DOWN)),
        "PageDown" => Some(('\0', K_PGDN)),
        "Delete" => Some(('\0', K_DEL)),
        "F11" => Some(('\0', K_F11)),
        "F12" => Some(('\0', K_F12)),
        "ShiftLeft" => Some(('\0', K_LEFT_SHIFT)),
        "ShiftRight" => Some(('\0', K_RIGHT_SHIFT)),
        _ => None,
    }
}

// Helpers

/// Gets the browser window.
pub fn window() -> Result<WebWindow, String> {
    web_sys::window().ok_or("body: no global `windows` exists.".to_string())
}

/// Gets the document of the browser window.
pub fn document() -> Result<Document, String> {
    window()?
        .document()
        .ok_or("body: should have a document on window.".to_string())
}

/// Gets the body of the browser document.
pub fn body() -> Result<HtmlElement, String> {
    document()?
        .body()
        .ok_or("body: document should have a body.".to_string())
}

/// Gets a canvas by the given id or create it if it does not exists.
pub fn canvas(id: &str) -> Result<HtmlCanvasElement, String> {
    let canvas = {
        if let Some(canvas) = document()?.get_element_by_id(id) {
            Some(canvas)
        } else {
            if let Ok(canvas) = document()?.create_element("canvas") {
                canvas.set_id(id);
                if let Err(_) = body()?.append_child(&canvas) {
                    return Err("canvas: Could not add canvas to body.".to_string());
                }
                Some(canvas)
            } else {
                None
            }
        }
    };
    canvas.map_or(Err("canvas: Could not create canvas.".to_string()), |c| {
        c.dyn_into::<HtmlCanvasElement>()
            .map_err(|_| "canvas: Could not convert canvas.".to_string())
    })
}

/// Gets the 2d context of the given canvas.
pub fn context_2d(canvas: &HtmlCanvasElement) -> Result<web_sys::CanvasRenderingContext2d, String> {
    canvas
        .get_context("2d")
        .map_err(|_| "context_2d: Could not get 2d context of canvas".to_string())?
        .ok_or("context_2d: Could not get 2d context of canvas".to_string())?
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .map_err(|_| "CanvasDisplay::new: Could not convert canvas context_2.".to_string())
}

/// Outputs a message to the web console.
pub fn log(msg: impl Into<JsValue>) {
    web_sys::console::log_1(&msg.into());
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    if let Ok(window) = window() {
        window
            .request_animation_frame(f.as_ref().unchecked_ref())
            .expect("should register `requestAnimationFrame` OK");
    }
}

/// Wraps request animation frame on web to run a loop.
pub fn animation_loop<R: FnMut() -> bool + 'static>(mut run: R) {
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        if !run() {
            return;
        }
        // Schedule ourself for another requestAnimationFrame callback.
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));
    request_animation_frame(g.borrow().as_ref().unwrap());
}

// connect web event handlers and pushes OrbClient events
fn connect_event_handlers(
    mouse_relative: Rc<RefCell<bool>>,
    canvas: &HtmlCanvasElement,
    events: &Rc<RefCell<Vec<Event>>>,
    button_state: &Rc<RefCell<(bool, bool, bool)>>,
    drop_content: &Rc<RefCell<Option<String>>>,
) {
    // mouse event
    {
        let events = events.clone();
        let closure = Closure::wrap(Box::new(move |e: web_sys::MouseEvent| {
            if *mouse_relative.borrow() {
                events.borrow_mut().push(
                    MouseEvent {
                        x: e.movement_x(),
                        y: e.movement_y(),
                    }
                    .to_event(),
                );
            } else {
                events.borrow_mut().push(
                    MouseEvent {
                        x: e.offset_x(),
                        y: e.offset_y(),
                    }
                    .to_event(),
                );
            }
        }) as Box<dyn FnMut(_)>);
        canvas
            .add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }

    // mouse down
    {
        let events = events.clone();
        let button_state = button_state.clone();
        let closure = Closure::wrap(Box::new(move |e: web_sys::MouseEvent| {
            *button_state.borrow_mut() = (e.button() == 0, e.button() == 1, e.button() == 2);

            events.borrow_mut().push(
                ButtonEvent {
                    left: e.button() == 0,
                    middle: e.button() == 1,
                    right: e.button() == 2,
                }
                .to_event(),
            );
        }) as Box<dyn FnMut(_)>);
        canvas
            .add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }

    // mouse up
    {
        let events = events.clone();
        let button_state = button_state.clone();
        let closure = Closure::wrap(Box::new(move |e: web_sys::MouseEvent| {
            *button_state.borrow_mut() = (
                !e.button() == 0 && button_state.borrow().0,
                !e.button() == 1 && button_state.borrow().1,
                !e.button() == 2 && button_state.borrow().2,
            );

            events.borrow_mut().push(
                ButtonEvent {
                    left: button_state.borrow().0,
                    middle: button_state.borrow().1,
                    right: button_state.borrow().2,
                }
                .to_event(),
            );
        }) as Box<dyn FnMut(_)>);
        canvas
            .add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }

    // key down and text input
    {
        let events = events.clone();

        let closure = Closure::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
            // need to check len of key to prevent keys like `Shift` or `Backspace` are handled as text input
            if !e.repeat() && e.key().len() <= 2 {
                events.borrow_mut().push(
                    TextInputEvent {
                        character: e.key().chars().nth(0).unwrap(),
                    }
                    .to_event(),
                );
            }
            if let Some(code) = convert_code(e.code(), e.shift_key()) {
                events.borrow_mut().push(
                    KeyEvent {
                        character: code.0,
                        scancode: code.1,
                        pressed: true,
                    }
                    .to_event(),
                );
            }
        }) as Box<dyn FnMut(_)>);
        document()
            .unwrap()
            .add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }

    // key up
    {
        let events = events.clone();

        let closure = Closure::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
            if let Some(code) = convert_code(e.code(), e.shift_key()) {
                events.borrow_mut().push(
                    KeyEvent {
                        character: code.0,
                        scancode: code.1,
                        pressed: false,
                    }
                    .to_event(),
                );
            }
        }) as Box<dyn FnMut(_)>);
        document()
            .unwrap()
            .add_event_listener_with_callback("keyup", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }

    // wheel (scroll)
    {
        let events = events.clone();

        let closure = Closure::wrap(Box::new(move |e: web_sys::WheelEvent| {
            events.borrow_mut().push(
                ScrollEvent {
                    x: e.delta_x() as i32,
                    y: e.delta_y() as i32,
                }
                .to_event(),
            )
        }) as Box<dyn FnMut(_)>);
        document()
            .unwrap()
            .add_event_listener_with_callback("wheel", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }

    // focus
    {
        let events = events.clone();

        let closure = Closure::wrap(Box::new(move |_: web_sys::FocusEvent| {
            events
                .borrow_mut()
                .push(FocusEvent { focused: true }.to_event())
        }) as Box<dyn FnMut(_)>);
        document()
            .unwrap()
            .add_event_listener_with_callback("focus", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }

    // blur
    {
        let events = events.clone();

        let closure = Closure::wrap(Box::new(move |_: web_sys::FocusEvent| {
            events
                .borrow_mut()
                .push(FocusEvent { focused: false }.to_event())
        }) as Box<dyn FnMut(_)>);
        document()
            .unwrap()
            .add_event_listener_with_callback("blur", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }

    // hover (enter)
    {
        let events = events.clone();
        let closure = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
            events
                .borrow_mut()
                .push(HoverEvent { entered: true }.to_event());
        }) as Box<dyn FnMut(_)>);
        canvas
            .add_event_listener_with_callback("mouseenter", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }

    // hover (leave)
    {
        let events = events.clone();
        let closure = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
            events
                .borrow_mut()
                .push(HoverEvent { entered: false }.to_event());
        }) as Box<dyn FnMut(_)>);
        canvas
            .add_event_listener_with_callback("mouseleave", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }

    // drop
    {
        let events = events.clone();
        let drop_content = drop_content.clone();
        let closure = Closure::wrap(Box::new(move |e: web_sys::DragEvent| {
            if let Some(data_transfer) = e.data_transfer() {
                if let Ok(text) = data_transfer.get_data("text") {
                    *drop_content.borrow_mut() = Some(text);
                    events
                        .borrow_mut()
                        .push(DropEvent { kind: DROP_TEXT }.to_event());
                }
            }
        }) as Box<dyn FnMut(_)>);
        canvas
            .add_event_listener_with_callback("drop", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }

    // resize
    {
        let events = events.clone();
        let canvas_clone = canvas.clone();
        let closure = Closure::wrap(Box::new(move |_: web_sys::UiEvent| {
            events.borrow_mut().push(
                ResizeEvent {
                    width: canvas_clone.width(),
                    height: canvas_clone.height(),
                }
                .to_event(),
            );
        }) as Box<dyn FnMut(_)>);
        canvas
            .add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }
}
