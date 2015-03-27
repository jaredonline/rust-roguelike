extern crate tcod;

use self::tcod::{Console, BackgroundFlag, KeyState, Color, TextAlignment};
use self::tcod::TextAlignment::Left;

use std::rc::Rc;
use std::cell::RefCell;

use std::sync::mpsc::Receiver;

use util::{Point, Bound};
use input::{InputComponent, TcodInputComponent, KeyboardInput};

pub enum RenderAction {
    Print(String, Window),
    Flush(Window)
}

pub enum Window {
    Map,
    Stats,
    Input,
    Messages
}

pub fn print_action(string: &str, window: Window) -> RenderAction {
    RenderAction::Print(String::from_str(string), window)
}

pub trait RenderingComponent {
    fn before_render_new_frame(&mut self);
    fn render_object(&mut self, &Point, char);
    fn after_render_new_frame(&mut self);
    fn wait_for_keypress(&self) -> KeyboardInput;
    fn closed(&self) -> bool;
    fn attach_window(&mut self, SafeTcodWindow);
    fn get_windows(&self) -> Vec<SafeTcodWindow>;

    // buffer messages
    fn push_string(&mut self, &str, Window);
    fn push_message(&mut self, &str);
    fn push_input(&mut self, &str);

    // flush messages
    fn flush_window(&mut self, Window);
    fn flush_input(&mut self);

    // handle channel input
    fn handle_action(&mut self, RenderAction);
}

pub struct TcodRenderingComponent {
    input:      Box<InputComponent<self::tcod::KeyState> + 'static>,
    map:        SafeTcodWindow,
    stats:      SafeTcodWindow,
    messages:   SafeTcodWindow,
    user_input: SafeTcodWindow,
    console:    Console,
    receiver:   Receiver<RenderAction>,
}

type Messages = Rc<RefCell<Vec<Box<String>>>>;

pub struct TcodWindow {
    console:      Console,
    bg_color:     Color,
    bounds:       Bound,
    messages:     Messages,
    max_messages: usize
}

impl TcodWindow {
    fn new(bound: &Bound, color: Color, max_messages: usize) -> TcodWindow {
        let height  = bound.max.y - bound.min.y;
        let width   = bound.max.x - bound.min.x;
        let console = Console::new(width, height);

        TcodWindow {
            bounds:   bound.clone(),
            bg_color: color,
            console:  console,
            messages: Rc::new(RefCell::new(vec![])),
            max_messages: max_messages
        }
    }

    fn buffer_message(&mut self, text: &str) {
        let message = Box::new(String::from_str(text));
        let mut messages = self.messages.borrow_mut();

        messages.insert(0, message);
        messages.truncate(self.max_messages);
    }

    fn clear(&mut self) {
        self.console.set_default_background(self.bg_color);
        self.console.clear();
    }

    fn print_message(&mut self, x: i32, y: i32, align: TextAlignment, text: &str) {
        self.console.print_ex(x, y, BackgroundFlag::Set, align, text);
    }

    fn flush_buffer(&mut self) {
        self.messages = Rc::new(RefCell::new(vec![]));
    }
}

pub type SafeTcodWindow = Rc<RefCell<TcodWindow>>;

impl TcodRenderingComponent {
    pub fn new(bound: &Bound, map_bound: &Bound, stats_bound: &Bound, messages_bound: &Bound, user_input_bound: &Bound, r: Receiver<RenderAction>) -> TcodRenderingComponent {
        let input   = Box::new(TcodInputComponent::new());
        let con     = Console::init_root(bound.max.x, bound.max.y, "libtcod Rust tutorial", false);

        let map_color        = Color::new(0, 0, 0);
        let stats_color      = Color::new(0, 0, 0);
        let messages_color   = Color::new(0, 0, 0);
        let user_input_color = Color::new(0, 0, 0);

        let map        = Rc::new(RefCell::new(TcodWindow::new(map_bound, map_color, 10)));
        let stats      = Rc::new(RefCell::new(TcodWindow::new(stats_bound, stats_color, 10)));
        let messages   = Rc::new(RefCell::new(TcodWindow::new(messages_bound, messages_color, 10)));
        let user_input = Rc::new(RefCell::new(TcodWindow::new(user_input_bound, user_input_color, 2)));

        TcodRenderingComponent {
            input:      input,
            console:    con,
            map:        map,
            stats:      stats,
            messages:   messages,
            user_input: user_input,
            receiver:   r
        }
    }
}

impl RenderingComponent for TcodRenderingComponent {
fn before_render_new_frame(&mut self) {
    self.console.clear();

    let mut exit = false;
    while !exit {
        match self.receiver.try_recv() {
            Ok(action) => self.handle_action(action),
            Err(_)     => exit = true
        }
    }

    for window in self.get_windows() {
        self.attach_window(window.clone());
    }
}

    fn handle_action(&mut self, action: RenderAction) {
        match action {
            RenderAction::Print(string, window) => self.push_string(string.as_ref(), window),
            RenderAction::Flush(window)         => self.flush_window(window)
        }
    }

    fn attach_window(&mut self, window: SafeTcodWindow) {
        let mut window = window.borrow_mut();
        window.clear();

        let w = window.messages.clone();
        let messages = w.borrow();

        let mut line = 0;
        for message in messages.iter() {
            window.print_message(0, line, Left, message);
            line += 1;
        }

        let bounds  = &window.bounds;
        let console = &window.console;

        Console::blit(console, 0, 0, (bounds.max.x) + 1, (bounds.max.y) + 1, &mut self.console, bounds.min.x, bounds.min.y, 1f32, 1f32);
    }

    fn render_object(&mut self, position: &Point, symbol: char) {
        self.console.put_char(position.x, position.y, symbol, BackgroundFlag::Set);
    }

    fn after_render_new_frame(&mut self) {
        Console::flush();
    }

    fn wait_for_keypress(&self) -> KeyboardInput {
        self.input.translate_key(Console::wait_for_keypress(true))
    }

    fn closed(&self) -> bool {
        Console::window_closed()
    }

    fn get_windows(&self) -> Vec<SafeTcodWindow> {
        vec![
            self.map.clone(),
            self.stats.clone(),
            self.messages.clone(),
            self.user_input.clone()
        ]
    }

    fn push_string(&mut self, message: &str, w_type: Window) {
        match w_type {
            Window::Map => {},
            Window::Input => self.push_input(message),
            Window::Messages => self.push_message(message),
            Window::Stats => {}
        };
    }

    fn flush_window(&mut self, w_type: Window) {
        match w_type {
            Window::Map => {},
            Window::Input => self.flush_input(),
            Window::Messages => {},
            Window::Stats => {}
        };
    }

    fn push_message(&mut self, message: &str) {
        let mut messages = self.messages.borrow_mut();
        messages.buffer_message(message);
    }

    fn push_input(&mut self, message: &str) {
        let mut input = self.user_input.borrow_mut();
        input.buffer_message(message);
    }

    fn flush_input(&mut self) {
        let mut input = self.user_input.borrow_mut();
        input.flush_buffer();
    }
}
