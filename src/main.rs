extern crate tcod;
extern crate dwemthys;

use dwemthys::game::Game;
use dwemthys::actor::Actor;
use dwemthys::rendering::RenderingComponent;
use dwemthys::movement::MovementComponent;
use dwemthys::input::{SpecialKey, KeyCode};

use tcod::Console;

fn main() {
    let mut game = Game::new();
    let mut c = Actor::heroine(game.windows.map.get_bounds());
    let mut npcs: Vec<Box<Actor>> = vec![
        box Actor::dog(10, 10, game.windows.map.get_bounds()),
        box Actor::cat(40, 25, game.windows.map.get_bounds()),
        box Actor::kobold(20, 20, game.windows.map.get_bounds())
    ];

    game.render(&npcs, &c);
    while !(Console::window_closed() || game.exit) {
        // wait for user input
        let keypress = game.wait_for_keypress();

        // update game state
        match keypress.key {
            SpecialKey(KeyCode::Escape) => game.exit = true,
            _                           => {}
        }

        game.update(&mut npcs, &mut c);

        // render
        game.render(&npcs, &c);
    }
}
