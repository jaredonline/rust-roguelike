use std::cell::RefCell;
use std::rc::Rc;

use util::Point;
use game::MoveInfo;
use rendering::renderers::{Color};
use rendering::windows::Windows;
use movement::{AggroMovementComponent, RandomMovementComponent, UserMovementComponent, MovementComponent};

pub struct Actor<'a> {
    pub position:     Point,
    pub display_char: char,
    pub movement_component: Box<MovementComponent + 'a>,
    pub is_pc: bool,
    pub foreground: Color,
    pub background: Color,
    pub health: u8
}

impl<'a> Actor<'a> {
    pub fn new(x: i32, y: i32, dc: char, mc: Box<MovementComponent + 'a>, is_pc: bool, foreground: Color, background: Color, health: u8) -> Actor<'a> {
        Actor {
            position: Point { x: x, y: y },
            display_char: dc,
            movement_component: mc,
            is_pc: is_pc,
            foreground: foreground,
            background: background,
            health: health
        }
    }

    pub fn clone(&'a self) -> Actor<'a> {
        let mc = self.movement_component.box_clone();
        Actor::new(self.position.x, self.position.y, self.display_char, mc, self.is_pc, self.foreground, self.background, self.health)
    }

    pub fn dog(x: i32, y: i32, move_info: Rc<RefCell<MoveInfo>>) -> Actor<'a> {
        let mc : Box<RandomMovementComponent> = box MovementComponent::new(move_info);
        Actor::new(x, y, 'd', mc, false, Color::White, Color::Black, 20u8)
    }

    pub fn cat(x: i32, y: i32, move_info: Rc<RefCell<MoveInfo>>) -> Actor<'a> {
        let mc : Box<RandomMovementComponent> = box MovementComponent::new(move_info);
        Actor::new(x, y, 'c', mc, false, Color::White, Color::Black, 20u8)
    }

    pub fn heroine(move_info: Rc<RefCell<MoveInfo>>) -> Actor<'a> {
        let point = {
            move_info.borrow().deref().char_location
        };
        let mc : Box<UserMovementComponent> = box MovementComponent::new(move_info);
        Actor::new(point.x, point.y, '@', mc, true, Color::Blue, Color::Black, 20u8)
    }

    pub fn kobold(x: i32, y: i32, move_info: Rc<RefCell<MoveInfo>>) -> Actor<'a> {
        let mc : Box<AggroMovementComponent> = box MovementComponent::new(move_info);
        Actor::new(x, y, 'k', mc, false, Color::Red, Color::Black, 20u8)
    }

    pub fn update(&mut self, windows: &mut Windows) {
        self.position = self.movement_component.update(self.position, windows);
    }
}
