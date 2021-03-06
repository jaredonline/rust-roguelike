#![allow(non_snake_case)]
extern crate tcod;
use self::tcod::{Console, KeyState,};
use input::{InputComponent, TcodInputComponent, KeyboardInput};
use util::{Point, Bound};

use rendering::windows::WindowComponent;

pub enum Color {
    Red,
    Blue,
    Black,
    White
}

pub trait RenderingComponent {
    fn before_render_new_frame(&mut self);
    fn render_object(&mut self, Point, char, Color, Color);
    fn after_render_new_frame(&mut self);
    fn wait_for_keypress(&self) -> KeyboardInput;
    fn attach_window(&mut self, &mut Box<WindowComponent>);

    fn translate_color(&self, Color) -> tcod::Color;
}

pub struct TcodRenderingComponent {
    pub console: Console,
    pub input_component: Box<InputComponent<KeyState> + 'static>
}

impl TcodRenderingComponent {
    pub fn new(bounds: Bound) -> TcodRenderingComponent {
        let console = Console::init_root(
            (bounds.max.x + 1) as i32,
            (bounds.max.y + 1) as i32,
            "libtcod Rust tutorial", false
        );

        let ic = Box::new(TcodInputComponent::new());

        TcodRenderingComponent {
            console: console,
            input_component: ic
        }
    }
}

impl RenderingComponent for TcodRenderingComponent {
    fn before_render_new_frame(&mut self) {
        self.console.clear();
    }
    
    fn attach_window(&mut self, window: &mut Box<WindowComponent>) {
        window.clear();
        let mut line = 0i32;
        let bounds   = window.get_bounds();
        let messages = window.get_messages();

        for message in messages.iter() {
            window.print_message(0, line, tcod::TextAlignment::Left, message.as_slice());
            line = line + 1;
        }

        let console  = window.get_console();

        Console::blit(&*console, 0, 0, (bounds.max.x as i32) + 1, (bounds.max.y as i32) + 1, &mut self.console, bounds.min.x as i32, bounds.min.y as i32, 1f32, 1f32);
    }

    fn render_object(&mut self, position: Point, symbol: char, foreground: Color, background: Color) {
        let f = self.translate_color(foreground);
        let b = self.translate_color(background);
        self.console.put_char_ex(position.x as i32, position.y as i32, symbol, f, b);
    }

    fn after_render_new_frame(&mut self) {
        Console::flush();
    }

    fn wait_for_keypress(&self) -> KeyboardInput {
      let ks = Console::wait_for_keypress(true);
      self.input_component.translate_input(ks)
    }

    fn translate_color(&self, input: Color) -> tcod::Color {
        match input {
            Color::Red   => tcod::Color::new(255u8, 0u8, 0u8),
            Color::Blue  => tcod::Color::new(0u8, 0u8, 255u8),
            Color::White => tcod::Color::new(255u8, 255u8, 255u8),
            Color::Black => tcod::Color::new(0u8, 0u8, 0u8)
        }
    }
}
