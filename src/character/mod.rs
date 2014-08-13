extern crate tcod;
use self::tcod::{Console, background_flag, key_code, Special};

use traits::Updates;
use util::{Point, DoesContain, DoesNotContain};
use game::Game;

pub struct Character {
    pub position:     Point,
    pub display_char: char
}

impl Character {
    pub fn new(x: int, y: int, dc: char) -> Character {
        Character { position: Point { x: x, y: y }, display_char: dc }
    }
}

impl Updates for Character {
    fn update(&mut self, keypress: tcod::KeyState, game: Game) {
        let mut offset = Point { x: 0, y: 0 };
        match keypress.key {
            Special(key_code::Up) => {
                offset.y = -1;
            },
            Special(key_code::Down) => {
                offset.y = 1;
            },
            Special(key_code::Left) => {
                offset.x = -1;
            },
            Special(key_code::Right) => {
                offset.x = 1;
            },
            _ => {}
        }

        match game.window_bounds.contains(self.position.offset(offset)) {
            DoesContain    => self.position = self.position.offset(offset),
            DoesNotContain => {}
        }
    }

    fn render(&self, console: &mut Console) {
        console.put_char(self.position.x, self.position.y, self.display_char, background_flag::Set);
    }
}
