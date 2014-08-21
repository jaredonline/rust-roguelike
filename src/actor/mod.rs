use util::Point;
use rendering::RenderingComponent;
use movement::MovementComponent;

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

    pub fn update(&mut self) {
        let offset    = self.movement_component.handle_input(self.position);
        self.position = offset;
    }

    pub fn render(&self, rendering_component: &mut RenderingComponent) {
        rendering_component.render_object(self.position, self.display_char);
    }
}

