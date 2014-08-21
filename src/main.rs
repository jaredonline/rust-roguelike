extern crate tcod;
extern crate dwemthys;

use dwemthys::game::Game;
use dwemthys::actor::Actor;
use dwemthys::movement::{AggroMovementComponent, RandomMovementComponent, MovementComponent};

use tcod::{Console, key_code, Special};

fn main() {
    let mut game = Game::new();
    let mc : Box<RandomMovementComponent> = box MovementComponent::new();
    let mut d = Actor::new(10, 10, 'd', mc);
    let mc : Box<RandomMovementComponent> = box MovementComponent::new();
    let mut ct = Actor::new(40, 25, 'c', mc);
    let mc : Box<AggroMovementComponent> = box MovementComponent::new();
    let mut kob = Actor::new(0, 0, 'k', mc);
    let npcs: Vec<&mut Actor> = vec![
        &mut d,
        &mut ct,
        &mut kob
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

        game.update(&npcs);
    }
}
