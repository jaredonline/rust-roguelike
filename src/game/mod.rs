extern crate tcod;

use util::{Bound, Point, Contains};
use rendering::{TcodRenderingComponent, RenderingComponent};
use movement::{TcodUserMovementComponent, MovementComponent};
use actor::Actor;

use self::tcod::KeyState;

static WIN_HEIGHT : i32 = 49;
static WIN_WIDTH  : i32 = 79;
static mut LAST_KEYPRESS : Option<KeyState> = None;
static BOUNDS : Bound = Bound {
    min: Point { x: 0, y: 0 },
    max: Point { x: WIN_WIDTH, y: WIN_HEIGHT }
};

pub struct Game {
    pub exit:                bool,
    pub rendering_component: Box<RenderingComponent>,
    pub character:           Actor
}

impl Game {
    pub fn last_keypress() -> Option<KeyState> {
        unsafe { LAST_KEYPRESS }
    }

    pub fn set_keypress(kp: KeyState) {
        unsafe { LAST_KEYPRESS = Some(kp); }
    }

    pub fn bounds_contain(point: Point) -> Contains {
        BOUNDS.contains(point)
    }

    pub fn new() -> Game {
        let ic : Box<TcodUserMovementComponent> = box MovementComponent::new();
        let c = Actor::new(40, 25, '@', ic);
        let rc : Box<TcodRenderingComponent> = box RenderingComponent::new(BOUNDS);
        Game { 
            exit:                false,
            rendering_component: rc,
            character:           c
        }
    }

    pub fn wait_for_keypress(&self) -> KeyState {
        let keypress = self.rendering_component.wait_for_keypress();
        Game::set_keypress(keypress);
        keypress
    }

    pub fn render(&mut self, objects: &Vec<&mut Actor>) {
        self.rendering_component.before_render_new_frame();
        for i in objects.iter() {
            i.render(self.rendering_component);
        }
        self.character.render(self.rendering_component);
        self.rendering_component.after_render_new_frame();
    }

    pub fn update(&mut self, objects: &Vec<&mut Actor>) {
        self.character.update();
        for &mut object in objects.iter() {
            object.update();
        }
    }
        
}
