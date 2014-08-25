extern crate tcod;

use util::{Point, Bound};
use rendering::{TcodRenderingComponent, RenderingComponent};
use actor::Actor;

use self::tcod::KeyState;

static mut LAST_KEYPRESS : Option<KeyState> = None;
static mut CHAR_LOCATION : Point = Point { x: 40, y: 25 };

pub struct Game {
    pub exit:                bool,
    pub window_bounds:       Bound,
    pub rendering_component: Box<RenderingComponent>
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
        let  bounds = Bound {
            min: Point { x: 0, y: 0 },
            max: Point { x: 79, y: 49 }
        };
        let rc : Box<TcodRenderingComponent> = box RenderingComponent::new(bounds);
        Game {
            exit: false,
            window_bounds: bounds,
            rendering_component: rc
        }
    }

    pub fn render(&mut self, npcs: &Vec<Box<Actor>>, c: &Actor) {
        self.rendering_component.before_render_new_frame();
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
