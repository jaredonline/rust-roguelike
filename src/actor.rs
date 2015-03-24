use std::sync::{Arc, RwLock, Mutex};

use movement::{MovementComponent, AggroMovementComponent, RandomMovementComponent, UserMovementComponent};
use game::{GameInfo};
use util::Point;

pub struct Actor {
    pub position:     Point,
    pub display_char: char,
    pub movement:     Box<MovementComponent + 'static>
}
pub type SafeActor = Arc<Mutex<Actor>>;
pub type Actors    = Vec<SafeActor>;

unsafe impl Send for Actor {}

impl Actor {
    pub fn new(x: i32, y: i32, c: char, mc: Box<MovementComponent>) -> Actor {
        Actor {
            position: Point {
                x: x,
                y: y
            },
            display_char: c,
            movement: mc
        }
    }

    pub fn mutex(x: i32, y: i32, c: char, mc: Box<MovementComponent>) -> SafeActor {
        Arc::new(Mutex::new(Actor::new(x, y, c, mc)))
    }

    pub fn heroine() -> SafeActor {
        let mc = Box::new(UserMovementComponent::new());
        Actor::mutex(40, 25, '@', mc)
    }

    pub fn dog() -> SafeActor {
        let mc = Box::new(RandomMovementComponent::new());
        Actor::mutex(10, 10, 'd', mc)
    }

    pub fn cat() -> SafeActor {
        let mc = Box::new(RandomMovementComponent::new());
        Actor::mutex(40, 25, 'c', mc)
    }

    pub fn kobold() -> SafeActor {
        let mc = Box::new(AggroMovementComponent::new());
        Actor::mutex(20, 20, 'k', mc)
    }

    pub fn update(&mut self, game: Arc<RwLock<GameInfo>>) {
        let point = self.movement.update(&self.position, game);
        self.position = point;
    }
}
