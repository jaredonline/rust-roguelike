extern crate tcod;
use tcod::{Console, background_flag, key_code, Special};
use std::rand::Rng;

struct Point {
    x: int,
    y: int
}

impl Point {
    fn offset_x(&self, offset: int) -> Point {
        Point { x: self.x + offset, y: self.y }
    }

    fn offset_y(&self, offset: int) -> Point {
        Point { x: self.x, y: self.y + offset }
    }

    fn offset(&self, offset: Point) -> Point {
        Point { x: self.x + offset.x, y: self.y + offset.y }
    }
}

struct Bound {
    min: Point,
    max: Point
}

enum Contains {
    DoesContain,
    DoesNotContain
}

impl Bound {
    fn contains(&self, point: Point) -> Contains {
        if 
            point.x >= self.min.x &&
            point.x <= self.max.x &&
            point.y >= self.min.y &&
            point.y <= self.max.y
        {
            DoesContain
        } else {
            DoesNotContain
        }
    }
}

struct Game {
    exit:          bool,
    window_bounds: Bound
}

struct Character {
    position:     Point,
    display_char: char
}

impl Character {
    fn new(x: int, y: int, dc: char) -> Character {
        Character { position: Point { x: x, y: y }, display_char: dc }
    }
}

struct NPC {
    position:     Point,
    display_char: char
}

impl NPC {
    fn new(x: int, y: int, dc: char) -> NPC {
        NPC { position: Point { x: x, y: y }, display_char: dc }
    }
}

trait Updates{
    fn update(&mut self, tcod::KeyState, Game);
    fn render(&self, &mut Console);
}

impl Updates for Character {
    fn update(&mut self, keypress: tcod::KeyState, game: Game) {
        let mut offset = Point { x: 0, y: 0 };
        match keypress.key {
            Special(key_code::Up) => {
                offset.y = -1;
            },
            Special(key_code::Down) => {
                offset.y = 1;
            },
            Special(key_code::Left) => {
                offset.x = -1;
            },
            Special(key_code::Right) => {
                offset.x = 1;
            },
            _ => {}
        }

        match game.window_bounds.contains(self.position.offset(offset)) {
            DoesContain    => self.position = self.position.offset(offset),
            DoesNotContain => {}
        }
    }

    fn render(&self, console: &mut Console) {
        console.put_char(self.position.x, self.position.y, self.display_char, background_flag::Set);
    }
}

impl Updates for NPC {
    fn update(&mut self, keypress: tcod::KeyState, game: Game) {
        let offset_x = std::rand::task_rng().gen_range(0, 3i) - 1;
        match game.window_bounds.contains(self.position.offset_x(offset_x)) {
            DoesContain    => self.position = self.position.offset_x(offset_x),
            DoesNotContain => {}
        }
        
        let offset_y = std::rand::task_rng().gen_range(0, 3i) - 1;
        match game.window_bounds.contains(self.position.offset_y(offset_y)) {
            DoesContain    => self.position = self.position.offset_y(offset_y),
            DoesNotContain => {}
        }
    }

    fn render(&self, console: &mut Console) {
        console.put_char(self.position.x, self.position.y, self.display_char, background_flag::Set);
    }
}

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
