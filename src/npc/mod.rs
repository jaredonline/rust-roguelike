use traits::Updates;
use util::{Point, DoesContain, DoesNotContain};
use game::Game;
use rendering::TcodRenderingComponent;

use std;
use std::rand::Rng;

pub struct NPC {
    position:     Point,
    display_char: char
}

impl NPC {
    pub fn new(x: i32, y: i32, dc: char) -> NPC {
        NPC { position: Point { x: x, y: y }, display_char: dc }
    }
}

impl Updates for NPC {
    fn update(&mut self, game: Game) {
        let offset_x = std::rand::task_rng().gen_range(0, 3i32) - 1;
        match game.window_bounds.contains(self.position.offset_x(offset_x)) {
            DoesContain    => self.position = self.position.offset_x(offset_x),
            DoesNotContain => {}
        }
        
        let offset_y = std::rand::task_rng().gen_range(0, 3i32) - 1;
        match game.window_bounds.contains(self.position.offset_y(offset_y)) {
            DoesContain    => self.position = self.position.offset_y(offset_y),
            DoesNotContain => {}
        }
    }

    fn render(&self, rendering_component: &mut TcodRenderingComponent) {
        rendering_component.render_object(self.position, self.display_char);
    }
}

