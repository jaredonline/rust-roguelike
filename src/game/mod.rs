extern crate tcod;

use util::{Point, Bound};
use rendering::{RenderingComponent, WindowComponent};
use rendering::{
    TcodRenderingComponent,
    TcodStatsWindowComponent,
    TcodInputWindowComponent,
    TcodMapWindowComponent,
    TcodMessagesWindowComponent
};
use actor::Actor;

use self::tcod::KeyState;

static mut LAST_KEYPRESS : Option<KeyState> = None;
static mut CHAR_LOCATION : Point = Point { x: 40, y: 25 };

pub struct Game {
    pub exit:                bool,
    pub window_bounds:       Bound,
    pub rendering_component: Box<RenderingComponent>,
    pub stats_window:        Box<WindowComponent>,
    pub map_window:          Box<WindowComponent>,
    pub input_window:        Box<WindowComponent>,
    pub messages_window:     Box<WindowComponent>
}

impl Game {
    pub fn get_last_keypress() -> Option<KeyState> {
        unsafe { LAST_KEYPRESS }
    }

    pub fn set_last_keypress(ks: KeyState) {
        unsafe { LAST_KEYPRESS = Some(ks); }
    }

    pub fn get_character_point() -> Point {
        unsafe { CHAR_LOCATION }
    }

    pub fn set_character_point(point: Point) {
        unsafe { CHAR_LOCATION = point; }
    }

    pub fn new() -> Game {
        let total_bounds   = Bound::new(0,  0, 99, 61);
        let stats_bounds   = Bound::new(79, 0, 99, 49);
        let input_bounds   = Bound::new(0, 50, 99, 52);
        let message_bounds = Bound::new(0, 53, 99, 61);
        let map_bounds     = Bound::new(0,  0, 78, 49);

        let rc  : Box<TcodRenderingComponent>      = box RenderingComponent::new(total_bounds);
        let sw  : Box<TcodStatsWindowComponent>    = box WindowComponent::new(stats_bounds);
        let iw  : Box<TcodInputWindowComponent>    = box WindowComponent::new(input_bounds);
        let mw  : Box<TcodMessagesWindowComponent> = box WindowComponent::new(message_bounds);
        let maw : Box<TcodMapWindowComponent>      = box WindowComponent::new(map_bounds);

        Game {
            exit:                false,
            window_bounds:       total_bounds,
            rendering_component: rc,
            stats_window:        sw,
            input_window:        iw,
            messages_window:     mw,
            map_window:          maw
        }
    }

    pub fn render(&mut self, npcs: &Vec<Box<Actor>>, c: &Actor) {
        self.rendering_component.before_render_new_frame();
        self.rendering_component.attach_window(&mut self.stats_window);
        self.rendering_component.attach_window(&mut self.input_window);
        self.rendering_component.attach_window(&mut self.messages_window);
        self.rendering_component.attach_window(&mut self.map_window);
        for i in npcs.iter() {
            i.render(self.rendering_component);
        }
        c.render(self.rendering_component);
        self.rendering_component.after_render_new_frame();
    }

    pub fn update(&mut self, npcs: &mut Vec<Box<Actor>>, c: &mut Actor) {
        c.update();
        Game::set_character_point(c.position);
        for i in npcs.mut_iter() {
            i.update();
        }
    }

    pub fn wait_for_keypress(&mut self) -> KeyState {
        let ks = self.rendering_component.wait_for_keypress();
        Game::set_last_keypress(ks);
        return ks;
    }
}
