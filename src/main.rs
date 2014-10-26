extern crate tcod;
extern crate dwemthys;

use dwemthys::game::Game;
use dwemthys::actor::Actor;
use dwemthys::rendering::renderers::RenderingComponent;
use dwemthys::movement::MovementComponent;
use dwemthys::input::{SpecialKey, KeyCode};
use dwemthys::util::Point;

use tcod::Console;

fn main() {
    let mut game = Game::new();
    game.maps.friends.push_actor(Point::new(10, 10), box Actor::dog(10, 10, game.move_info.clone()));
    game.maps.friends.push_actor(Point::new(40, 25), box Actor::cat(40, 25, game.move_info.clone()));
    game.maps.enemies.push_actor(Point::new(20, 20), box Actor::kobold(20, 20, game.move_info.clone()));

    let char_location = {
        game.move_info.borrow().deref().char_location
    };
    game.maps.pcs.push_actor(char_location, box Actor::heroine(game.move_info.clone()));

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
