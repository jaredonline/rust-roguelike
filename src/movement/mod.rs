extern crate tcod;

use util::{
    Point,
    DoesContain,
    DoesNotContain,
    LeftOfPoint,
    RightOfPoint,
    OnPointX,
    AbovePoint,
    BelowPoint,
    OnPointY,
    PointsEqual,
    PointsNotEqual
};
use game::Game;

use std;
use std::rand::Rng;

use self::tcod::{Special, key_code};

pub trait MovementComponent {
    fn new() -> Self;
    fn handle_input(&self, Point) -> Point;
}

pub struct TcodUserMovementComponent;
pub struct RandomMovementComponent;
pub struct AggroMovementComponent;

impl MovementComponent for TcodUserMovementComponent {
    fn new() -> TcodUserMovementComponent {
        TcodUserMovementComponent
    }

    fn handle_input(&self, point: Point) -> Point {
        let mut offset = Point { x: point.x, y:  point.y};
        match Game::last_keypress() {
            Some(keypress) => match keypress.key {
                Special(key_code::Up) => {
                    offset = offset.offset_y(-1);
                },
                Special(key_code::Down) => {
                    offset = offset.offset_y(1);
                },
                Special(key_code::Left) => {
                    offset = offset.offset_x(-1);
                },
                Special(key_code::Right) => {
                    offset = offset.offset_x(1);
                },
                _ => {}
            },
            None => {}
        }

        match Game::bounds_contain(offset) {
            DoesContain    => return offset,
            DoesNotContain => return point
        }
    }
}

impl MovementComponent for RandomMovementComponent {
    fn new() -> RandomMovementComponent {
        RandomMovementComponent
    }

    fn handle_input(&self, point: Point) -> Point {
        let mut offset = Point { x: point.x, y: point.y };
        let offset_x = std::rand::task_rng().gen_range(0, 3i32) - 1;
        match Game::bounds_contain(point.offset_x(offset_x)) {
            DoesContain    => offset.x = point.offset_x(offset_x).x,
            DoesNotContain => {}
        }
        
        let offset_y = std::rand::task_rng().gen_range(0, 3i32) - 1;
        match Game::bounds_contain(point.offset_y(offset_y)) {
            DoesContain    => offset.y = point.offset_y(offset_y).y,
            DoesNotContain => {}
        }

        return offset;
    }
}

impl MovementComponent for AggroMovementComponent {
    fn new() -> AggroMovementComponent {
        AggroMovementComponent
    }

    fn handle_input(&self, point: Point) -> Point {
        let char_point = Game::get_char_point();
        let mut offset = Point { x: 0, y: 0 };
        match point.compare_x(char_point) {
            RightOfPoint => offset = offset.offset_x(-1),
            LeftOfPoint  => offset = offset.offset_x(1),
            OnPointX     => {}
        }
        
        match point.compare_y(char_point) {
            BelowPoint => offset = offset.offset_y(-1),
            AbovePoint => offset = offset.offset_y(1),
            OnPointY   => {}
        }

        match point.offset(offset).compare(char_point) {
            PointsEqual    => { return point; },
            PointsNotEqual => {
                match Game::bounds_contain(point.offset(offset)) {
                    DoesContain    => { return point.offset(offset); }
                    DoesNotContain => { return point; }
                }
            }
        }
    }
}
