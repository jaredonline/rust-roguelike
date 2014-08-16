extern crate tcod;
extern crate dwemthys;

use dwemthys::util::{Point, Bound};
use dwemthys::traits::Updates;
use dwemthys::game::Game;
use dwemthys::character::Character;
use dwemthys::npc::NPC;
use dwemthys::rendering::TcodRenderingComponent;

use tcod::{Console, key_code, Special};

fn render(con: &mut Console, objs: &Vec<&mut Updates>, c: Character) {
    let mut rendering_component = TcodRenderingComponent { console: con };
    rendering_component.before_render_new_frame();
    for i in objs.iter() {
        i.render(&mut rendering_component);
    }
    c.render(&mut rendering_component);
    rendering_component.after_render_new_frame();
}

fn update(objs: &Vec<&mut Updates>, c: &mut Character, keypress: tcod::KeyState, game: Game) {
    c.update(keypress, game);
    for &mut i in objs.iter() {
        i.update(game);
    }
}

fn main() {
    let mut game = Game { exit: false, window_bounds: Bound { min: Point { x: 0, y: 0 }, max: Point { x: 79, y: 49 } } };
    let mut con = Console::init_root((game.window_bounds.max.x + 1) as int, (game.window_bounds.max.y + 1) as int, "libtcod Rust tutorial", false);
    let mut c = Character::new(40, 25, '@');
    let mut d = NPC::new(10, 10, 'd');
    let mut ct = NPC::new(40, 25, 'c');
    let npcs: Vec<&mut Updates> = vec![
        &mut d as &mut Updates,
        &mut ct as &mut Updates,
    ];

    render(&mut con, &npcs, c);
    while !(Console::window_closed() || game.exit) {
        // wait for user input
        let keypress = con.wait_for_keypress(true);

        // update game state
        match keypress.key {
            Special(key_code::Escape) => game.exit = true,
            _                         => {}
        }
        update(&npcs, &mut c, keypress, game);

        // render
        render(&mut con, &npcs, c);
    }
}
