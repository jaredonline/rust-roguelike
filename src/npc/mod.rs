extern crate tcod;
use self::tcod::{Console, background_flag};

use traits::Updates;
use util::{Point, DoesContain, DoesNotContain};
use game::Game;

use std;
use std::rand::Rng;

pub struct NPC {
    position:     Point,
    display_char: char
}

impl NPC {
    pub fn new(x: int, y: int, dc: char) -> NPC {
        NPC { position: Point { x: x, y: y }, display_char: dc }
    }
}

impl Updates for NPC {
    fn update(&mut self, keypress: tcod::KeyState, game: Game) {
        let offset_x = std::rand::task_rng().gen_range(0, 3i) - 1;
        match game.window_bounds.contains(self.position.offset_x(offset_x)) {
            DoesContain    => self.position = self.position.offset_x(offset_x),
            DoesNotContain => {}
        }
        
        let offset_y = std::rand::task_rng().gen_range(0, 3i) - 1;
        match game.window_bounds.contains(self.position.offset_y(offset_y)) {
            DoesContain    => self.position = self.position.offset_y(offset_y),
            DoesNotContain => {}
        }
    }

    fn render(&self, console: &mut Console) {
        console.put_char(self.position.x, self.position.y, self.display_char, background_flag::Set);
    }
}

