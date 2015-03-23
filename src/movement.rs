extern crate rand;
extern crate tcod;

use self::rand::{Rng, thread_rng};
use self::tcod::KeyCode;
use self::tcod::Key::Special;

use std::sync::{Arc, RwLock};

use game::Game;
use util::{Point, Contains};

pub trait MovementComponent {
    fn update(&self, &Point, Arc<RwLock<Game>>) -> Point;
}

pub struct UserMovementComponent;

impl UserMovementComponent {
    pub fn new() -> UserMovementComponent { UserMovementComponent }
}

impl MovementComponent for UserMovementComponent {
    fn update(&self, start: &Point, game: Arc<RwLock<Game>>) -> Point {
        let mut offset = Point { x: 0, y: 0 };
        let game = game.read().unwrap();
        match game.keypress.unwrap().key {
            Special(KeyCode::Up) => {
                offset = offset.offset_y(-1);
            },
            Special(KeyCode::Down) => {
                offset = offset.offset_y(1);
            },
            Special(KeyCode::Left) => {
                offset = offset.offset_x(-1);
            },
            Special(KeyCode::Right) => {
                offset = offset.offset_x(1);
            },
            _ => {}
        }

        match game.window_bounds.contains(&start.offset(&offset)) {
            Contains::DoesContain    => start.offset(&offset),
            Contains::DoesNotContain => Point { x: start.x, y: start.y }
        }
    }
}

pub struct RandomMovementComponent;

impl RandomMovementComponent {
    pub fn new() -> RandomMovementComponent { RandomMovementComponent }
}

impl MovementComponent for RandomMovementComponent {
    fn update(&self, start: &Point, game: Arc<RwLock<Game>>) -> Point {
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

