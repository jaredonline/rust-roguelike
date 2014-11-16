use std::num;
use std::rand;
use std::rand::distributions::{IndependentSample, Range};

pub trait Weapon {
    fn new() -> Self;
    fn get_name(&self) -> String;
    fn deal_damage(&self) -> u16;
}

pub struct Boomerang {
    name: String,
    base_damage: u8
}

impl Weapon for Boomerang {
    fn new() -> Boomerang {
        Boomerang {
            name: String::from_str("Little Boomerang"),
            base_damage: 13
        }
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn deal_damage(&self) -> u16 {
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

impl Weapon for Sword {
    fn new() -> Sword {
        Sword {
            name: String::from_str("Heroic Sworc"),
            base_damage: 4
        }
    }

    fn get_name(&self) -> String { self.name.clone() }
    
    fn deal_damage(&self) -> u16 {
        let max = self.base_damage + num::pow(56u8 % 10u8, 2);
        let mut rng = rand::task_rng();
        Range::new(0u8, max).ind_sample(&mut rng) as u16
    }
}

pub struct Lettuce {
    name: String,
    base_damage: u8
}
