use std::cell::RefCell;
use std::rc::Rc;

use input::Key::{SpecialKey,};
use input::{KeyCode};
use game::MoveInfo;
use util::{
    Point,
};
use util::Contains::{
    DoesContain,
    DoesNotContain,
};
use util::XPointRelation::{
    LeftOfPoint,
    RightOfPoint,
    OnPointX,
};
use util::YPointRelation::{
    AbovePoint,
    BelowPoint,
    OnPointY,
};
use util::PointEquality::{
    PointsEqual,
    PointsNotEqual
};
use rendering::windows::Windows;

use std;
use std::rand::Rng;

pub trait MovementComponent {
    fn new(Rc<RefCell<MoveInfo>>) -> Self;
    fn update(&self, Point, &mut Windows) -> Point;
    fn box_clone(&self) -> Box<MovementComponent + 'static>;
}

pub struct AggroMovementComponent {
    move_info: Rc<RefCell<MoveInfo>>
}

impl MovementComponent for AggroMovementComponent {
    fn new(move_info: Rc<RefCell<MoveInfo>>) -> AggroMovementComponent {
        AggroMovementComponent { move_info: move_info }
    }

    fn box_clone(&self) -> Box<MovementComponent + 'static> {
        box AggroMovementComponent { move_info: self.move_info.clone() }
    }

    fn update(&self, point: Point, _: &mut Windows) -> Point {
        let char_point = {
            self.move_info.borrow().deref().char_location
        };
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
                let bound = {
                    self.move_info.borrow().deref().bounds
                };
                match bound.contains(point.offset(offset)) {
                    DoesContain    => { return point.offset(offset); }
                    DoesNotContain => { return point; }
                }
            }
        }
    }
}

pub struct UserMovementComponent {
    move_info: Rc<RefCell<MoveInfo>>
}

impl MovementComponent for UserMovementComponent {
    fn new(move_info: Rc<RefCell<MoveInfo>>) -> UserMovementComponent {
        UserMovementComponent { move_info: move_info }
    }

    fn box_clone(&self) -> Box<MovementComponent + 'static> {
        box UserMovementComponent { move_info: self.move_info.clone() }
    }

    fn update(&self, point: Point, windows: &mut Windows) -> Point {
        let mut offset = point.clone();
        let last_keypress = {
            self.move_info.borrow().deref().last_keypress
        };
        offset = match last_keypress {
            Some(keypress) => {
                match keypress.key {
                    SpecialKey(KeyCode::Up) => {
                        offset.offset_y(-1)
                    },
                    SpecialKey(KeyCode::Down) => {
                        offset.offset_y(1)
                    },
                    SpecialKey(KeyCode::Left) => {
                        offset.offset_x(-1)
                    },
                    SpecialKey(KeyCode::Right) => {
                        offset.offset_x(1)
                    },
                    _ => { offset }
                }
            },
            None => { offset }
        };

        let bound = {
            self.move_info.borrow().deref().bounds
        };
        match bound.contains(offset) {
            DoesContain    => {
                offset
            }
            DoesNotContain => {
                windows.messages.buffer_message("You can't move that way!");
                point
            }
        }
    }
}

pub struct RandomMovementComponent {
    move_info: Rc<RefCell<MoveInfo>>
}

impl MovementComponent for RandomMovementComponent {
    fn new(move_info: Rc<RefCell<MoveInfo>>) -> RandomMovementComponent {
        RandomMovementComponent { move_info: move_info }
    }

    fn box_clone(&self) -> Box<MovementComponent + 'static> {
        box RandomMovementComponent { move_info: self.move_info.clone() }
    }

    fn update(&self, point: Point, _: &mut Windows) -> Point {
        let mut offset = Point { x: point.x, y: point.y };
        let offset_x = std::rand::task_rng().gen_range(0, 3i32) - 1;
        let bound = {
            self.move_info.borrow().deref().bounds
        };
        match bound.contains(offset.offset_x(offset_x)) {
            DoesContain    => offset = offset.offset_x(offset_x),
            DoesNotContain => { return point; }
        }

        let offset_y = std::rand::task_rng().gen_range(0, 3i32) - 1;
        let bound = {
            self.move_info.borrow().deref().bounds
        };
        match bound.contains(offset.offset_y(offset_y)) {
            DoesContain    => offset = offset.offset_y(offset_y),
            DoesNotContain => { return point; }
        }

        offset
    }
}
