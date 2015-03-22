extern crate tcod;
use self::tcod::{Console, BackgroundFlag,};

use util::{Bound};

macro_rules! window_component_getters(
    () => {
        fn get_console(&mut self)      -> &mut Console          { &mut self.console     }
        fn get_bounds(&self)           ->      Bound            { self.bounds           }
        fn get_bg_color(&self)         ->      tcod::Color      { self.background_color }
        fn get_fg_color(&self)         ->      tcod::Color      { self.foreground_color }
        fn get_mut_messages(&mut self) -> &mut Vec<Box<String>> { &mut self.messages    } 
        fn get_max_messages(&self)     ->      usize             { self.max_messages     }
        fn get_messages(&self)         ->      Vec<Box<String>> { self.messages.clone() }
    };
);

macro_rules! window_component_def(
    ($name:ident) => {
        pub struct $name {
            pub console:          Console,
            pub background_color: tcod::Color,
            pub foreground_color: tcod::Color,
            pub bounds:           Bound,
            messages:             Vec<Box<String>>,
            max_messages:         usize
        }
    };
);

macro_rules! window_component_init(
    ($name:ident, $bg_color:expr, $fg_color:expr, $max_messages:expr) => {
        pub fn new(bounds: Bound) -> $name {
            let height = bounds.max.y - bounds.min.y + 1;
            let width  = bounds.max.x - bounds.min.x + 1;
            let console = Console::new(
                width  as i32,
                height as i32,
            );

            $name {
                console:          console,
                background_color: $bg_color,
                foreground_color: $fg_color,
                bounds:           bounds,
                messages:         vec![],
                max_messages:     $max_messages
            }
        }
    };
);

pub trait WindowComponent {
    fn get_bounds(&self)           -> Bound;
    fn get_bg_color(&self)         -> tcod::Color;
    fn get_fg_color(&self)         -> tcod::Color;
    fn get_console(&mut self)      -> &mut Console;
    fn get_mut_messages(&mut self) -> &mut Vec<Box<String>>;
    fn get_max_messages(&self)     -> usize;
    fn get_messages(&self)         -> Vec<Box<String>>;

    fn clear(&mut self) {
        let bg_color = self.get_bg_color();
        let fg_color = self.get_fg_color();
        let console  = self.get_console();
        console.set_default_background(bg_color);
        console.set_default_foreground(fg_color);
        console.clear();
    }

    fn print_message(&mut self, x: i32, y: i32, alignment: tcod::TextAlignment, text: &str) {
        let console = self.get_console();
        console.print_ex(x, y, BackgroundFlag::Set, alignment, text);
    }

    fn buffer_message(&mut self, text: &str) {
        let max      = self.get_max_messages();
        let message  = String::from_str(text);
        let messages = self.get_mut_messages();

        messages.insert(0, Box::new(message));
        messages.truncate(max);
    }

    fn flush_buffer(&mut self) {
        let max      = self.get_max_messages();
        let messages = self.get_mut_messages();
        
        for _ in range(0, max) {
            messages.insert(0, Box::new(String::from_str("")));
        }
        messages.truncate(max);
    }
}


window_component_def!(TcodStatsWindowComponent);
impl TcodStatsWindowComponent {
    window_component_init!(TcodStatsWindowComponent, tcod::Color::new(0u8, 0u8, 0u8), tcod::Color::new(255u8, 255u8, 255u8), 10u);
}

impl WindowComponent for TcodStatsWindowComponent {
    window_component_getters!();
}

window_component_def!(TcodInputWindowComponent);
impl TcodInputWindowComponent {
    window_component_init!(TcodInputWindowComponent, tcod::Color::new(255u8, 255u8, 255u8), tcod::Color::new(0u8, 0u8, 0u8), 1u);
}

impl WindowComponent for TcodInputWindowComponent {
    window_component_getters!();
}

window_component_def!(TcodMapWindowComponent);
impl TcodMapWindowComponent {
    window_component_init!(TcodMapWindowComponent, tcod::Color::new(0u8, 0u8, 0u8), tcod::Color::new(255u8, 255u8, 255u8), 10u);
}

impl WindowComponent for TcodMapWindowComponent {
    window_component_getters!();
}

window_component_def!(TcodMessagesWindowComponent);
impl TcodMessagesWindowComponent {
    window_component_init!(TcodMessagesWindowComponent, tcod::Color::new(0u8, 0u8, 0u8), tcod::Color::new(160u8, 160u8, 160u8), 10u);
}

impl WindowComponent for TcodMessagesWindowComponent {
    window_component_getters!();
}

pub struct Windows {
    pub stats:    Box<WindowComponent + 'static>,
    pub map:      Box<WindowComponent + 'static>,
    pub input:    Box<WindowComponent + 'static>,
    pub messages: Box<WindowComponent + 'static>
}

impl Windows {
    pub fn get_map_bounds(&self) -> Bound {
        self.map.get_bounds()
    }
}
