extern crate tcod;
use self::tcod::{Console, background_flag, KeyState, Color};

use util::{Point, Bound};

pub trait RenderingComponent {
    fn new(Bound) -> Self;
    fn before_render_new_frame(&mut self);
    fn render_object(&mut self, Point, char);
    fn after_render_new_frame(&mut self);
    fn wait_for_keypress(&self) -> KeyState;
    fn attach_window(&mut self, &mut Box<WindowComponent>);
}

pub trait WindowComponent {
    fn new(Bound) -> Self;

    fn get_bounds(&self)      -> Bound;
    fn get_bg_color(&self)    -> Color;
    fn get_console(&mut self) -> &mut Console;

    fn clear(&mut self) {
        let color       = self.get_bg_color();
        let mut console = self.get_console();
        console.set_default_background(color);
        console.clear();
    }

    fn print_message(&mut self, x: int, y: int, alignment: tcod::TextAlignment, text: &str) {
        let mut console = self.get_console();
        console.print_ex(x, y, background_flag::Set, alignment, text);
    }
}

pub struct TcodStatsWindowComponent {
    pub console:          Console,
    pub background_color: Color,
    bounds:               Bound
}

impl WindowComponent for TcodStatsWindowComponent {
    fn new(bounds: Bound) -> TcodStatsWindowComponent {
        let height = bounds.max.y - bounds.min.y + 1;
        let width  = bounds.max.x - bounds.min.x + 1;
        let mut console = Console::new(
            width  as int,
            height as int,
        );

        let red = Color::new(255u8, 0u8, 0u8);
        TcodStatsWindowComponent {
            console:          console,
            background_color: red,
            bounds:           bounds
        }
    }

    fn get_console(&mut self) -> &mut Console { &mut self.console     }
    fn get_bounds(&self)      ->      Bound   { self.bounds           }
    fn get_bg_color(&self)    ->      Color   { self.background_color }
}

pub struct TcodInputWindowComponent {
    pub console:          Console,
    pub background_color: Color,
    bounds:               Bound
}

impl WindowComponent for TcodInputWindowComponent {
    fn new(bounds: Bound) -> TcodInputWindowComponent {
        let height = bounds.max.y - bounds.min.y + 1;
        let width  = bounds.max.x - bounds.min.x + 1;
        let console = Console::new(
            width  as int,
            height as int,
        );

        let green = Color::new(0u8, 255u8, 0u8);
        TcodInputWindowComponent {
            console:          console,
            background_color: green,
            bounds:           bounds
        }
    }

    fn get_console(&mut self) -> &mut Console { &mut self.console     }
    fn get_bounds(&self)      ->      Bound   { self.bounds           }
    fn get_bg_color(&self)    ->      Color   { self.background_color }
}

pub struct TcodMapWindowComponent {
    pub console:          Console,
    pub background_color: Color,
    bounds:               Bound
}

impl WindowComponent for TcodMapWindowComponent {
    fn new(bounds: Bound) -> TcodMapWindowComponent {
        let height = bounds.max.y - bounds.min.y + 1;
        let width  = bounds.max.x - bounds.min.x + 1;
        let console = Console::new(
            width  as int,
            height as int,
        );

        let white = Color::new(255u8, 255u8, 255u8);
        TcodMapWindowComponent {
            console:          console,
            background_color: white,
            bounds:           bounds
        }
    }

    fn get_console(&mut self) -> &mut Console { &mut self.console     }
    fn get_bounds(&self)      ->      Bound   { self.bounds           }
    fn get_bg_color(&self)    ->      Color   { self.background_color }
}

pub struct TcodMessagesWindowComponent {
    pub console:          Console,
    pub background_color: Color,
    bounds:               Bound
}

impl WindowComponent for TcodMessagesWindowComponent {
    fn new(bounds: Bound) -> TcodMessagesWindowComponent {
        let height = bounds.max.y - bounds.min.y + 1;
        let width  = bounds.max.x - bounds.min.x + 1;
        let console = Console::new(
            width  as int,
            height as int,
        );

        let blue = Color::new(0u8, 0u8, 255u8);
        TcodMessagesWindowComponent {
            console:          console,
            background_color: blue,
            bounds:           bounds
        }
    }

    fn get_console(&mut self) -> &mut Console { &mut self.console     }
    fn get_bounds(&self)      ->      Bound   { self.bounds           }
    fn get_bg_color(&self)    ->      Color   { self.background_color }
}

pub struct TcodRenderingComponent {
    pub console: Console,
    bounds: Bound
}

impl RenderingComponent for TcodRenderingComponent {
    fn new(bounds: Bound) -> TcodRenderingComponent {
        let console = Console::init_root(
            (bounds.max.x + 1) as int,
            (bounds.max.y + 1) as int,
            "libtcod Rust tutorial", false
        );

        TcodRenderingComponent {
            console: console,
            bounds: bounds
        }
    }

    fn before_render_new_frame(&mut self) {
        self.console.clear();
    }
    
    fn attach_window(&mut self, window: &mut Box<WindowComponent>) {
        window.clear();
        window.print_message(0, 0, tcod::Left, "Sup foo!");
        window.print_message(0, 1, tcod::Left, "Nothin fool!");
        let bounds  = window.get_bounds();
        let console = window.get_console();

        Console::blit(&*console, 0, 0, (bounds.max.x as int) + 1, (bounds.max.y as int) + 1, &mut self.console, bounds.min.x as int, bounds.min.y as int, 1f32, 1f32);
    }

    fn render_object(&mut self, position: Point, symbol: char) {
        self.console.put_char(position.x as int, position.y as int, symbol, background_flag::Set);
    }

    fn after_render_new_frame(&mut self) {
        self.console.flush();
    }

    fn wait_for_keypress(&self) -> KeyState {
      self.console.wait_for_keypress(true)
    }
}
