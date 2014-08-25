extern crate tcod;
use self::tcod::{key_code, Special};
use game::Game;
use util::{
    Bound,
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
use std;
use std::rand::Rng;

pub trait MovementComponent {
    fn new(Bound) -> Self;
    fn update(&self, Point) -> Point;
}

pub struct RandomMovementComponent {
    window_bounds: Bound
}

pub struct TcodUserMovementComponent {
    window_bounds: Bound
}

pub struct AggroMovementComponent {
    window_bounds: Bound
}

impl MovementComponent for AggroMovementComponent {
    fn new(bound: Bound) -> AggroMovementComponent {
        AggroMovementComponent { window_bounds: bound }
    }

    fn update(&self, point: Point) -> Point {
        let char_point = Game::get_character_point();
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
                match self.window_bounds.contains(point.offset(offset)) {
                    DoesContain    => { return point.offset(offset); }
                    DoesNotContain => { return point; }
                }
            }
        }
    }
}

impl MovementComponent for TcodUserMovementComponent {
    fn new(bound: Bound) -> TcodUserMovementComponent {
        TcodUserMovementComponent { window_bounds: bound }
    }

    fn update(&self, point: Point) -> Point {
        let mut offset = Point { x: point.x, y: point.y };
        offset = match Game::get_last_keypress() {
            Some(keypress) => {
                match keypress.key {
                    Special(key_code::Up) => {
                        offset.offset_y(-1)
                    },
                    Special(key_code::Down) => {
                        offset.offset_y(1)
                    },
                    Special(key_code::Left) => {
                        offset.offset_x(-1)
                    },
                    Special(key_code::Right) => {
                        offset.offset_x(1)
                    },
                    _ => { offset }
                }
            },
            None => { offset }
        };

        match self.window_bounds.contains(offset) {
            DoesContain    => { offset }
            DoesNotContain => { point }
        }
    }
}

impl MovementComponent for RandomMovementComponent {
    fn new(bound: Bound) -> RandomMovementComponent {
        RandomMovementComponent { window_bounds: bound }
    }

    fn update(&self, point: Point) -> Point {
        let mut offset = Point { x: point.x, y: point.y };
        let offset_x = std::rand::task_rng().gen_range(0, 3i32) - 1;
        match self.window_bounds.contains(offset.offset_x(offset_x)) {
            DoesContain    => offset = offset.offset_x(offset_x),
            DoesNotContain => { return point; }
        }

        let offset_y = std::rand::task_rng().gen_range(0, 3i32) - 1;
        match self.window_bounds.contains(offset.offset_y(offset_y)) {
            DoesContain    => offset = offset.offset_y(offset_y),
            DoesNotContain => { return point; }
        }

        offset
    }
}
