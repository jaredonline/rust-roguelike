#![feature(core)]

extern crate dwemthys;
extern crate tcod;
extern crate core;

use tcod::Key::Special;
use tcod::{Console, KeyCode};

use std::sync::{Arc, Mutex, RwLock};
use std::thread;

use dwemthys::util::{Point, Bound};
use dwemthys::game::Game;
use dwemthys::movement::{MovementComponent};
use dwemthys::actor::Actor;

fn render(con: &mut Console, actors: &Vec<Arc<Mutex<Actor>>>) {
    con.clear();
    for i in actors {
        i.lock().unwrap().render(con);
    }
    Console::flush();
}

fn update(actors: &Vec<Arc<Mutex<Actor>>>, game: Arc<RwLock<Game>>) {
    let _ : Vec<_> = actors.iter().map(|a| {
        let actor = a.clone();
        let game  = game.clone();

        thread::spawn(move || {
            actor.lock().unwrap().update(game);
        })
    }).collect();
}

fn main() {
    let window_bounds = Bound { min: Point { x: 0, y: 0 }, max: Point { x: 79, y: 49 } };
    let mut con = Console::init_root(window_bounds.max.x, window_bounds.max.y, "libtcod Rust tutorial", false);
    let mut exit = false;

    let ch  = Actor::heroine();
    let dog = Actor::dog();

    let updates = vec![
        dog,
        ch
    ];

    let game = Arc::new(RwLock::new(Game {
        window_bounds: window_bounds,
        keypress:      None
    }));

    //render
    while !(Console::window_closed() || exit) {
        render(&mut con, &updates);

        // wait for keypress
        let keypress = Console::wait_for_keypress(true);
        match keypress.key {
            Special(KeyCode::Escape) => { exit = true },
            _ => {}
        }

        {
            game.write().unwrap().keypress = Some(keypress);
        }


        update(&updates, game.clone());
    }
}
