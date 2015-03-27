extern crate dwemthys;

use dwemthys::game::Game;

fn main() {
    let mut game = Game::new();
    game.render();

    // game loop!
    while !(game.exit()) {
        game.wait_for_keypress();
        game.update();
        game.render();
    }
}
