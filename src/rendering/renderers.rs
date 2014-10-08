extern crate tcod;
use self::tcod::{Console, background_flag, KeyState,};
use input::{InputComponent, TcodInputComponent, KeyboardInput};
use util::{Point, Bound};

use rendering::windows::WindowComponent;

pub trait RenderingComponent {
    fn new(Bound) -> Self;
    fn before_render_new_frame(&mut self);
    fn render_object(&mut self, Point, char);
    fn after_render_new_frame(&mut self);
    fn wait_for_keypress(&self) -> KeyboardInput;
    fn attach_window(&mut self, &mut Box<WindowComponent>);
}

pub struct TcodRenderingComponent<'a> {
    pub console: Console,
    pub input_component: Box<InputComponent<KeyState> + 'a>
}

impl<'a> RenderingComponent for TcodRenderingComponent<'a> {
    fn new(bounds: Bound) -> TcodRenderingComponent<'a> {
        let console = Console::init_root(
            (bounds.max.x + 1) as int,
            (bounds.max.y + 1) as int,
            "libtcod Rust tutorial", false
        );

        let ic : Box<TcodInputComponent> = box InputComponent::new();

        TcodRenderingComponent {
            console: console,
            input_component: ic
        }
    }

    fn before_render_new_frame(&mut self) {
        self.console.clear();
    }
    
    fn attach_window(&mut self, window: &mut Box<WindowComponent>) {
        window.clear();
        let mut line = 0i;
        let bounds   = window.get_bounds();
        let messages = window.get_messages();

        for message in messages.iter() {
            window.print_message(0, line, tcod::Left, message.as_slice());
            line = line + 1;
        }

        let console  = window.get_console();

        Console::blit(&*console, 0, 0, (bounds.max.x as int) + 1, (bounds.max.y as int) + 1, &mut self.console, bounds.min.x as int, bounds.min.y as int, 1f32, 1f32);
    }

    fn render_object(&mut self, position: Point, symbol: char) {
        self.console.put_char(position.x as int, position.y as int, symbol, background_flag::Set);
    }

    fn after_render_new_frame(&mut self) {
        self.console.flush();
    }

    fn wait_for_keypress(&self) -> KeyboardInput {
      let ks = self.console.wait_for_keypress(true);
      self.input_component.translate_input(ks)
    }
}
