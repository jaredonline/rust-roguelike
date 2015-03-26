extern crate dwemthys;

use dwemthys::game::Game;
use dwemthys::movement::{MovementComponent};
use dwemthys::actor::Actor;
use dwemthys::rendering::{RenderingComponent};

fn main() {
    let pcs = vec![
        Actor::heroine()
    ];

    let npcs = vec![
        Actor::dog(),
        Actor::cat(),
        Actor::kobold()
    ];

    let mut game = Game::new();

    // game loop!
    game.render(&pcs, &npcs);
    while !(game.exit()) {
        game.wait_for_keypress();
        game.update(&pcs, &npcs);
        game.render(&pcs, &npcs);
    }
}
