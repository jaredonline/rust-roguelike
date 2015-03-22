extern crate rand;
extern crate core;

use std::num;
use self::rand::distributions::{IndependentSample, Range};

use actor::Actor;
use self::core::ops::Deref;

pub trait Weapon {
    fn get_name(&self) -> String;
    fn deal_damage(&self, &Box<Actor>) -> u16;
}

pub struct Boomerang {
    name: String,
    base_damage: u8
}

impl Boomerang {
    pub fn new() -> Boomerang {
        Boomerang {
            name: String::from_str("Little Boomerang"),
            base_damage: 13
        }
    }
}

impl Weapon for Boomerang {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn deal_damage(&self, _: &Box<Actor>) -> u16 {
        self.base_damage as u16
    }
}

pub struct Bomb {
    name: String,
    base_damage: u8
}

pub struct Sword {
    name: String,
    base_damage: u8
}

impl Sword {
    pub fn new() -> Sword {
        Sword {
            name: String::from_str("Heroic Sworc"),
            base_damage: 4
        }
    }
}

impl Weapon for Sword {
    fn get_name(&self) -> String { self.name.clone() }
    
    fn deal_damage(&self, enemy: &Box<Actor>) -> u16 {
        let x   = enemy.health % 10u8;
        let max = self.base_damage + (x * x);
        let mut rng = rand::thread_rng();
        Range::new(0u8, max).ind_sample(&mut rng) as u16
    }
}

pub struct Lettuce {
    name: String,
    base_damage: u8
}
