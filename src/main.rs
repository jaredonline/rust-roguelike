extern crate tcod;
extern crate dwemthys;

use dwemthys::util::{Point, Bound};
use dwemthys::traits::Updates;
use dwemthys::game::Game;
use dwemthys::character::Character;
use dwemthys::npc::NPC;

use tcod::{Console, key_code, Special};

fn render(con: &mut Console, objs: &Vec<Box<Updates>>) {
    con.clear();
    for i in objs.iter() {
        i.render(con);
    }
    con.flush();
}

fn update(objs: &mut Vec<Box<Updates>>, keypress: tcod::KeyState, game: Game) {
    for i in objs.mut_iter() {
        i.update(keypress, game);
    }
}

fn main() {
    let mut game = Game { exit: false, window_bounds: Bound { min: Point { x: 0, y: 0 }, max: Point { x: 79, y: 49 } } };
    let mut con = Console::init_root(game.window_bounds.max.x + 1, game.window_bounds.max.y + 1, "libtcod Rust tutorial", false);
    let c = box Character::new(40, 25, '@') as Box<Updates>;
    let d = box NPC::new(10, 10, 'd') as Box<Updates>;
    let mut objs: Vec<Box<Updates>> = vec![
        c, d
    ];

    render(&mut con, &objs);
    while !(Console::window_closed() || game.exit) {
        // wait for user input
        let keypress = con.wait_for_keypress(true);

        // update game state
        match keypress.key {
            Special(key_code::Escape) => game.exit = true,
            _                         => {}
        }
        update(&mut objs, keypress, game);

        // render
        render(&mut con, &objs);
    }
}
