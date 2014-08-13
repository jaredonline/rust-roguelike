extern crate tcod;
extern crate dwemthys;

use dwemthys::util::{Point, Bound};
use dwemthys::traits::Updates;
use dwemthys::game::Game;
use dwemthys::character::Character;
use dwemthys::npc::NPC;

use tcod::{Console, key_code, Special};

fn render(con: &mut Console, objs: &Vec<&mut Updates>) {
    con.clear();
    for i in objs.iter() {
        i.render(con);
    }
    con.flush();
}

fn update(objs: &Vec<&mut Updates>, keypress: tcod::KeyState, game: Game) {
    for &mut i in objs.iter() {
        i.update(keypress, game);
    }
}

fn main() {
    let mut game = Game { exit: false, window_bounds: Bound { min: Point { x: 0, y: 0 }, max: Point { x: 79, y: 49 } } };
    let mut con = Console::init_root(game.window_bounds.max.x + 1, game.window_bounds.max.y + 1, "libtcod Rust tutorial", false);
    let mut c = Character::new(40, 25, '@');
    let mut d = NPC::new(10, 10, 'd');
    let objs: Vec<&mut Updates> = vec![
        &mut d as &mut Updates,
        &mut c as &mut Updates
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
        update(&objs, keypress, game);

        // render
        render(&mut con, &objs);
    }
}
