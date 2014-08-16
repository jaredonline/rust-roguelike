extern crate tcod;
use self::tcod::{Console, background_flag, KeyState};

use util::{Point, Bound};

pub trait RenderingComponent {
    fn new(Bound) -> Self;
    fn before_render_new_frame(&mut self);
    fn render_object(&mut self, Point, char);
    fn after_render_new_frame(&mut self);
    fn wait_for_keypress(&self) -> KeyState;
}

pub struct TcodRenderingComponent {
    pub console: Console
}

impl RenderingComponent for TcodRenderingComponent {
    fn new(bounds: Bound) -> TcodRenderingComponent {
        let console = Console::init_root(
            (bounds.max.x + 1) as int,
            (bounds.max.y + 1) as int,
            "libtcod Rust tutorial", false
        );

        TcodRenderingComponent {
            console: console
        }
    }

    fn before_render_new_frame(&mut self) {
        self.console.clear();
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

