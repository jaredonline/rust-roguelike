extern crate tcod;
use self::tcod::{Console, background_flag, Color};

use util::{Bound};

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
            pub bounds:           Bound,
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

pub struct Windows<'a> {
    pub stats:    Box<WindowComponent + 'a>,
    pub map:      Box<WindowComponent + 'a>,
    pub input:    Box<WindowComponent + 'a>,
    pub messages: Box<WindowComponent + 'a>
}

impl<'a > Windows<'a > {
    pub fn all_windows(&'a mut self) -> Vec<&mut Box<WindowComponent>> {
        let windows = vec![
            &mut self.stats,
            &mut self.input,
            &mut self.messages,
            &mut self.map
        ];

        return windows;
    }

    pub fn get_map_bounds(&self) -> Bound {
        self.map.get_bounds()
    }
}
