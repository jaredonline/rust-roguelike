extern crate tcod;
extern crate dwemthys;

use dwemthys::traits::Updates;
use dwemthys::game::Game;
use dwemthys::character::Character;
use dwemthys::npc::NPC;

use tcod::{Console, key_code, Special};

fn main() {
    let mut c = Character::new(40, 25, '@');
    let mut d = NPC::new(10, 10, 'd');
    let mut ct = NPC::new(40, 25, 'c');
    let mut game = Game::new();
    let npcs: Vec<&mut Updates> = vec![
        &mut d as &mut Updates,
        &mut ct as &mut Updates,
    ];

    while !(Console::window_closed() || game.exit) {
        game.render(&npcs, c);

        // wait for user input
        let keypress = game.wait_for_keypress();

        // update game state
        match keypress.key {
            Special(key_code::Escape) => game.exit = true,
            _                         => {}
        }

        game.update(&npcs, &mut c, keypress);
    }
}
