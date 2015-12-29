extern crate tcod;
use std::cmp::{min, max};

use std::fmt::{Display, Formatter, Result};

use tcod::{Console, RootConsole, BackgroundFlag};
use tcod::console::Offscreen;
use tcod::colors::{self, Color};
use tcod::input::Key;
use tcod::input::KeyCode::{Up, Down, Left, Right, Escape};
use tcod::random::Rng;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const MAP_WIDTH: usize = 80;
const MAP_HEIGHT: usize = 45;

const COLOR_DARK_WALL: Color = Color {
    r: 0,
    g: 0,
    b: 100,
};
const COLOR_DARK_GROUND: Color = Color {
    r: 50,
    g: 50,
    b: 150,
};

const ROOM_MAX_SIZE: i32 = 10;
const ROOM_MIN_SIZE: i32 = 6;
const MAX_ROOMS: i32 = 30;

type Map = Vec<Vec<Tile>>;

struct Rect {
    x: i32,
    x2: i32,
    y: i32,
    y2: i32,
}

impl Display for Rect {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f,
               "Rect<x1: {}, x2: {}, y1: {}, y2: {}>",
               self.x,
               self.x2,
               self.y,
               self.y2)
    }
}

impl Rect {
    fn new(x: i32, y: i32, w: i32, h: i32) -> Rect {
        Rect {
            x: x,
            y: y,
            x2: x + w,
            y2: y + h,
        }
    }

    fn center(&self) -> (i32, i32) {
        let center_x = (self.x + self.x2) / 2;
        let center_y = (self.y + self.y2) / 2;
        (center_x, center_y)
    }

    fn intersect(&self, other: &Rect) -> bool {
        self.x <= other.x2 && self.x2 >= other.x && self.y <= other.y2 && self.y2 >= other.y
    }
}

struct Object {
    x: i32,
    y: i32,
    char: char,
    color: Color,
}

impl Object {
    fn new(x: i32, y: i32, char: char, color: Color) -> Object {
        Object {
            x: x,
            y: y,
            char: char,
            color: color,
        }
    }

    fn move_by(&mut self, dx: i32, dy: i32, map: &Map) {
        if map[(self.x + dx) as usize][(self.y + dy) as usize].blocked != true {
            self.x = self.x + dx;
            self.y = self.y + dy;
        }
    }

    fn draw(&self, con: &mut Console) {
        con.set_default_foreground(self.color);
        con.put_char(self.x, self.y, self.char, BackgroundFlag::None);
    }

    fn clear(&self, con: &mut Console) {
        con.put_char(self.x, self.y, ' ', BackgroundFlag::None);
    }
}

#[derive(Copy, Clone)]
struct Tile {
    blocked: bool,
    block_site: bool,
}

impl Tile {
    fn new(blocked: bool, block_site: bool) -> Tile {
        Tile {
            blocked: blocked,
            block_site: block_site,
        }
    }
}

fn main() {
    let mut root = RootConsole::initializer()
                       .size(SCREEN_WIDTH, SCREEN_HEIGHT)
                       .title("libtcod Rust tutorial")
                       .init();

    let mut con = Offscreen::new(SCREEN_WIDTH, SCREEN_HEIGHT);

    let mut player = Object::new(25, 23, '@', colors::WHITE);
    let npc = Object::new(SCREEN_WIDTH / 2 - 5,
                          SCREEN_HEIGHT / 2 - 5,
                          '@',
                          colors::YELLOW);

    let map = make_map(&mut player);
    let mut objects = [player, npc];


    let mut exit = false;
    while !(root.window_closed() || exit) {
        render_all(&objects, &mut root, &mut con, &map);
        root.flush();

        for object in &objects {
            object.clear(&mut con);
        }
        exit = handle_keys(&mut root, &mut objects[0], &map);
    }
}

