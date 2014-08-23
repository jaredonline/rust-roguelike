use util::Point;
use rendering::RenderingComponent;
use movement::{AggroMovementComponent, RandomMovementComponent, MovementComponent};

pub struct Actor {
    pub position:           Point,
    pub display_char:       char,
    pub movement_component: Box<MovementComponent>
}

impl Actor {
    pub fn new(x: i32, y: i32, dc: char, mc: Box<MovementComponent>) -> Actor {
        Actor {
            position:           Point { x: x, y: y },
            display_char:       dc,
            movement_component: mc
        }
    }

    pub fn dog() -> Actor {
        let mc : Box<RandomMovementComponent> = box MovementComponent::new();
        Actor::new(10, 10, 'd', mc)
    }
    pub fn cat() -> Actor {
        let mc : Box<RandomMovementComponent> = box MovementComponent::new();
        Actor::new(40, 25, 'c', mc)
    }

    pub fn kobold() -> Actor {
        let mc : Box<AggroMovementComponent> = box MovementComponent::new();
        Actor::new(0, 0, 'k', mc)
    }    

    pub fn update(&mut self) {
        let offset    = self.movement_component.handle_input(self.position);
        self.position = offset;
    }

    pub fn render(&self, rendering_component: &mut RenderingComponent) {
        rendering_component.render_object(self.position, self.display_char);
    }
}

