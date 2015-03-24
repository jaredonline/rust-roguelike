extern crate tcod;

use self::tcod::{Console, BackgroundFlag, KeyState};

use util::{Point, Bound};
use input::{InputComponent, TcodInputComponent, KeyboardInput};

pub trait RenderingComponent {
    fn before_render_new_frame(&mut self);
    fn render_object(&mut self, &Point, char);
    fn after_render_new_frame(&mut self);
    fn wait_for_keypress(&self) -> KeyboardInput;
    fn closed(&self) -> bool;
}

pub struct TcodRenderingComponent {
    input:   Box<InputComponent<self::tcod::KeyState> + 'static>,
    console: Console
}

impl TcodRenderingComponent {
    pub fn new(bound: &Bound) -> TcodRenderingComponent {
        let input = Box::new(TcodInputComponent::new());
        let con = Console::init_root(bound.max.x, bound.max.y, "libtcod Rust tutorial", false);
        TcodRenderingComponent {
            input:   input,
            console: con
        }
    }
}

impl RenderingComponent for TcodRenderingComponent {
    fn before_render_new_frame(&mut self) {
        self.console.clear();
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
}
