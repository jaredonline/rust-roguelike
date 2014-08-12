use std;
use std::rand::Rng;

use monster::Monster;

pub struct Player {
    pub strength: int,
    pub life:     int,
    pub charisma: int,
    pub bombs:    int
}

impl Player {
    pub fn new(s: int, l: int, c: int, b: int) -> Player {
        Player { life: l, strength: s, charisma: c, bombs: b }
    }

    pub fn rabbit() -> Player {
        Player::new(2, 10, 44, 3)
    }

    pub fn fight(&mut self, mon: &mut Monster, weapon: int) {
        if self.life <= 0 {
            println!("[You're too dead to fight!]");
            return;
        }

        let hit = std::rand::task_rng().gen_range(0i, self.strength + weapon);
        println!("[You hit {:s} with {:d} with points of damage!]", mon.name, hit);
        mon.hit(hit);

        if mon.life > 0 {
            let hit = std::rand::task_rng().gen_range(0i, mon.strength + mon.weapon);
            println!("[{:s} hit you with {:d} points of damage!]", mon.name, hit);
            self.hit(hit);
        }
    }

    pub fn lettuce(&mut self) {
        let lettuce = std::rand::task_rng().gen_range(0i, self.charisma);
        println!("[Healthy lettuce gives you {:d} life points!!]", lettuce);
        self.life += lettuce
    }

    pub fn hit(&mut self, dmg: int) {
        let p_up = std::rand::task_rng().gen_range(0i, self.charisma);
        let mut life = self.life;
        if p_up % 9 == 7 {
            life += p_up / 4;
            println!("[Your magick powers up {:d}!]", p_up);
        }
        life -= dmg;
        if life < 0 {
            println!("[You have died.]");
        }
        self.life = life;
    }
}
