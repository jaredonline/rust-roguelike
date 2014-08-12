extern crate tcod;
use tcod::{Console, background_flag, key_code, Special};

fn render(con: &mut Console) {
    con.clear();
    con.put_char(40, 25, '@', background_flag::Set);
    con.flush();
}

fn main() {
    let mut con = Console::init_root(80, 50, "libtcod Rust tutorial", false);
    let mut exit = false;
    // render
    render(&mut con);
    while !(Console::window_closed() || exit) {
        // wait for user input
        let keypress = con.wait_for_keypress(true);

        // update game state
        match keypress.key {
            Special(key_code::Escape) => exit = true,
            _ => {}
        }

        // render
        render(&mut con);
    }
}
