extern crate tcod;

use self::tcod::KeyState;
use util::Bound;

pub struct Game {
    pub window_bounds: Bound,
    pub keypress:      Option<KeyState>
}
