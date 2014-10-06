extern crate tcod;
use self::tcod::{Console, background_flag, KeyState, Color};
use input::{InputComponent, TcodInputComponent, KeyboardInput};

use util::{Point, Bound};

macro_rules! window_component_getters(
    () => {
        fn get_console(&mut self)      -> &mut Console          { &mut self.console     }
        fn get_bounds(&self)           ->      Bound            { self.bounds           }
        fn get_bg_color(&self)         ->      Color            { self.background_color }
        fn get_mut_messages(&mut self) -> &mut Vec<Box<String>> { &mut self.messages    } 
        fn get_max_messages(&self)     ->      uint             { self.max_messages     }
        fn get_messages(&self)         ->      Vec<Box<String>> { self.messages.clone() }
    }
)

macro_rules! window_component_def(
    ($name:ident) => {
        pub struct $name {
            pub console:          Console,
            pub background_color: Color,
            bounds:               Bound,
            messages:             Vec<Box<String>>,
            max_messages:         uint
        }
    }
)

macro_rules! window_component_init(
    ($name:ident, $color:expr, $max_messages:expr) => {
        fn new(bounds: Bound) -> $name {
            let height = bounds.max.y - bounds.min.y + 1;
            let width  = bounds.max.x - bounds.min.x + 1;
            let console = Console::new(
                width  as int,
                height as int,
            );

            $name {
                console:          console,
                background_color: $color,
                bounds:           bounds,
                messages:         vec![],
                max_messages:     $max_messages
            }
        }
    }
)

pub trait WindowComponent {
    fn new(Bound) -> Self;

    fn get_bounds(&self)       -> Bound;
    fn get_bg_color(&self)     -> Color;
    fn get_console(&mut self)  -> &mut Console;
    fn get_mut_messages(&mut self) -> &mut Vec<Box<String>>;
    fn get_max_messages(&self) -> uint;
    fn get_messages(&self) -> Vec<Box<String>>;

    fn clear(&mut self) {
        let color   = self.get_bg_color();
        let console = self.get_console();
        console.set_default_background(color);
        console.clear();
    }

    fn print_message(&mut self, x: int, y: int, alignment: tcod::TextAlignment, text: &str) {
        let console = self.get_console();
        console.print_ex(x, y, background_flag::Set, alignment, text);
    }

    fn buffer_message(&mut self, text: &str) {
        let max      = self.get_max_messages();
        let message  = String::from_str(text);
        let messages = self.get_mut_messages();

        messages.insert(0, box message);
        messages.truncate(max);
    }

    fn flush_buffer(&mut self) {
        let max      = self.get_max_messages();
        let messages = self.get_mut_messages();
        
        for _ in range(0, max) {
            messages.insert(0, box String::from_str(""));
        }
        messages.truncate(max);
    }
}


window_component_def!(TcodStatsWindowComponent)
impl WindowComponent for TcodStatsWindowComponent {
    window_component_init!(TcodStatsWindowComponent, Color::new(0u8, 0u8, 0u8), 10u)
    window_component_getters!()
}

window_component_def!(TcodInputWindowComponent)
impl WindowComponent for TcodInputWindowComponent {
    window_component_init!(TcodInputWindowComponent, Color::new(0u8, 0u8, 0u8), 2u)
    window_component_getters!()
}

window_component_def!(TcodMapWindowComponent)
impl WindowComponent for TcodMapWindowComponent {
    window_component_init!(TcodMapWindowComponent, Color::new(0u8, 0u8, 0u8), 10u)
    window_component_getters!()
}

window_component_def!(TcodMessagesWindowComponent)
impl WindowComponent for TcodMessagesWindowComponent {
    window_component_init!(TcodMessagesWindowComponent, Color::new(0u8, 0u8, 0u8), 10u)
    window_component_getters!()
}

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
