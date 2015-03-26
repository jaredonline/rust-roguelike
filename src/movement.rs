extern crate rand;

use self::rand::{Rng, thread_rng};
use std::sync::{Arc, RwLock};

use game::{SafeGameInfo, GameInfo};
use rendering::{print_action, Window};
use util::{Point, Contains, XRelation, YRelation, PointEquality};
use input::{KeyCode};
use input::Key::{SpecialKey, Printable};

pub trait MovementComponent {
    fn update(&self, &Point, Arc<RwLock<GameInfo>>) -> Point;
}

pub struct UserMovementComponent;

impl UserMovementComponent {
    pub fn new() -> UserMovementComponent { UserMovementComponent }
}

impl MovementComponent for UserMovementComponent {
    fn update(&self, start: &Point, game: SafeGameInfo) -> Point {
        let mut offset = Point { x: 0, y: 0 };
        let game = game.read().unwrap();
        match game.keypress.key {
            Printable('w') |
            SpecialKey(KeyCode::Up) => {
                offset = offset.offset_y(-1);
            },
            Printable('s') | 
            SpecialKey(KeyCode::Down) => {
                offset = offset.offset_y(1);
            },
            Printable('a') |
            SpecialKey(KeyCode::Left) => {
                offset = offset.offset_x(-1);
            },
            Printable('d') |
            SpecialKey(KeyCode::Right) => {
                offset = offset.offset_x(1);
            },
            _ => {
            }
        }

        match game.window_bounds.contains(&start.offset(&offset)) {
            Contains::DoesContain    => start.offset(&offset),
            Contains::DoesNotContain => {
                let _ = game.sender.send(print_action("You can't move that way!", Window::Messages));
                Point { x: start.x, y: start.y }
            }
        }
    }
}

pub struct RandomMovementComponent;

impl RandomMovementComponent {
    pub fn new() -> RandomMovementComponent { RandomMovementComponent }
}

impl MovementComponent for RandomMovementComponent {
    fn update(&self, start: &Point, game: Arc<RwLock<GameInfo>>) -> Point {
        let mut new_point = Point { x: start.x, y: start.y };
        let game = game.read().unwrap();
        let offset_x = thread_rng().gen_range(0, 3i32) - 1;
        match game.window_bounds.contains(&new_point.offset_x(offset_x)) {
            Contains::DoesContain    => new_point = new_point.offset_x(offset_x),
            Contains::DoesNotContain => {}
        }

        let offset_y = thread_rng().gen_range(0, 3i32) - 1;
        match game.window_bounds.contains(&new_point.offset_y(offset_y)) {
            Contains::DoesContain    => new_point = new_point.offset_y(offset_y),
            Contains::DoesNotContain => {}
        }

        new_point
    }
}

pub struct AggroMovementComponent;

impl AggroMovementComponent {
    pub fn new() -> AggroMovementComponent { AggroMovementComponent }
}

impl MovementComponent for AggroMovementComponent {
    fn update(&self, start: &Point, game: SafeGameInfo) -> Point {
        let game = game.read().unwrap();
        let mut offset = Point { x: 0, y: 0 };
        
        match start.compare_x(&game.char_position) {
            XRelation::Left  => offset = offset.offset_x(1),
            XRelation::Right => offset = offset.offset_x(-1),
            _ => {}
        }

        match start.compare_y(&game.char_position) {
            YRelation::Above => offset = offset.offset_y(1),
            YRelation::Below => offset = offset.offset_y(-1),
            _ => {}
        }

        match start.offset(&offset).compare(&game.char_position) {
            PointEquality::Equal    => { return start.clone(); }
            PointEquality::NotEqual => {
                match game.window_bounds.contains(&start.offset(&offset)) {
                    Contains::DoesContain    => { return start.offset(&offset) }
                    Contains::DoesNotContain => { return start.clone(); }
                }
            }
        }
    }
}
