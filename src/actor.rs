use std::sync::{Arc, Mutex};

use movement::{MovementComponent, AggroMovementComponent, RandomMovementComponent, UserMovementComponent};
use game::SafeGameInfo;
use util::Point;
use std::fmt::{Formatter, Display, Result};

static mut ACTOR_TICKET : i32 = 0;

pub struct Actor {
    pub position:     Point,
    pub display_char: char,
    pub movement:     Box<MovementComponent + 'static>,
    pub id:           i32
}
pub type SafeActor = Arc<Mutex<Actor>>;
pub type Actors    = Vec<SafeActor>;

unsafe impl Send for Actor {}

impl Display for Actor {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        write!(fmt, "{}", self.id)
    }
}

impl Actor {
    pub fn new(x: i32, y: i32, c: char, mc: Box<MovementComponent>) -> Actor {
        let ticket = unsafe {
            ACTOR_TICKET += 1;
            ACTOR_TICKET
        };
        Actor {
            position: Point {
                x: x,
                y: y
            },
            display_char: c,
            movement: mc,
            id: ticket
        }
    }

    pub fn mutex(x: i32, y: i32, c: char, mc: Box<MovementComponent>) -> SafeActor {
        Arc::new(Mutex::new(Actor::new(x, y, c, mc)))
    }

    pub fn heroine() -> SafeActor {
        let mc = Box::new(UserMovementComponent::new());
        Actor::mutex(40, 25, 1 as char, mc)
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

    pub fn update(&mut self, game: SafeGameInfo) {
        let point = self.movement.update(&self.position, game);
        self.position = point;
    }
}
