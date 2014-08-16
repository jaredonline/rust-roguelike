extern crate tcod;
use self::tcod::{key_code, Special};

use util::{Point, DoesContain, DoesNotContain};
use game::Game;
use rendering::TcodRenderingComponent;

pub struct Character {
    pub position:     Point,
    pub display_char: char
}

impl Character {
    pub fn new(x: i32, y: i32, dc: char) -> Character {
        Character { position: Point { x: x, y: y }, display_char: dc }
    }

    pub fn update(&mut self, keypress: tcod::KeyState, game: Game) {
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

    pub fn render(&self, rendering_component: &mut TcodRenderingComponent) {
        rendering_component.render_object(self.position, self.display_char);
    }
}
