extern crate tcod;
extern crate dwemthys;

use dwemthys::game::Game;
use dwemthys::actor::Actor;

use tcod::{Console, key_code, Special};

fn main() {
    let mut game = Game::new();
    let mut npcs: Vec<Box<Actor>> = vec![
        box Actor::dog(),
        box Actor::cat(),
        box Actor::kobold()
    ];

    while !(Console::window_closed() || game.exit) {
        game.render(&npcs);

        // wait for user input
        let keypress = game.wait_for_keypress();

        // update game state
        match keypress.key {
            Special(key_code::Escape) => game.exit = true,
            _                         => {}
        }

        game.update(&mut npcs);
    }
}
