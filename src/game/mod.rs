extern crate tcod;

use util::{Bound, Point};
use rendering::{TcodRenderingComponent, RenderingComponent};
use traits::Updates;
use character::Character;

use self::tcod::KeyState;

static WIN_HEIGHT : i32 = 49;
static WIN_WIDTH  : i32 = 79;

pub struct Game {
    pub exit:                bool,
    pub window_bounds:       Bound,
    pub rendering_component: Box<RenderingComponent>
}

impl Game {
    pub fn new() -> Game {
        let bounds  = Bound { min: Point { x: 0, y: 0 }, max: Point { x: WIN_WIDTH, y: WIN_HEIGHT } };
        let rc : Box<TcodRenderingComponent> = box RenderingComponent::new(bounds);
        Game { 
            exit:                false,
            window_bounds:       bounds,
            rendering_component: rc
        }
    }

    pub fn wait_for_keypress(&self) -> KeyState {
        self.rendering_component.wait_for_keypress()
    }

    pub fn render(&mut self, objects: &Vec<&mut Updates>, character: Character) {
        self.rendering_component.before_render_new_frame();
        for i in objects.iter() {
            i.render(self.rendering_component);
        }
        character.render(self.rendering_component);
        self.rendering_component.after_render_new_frame();
    }

    pub fn update(&self, objects: &Vec<&mut Updates>, character: &mut Character, keypress: KeyState) {
        character.update(keypress, self);
        for &mut object in objects.iter() {
            object.update(self);
        }
    }
        
}
