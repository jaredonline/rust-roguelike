extern crate tcod;
use self::tcod::{Console};
use game::Game;

pub trait Updates {
    fn update(&mut self, tcod::KeyState, Game);
    fn render(&self, &mut Console);
}
