extern crate tcod;

use self::tcod::{Console, BackgroundFlag};

use std::sync::{Arc, RwLock, Mutex};

use movement::{MovementComponent, RandomMovementComponent, UserMovementComponent};
use game::Game;
use util::Point;

pub struct Actor {
    pub position:     Point,
    pub display_char: char,
    pub movement:     Box<MovementComponent + 'static>
}

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

    pub fn mutex(x: i32, y: i32, c: char, mc: Box<MovementComponent>) -> Arc<Mutex<Actor>> {
        Arc::new(Mutex::new(Actor::new(x, y, c, mc)))
    }

    pub fn heroine() -> Arc<Mutex<Actor>> {
        let mc = Box::new(UserMovementComponent::new());
        Actor::mutex(40, 25, '@', mc)
    }

    pub fn dog() -> Arc<Mutex<Actor>> {
        let mc = Box::new(RandomMovementComponent::new());
        Actor::mutex(10, 10, 'd', mc)
    }

    pub fn render(&self, console: &mut Console) {
        console.put_char(self.position.x, self.position.y, self.display_char, BackgroundFlag::Set);
    }

    pub fn update(&mut self, game: Arc<RwLock<Game>>) {
        let point = self.movement.update(&self.position, game);
        self.position = point;
    }
}
