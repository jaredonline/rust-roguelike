extern crate tcod;
use tcod::{Console, RootConsole, BackgroundFlag,};
use tcod::input::Key;
use tcod::input::KeyCode::{Escape,};

fn main() {
    let mut con = RootConsole::initializer()
        .size(80, 50)
        .title("libtcod Rust tutorial")
        .init();

    let mut exit = false;
    while !(con.window_closed() || exit) {
        con.clear();
        con.put_char(40, 25, '@', BackgroundFlag::Set);
        con.flush();
        let keypress = con.wait_for_keypress(true);

        match keypress {
            Key { code: Escape, .. } => exit = true,
            _ => {}
        }
    }
}
