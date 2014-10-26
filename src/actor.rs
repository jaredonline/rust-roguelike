use std::cell::RefCell;
use std::rc::Rc;

use util::Point;
use game::MoveInfo;
use rendering::renderers::RenderingComponent;
use rendering::windows::Windows;
use movement::{AggroMovementComponent, RandomMovementComponent, UserMovementComponent, MovementComponent};

pub struct Actor<'a> {
    pub position:     Point,
    pub display_char: char,
    pub movement_component: Box<MovementComponent + 'a>,
    pub is_pc: bool
}

impl<'a> Clone for Actor<'a> {
    fn clone(&self) -> Actor<'a> {
        let mc = self.movement_component.box_clone();
        Actor::new(self.position.x, self.position.y, self.display_char, mc, self.is_pc)
    }
}

impl<'a> Actor<'a> {
    pub fn new(x: i32, y: i32, dc: char, mc: Box<MovementComponent>, is_pc: bool) -> Actor<'a> {
        Actor { position: Point { x: x, y: y }, display_char: dc, movement_component: mc, is_pc: is_pc }
    }

    pub fn dog(x: i32, y: i32, move_info: Rc<RefCell<MoveInfo>>) -> Actor<'a> {
        let mc : Box<RandomMovementComponent> = box MovementComponent::new(move_info);
        Actor::new(x, y, 'd', mc, false)
    }

    pub fn cat(x: i32, y: i32, move_info: Rc<RefCell<MoveInfo>>) -> Actor<'a> {
        let mc : Box<RandomMovementComponent> = box MovementComponent::new(move_info);
        Actor::new(x, y, 'c', mc, false)
    }

    pub fn heroine(move_info: Rc<RefCell<MoveInfo>>) -> Actor<'a> {
        let point = {
            move_info.borrow().deref().char_location
        };
        let mc : Box<UserMovementComponent> = box MovementComponent::new(move_info);
        Actor::new(point.x, point.y, '@', mc, true)
    }

    pub fn kobold(x: i32, y: i32, move_info: Rc<RefCell<MoveInfo>>) -> Actor<'a> {
        let mc : Box<AggroMovementComponent> = box MovementComponent::new(move_info);
        Actor::new(x, y, 'k', mc, false)
    }

    pub fn update(&mut self, windows: &mut Windows) {
        self.position = self.movement_component.update(self.position, windows);
    }

    pub fn render(&self, rendering_component: &mut Box<RenderingComponent>) {
        rendering_component.render_object(self.position, self.display_char);
    }
}