fn handle_keys(console: &mut RootConsole, player: &mut Object, map: &Map) -> bool {
    let keypress = console.wait_for_keypress(true);
    let mut ret = false;

    match keypress {
        Key { code: Up, .. } => player.move_by(0, -1, map),
        Key { code: Down, .. } => player.move_by(0, 1, map),
        Key { code: Left, .. } => player.move_by(-1, 0, map),
        Key { code: Right, .. } => player.move_by(1, 0, map),
        Key { code: Escape, .. } => {
            ret = true;
        }
        _ => {}
    };

    return ret;
}

fn make_map(player: &mut Object) -> Map {
    let mut map = vec![];

    for _ in 0..MAP_WIDTH {
        let col = vec![Tile::new(true, true); MAP_HEIGHT];
        map.push(col);
    }

    let rng = Rng::get_instance();
    let mut rooms = vec![];
    let mut num_rooms = 0;

    for _ in 0..MAX_ROOMS {
        let w = rng.get_int(ROOM_MIN_SIZE, ROOM_MAX_SIZE);
        let h = rng.get_int(ROOM_MIN_SIZE, ROOM_MAX_SIZE);
        let x = rng.get_int(0, (MAP_WIDTH - w as usize - 1) as i32);
        let y = rng.get_int(0, (MAP_HEIGHT - h as usize - 1) as i32);

        let rect = Rect::new(x, y, w, h);
        let mut failed = false;
        for other in &rooms {
            if rect.intersect(other) {
                failed = true;
                break;
            }
        }

        if failed != true {
            create_room(&rect, &mut map);

            let (new_x, new_y) = rect.center();

            if num_rooms == 0 {
                player.x = new_x;
                player.y = new_y;
            } else {
                let (prev_x, prev_y) = rooms[num_rooms - 1].center();

                if rng.get_int(0, 1) == 1 {
                    create_h_tunnel(prev_x, new_x, prev_y, &mut map);
                    create_v_tunnel(prev_y, new_y, new_x, &mut map);
                } else {
                    create_v_tunnel(prev_y, new_y, prev_x, &mut map);
                    create_h_tunnel(prev_x, new_x, new_y, &mut map);
                }
            }

            rooms.push(rect);
            num_rooms = num_rooms + 1;
        }
    }

    map
}

fn render_all(objects: &[Object], root: &mut RootConsole, con: &mut Offscreen, map: &Map) {
    for object in objects {
        object.draw(con);
    }

    for x in 0..MAP_WIDTH {
        for y in 0..MAP_HEIGHT {
            let wall = map[x][y].block_site;
            let (x, y) = (x as i32, y as i32);

            if wall {
                con.set_char_background(x, y, COLOR_DARK_WALL, BackgroundFlag::Set);
            } else {
                con.set_char_background(x, y, COLOR_DARK_GROUND, BackgroundFlag::Set);
            }
        }
    }

    tcod::console::blit(con,
                        (0, 0),
                        (SCREEN_WIDTH, SCREEN_HEIGHT),
                        root,
                        (0, 0),
                        1.0,
                        1.0);
}

fn create_room(room: &Rect, map: &mut Map) {
    for x in room.x + 1..room.x2 {
        for y in room.y + 1..room.y2 {
            let (x, y) = (x as usize, y as usize);
            map[x][y].blocked = false;
            map[x][y].block_site = false;
        }
    }
}

fn create_h_tunnel(x1: i32, x2: i32, y: i32, map: &mut Map) {
    for x in min(x1, x2)..max(x1, x2) + 1 {
        let (x, y) = (x as usize, y as usize);
        map[x][y].blocked = false;
        map[x][y].block_site = false;
    }
}

fn create_v_tunnel(y1: i32, y2: i32, x: i32, map: &mut Map) {
    for y in min(y1, y2)..max(y1, y2) + 1 {
        let (x, y) = (x as usize, y as usize);
        map[x][y].blocked = false;
        map[x][y].block_site = false;
    }
}
