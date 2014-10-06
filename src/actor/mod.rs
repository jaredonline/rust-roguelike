use util::{Point, Bound};
use game::{Game, Windows};
use rendering::RenderingComponent;
use movement::{AggroMovementComponent, RandomMovementComponent, UserMovementComponent, MovementComponent};

pub struct Actor<'a> {
    pub position:     Point,
    pub display_char: char,
    pub movement_component: Box<MovementComponent + 'a>
}

impl<'a> Actor<'a> {
    pub fn new(x: i32, y: i32, dc: char, mc: Box<MovementComponent>) -> Actor<'a> {
        Actor { position: Point { x: x, y: y }, display_char: dc, movement_component: mc }
    }

    pub fn dog(x: i32, y: i32, bound: Bound) -> Actor<'a> {
        let mc : Box<RandomMovementComponent> = box MovementComponent::new(bound);
        Actor::new(x, y, 'd', mc)
    }

    pub fn cat(x: i32, y: i32, bound: Bound) -> Actor<'a> {
        let mc : Box<RandomMovementComponent> = box MovementComponent::new(bound);
        Actor::new(x, y, 'c', mc)
    }

    pub fn heroine(bound: Bound) -> Actor<'a> {
        let point = Game::get_character_point();
        let mc : Box<UserMovementComponent> = box MovementComponent::new(bound);
        Actor::new(point.x, point.y, '@', mc)
    }

    pub fn kobold(x: i32, y: i32, bound: Bound) -> Actor<'a> {
        let mc : Box<AggroMovementComponent> = box MovementComponent::new(bound);
        Actor::new(x, y, 'k', mc)
    }

    pub fn update(&mut self, windows: &mut Windows) {
        self.position = self.movement_component.update(self.position, windows);
    }

    pub fn render(&self, rendering_component: &mut Box<RenderingComponent>) {
        rendering_component.render_object(self.position, self.display_char);
    }
}
