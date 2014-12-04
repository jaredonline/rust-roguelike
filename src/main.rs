extern crate tcod;
extern crate dwemthys;

use dwemthys::game::Game;
use dwemthys::input::{SpecialKey, KeyCode};
use tcod::Console;

#[cfg(not(test))]
fn main() {
    let mut game = Game::new();

    game.render();
    while !(Console::window_closed() || game.exit) {
        // wait for user input
        let keypress = game.wait_for_keypress();

        // update game state
        match keypress.key {
            SpecialKey(KeyCode::Escape) => game.exit = true,
            _                           => {}
        }
        game.update();

        // render
        game.render();
    }
}
