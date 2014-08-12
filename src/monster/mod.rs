use std;
use std::rand::Rng;

pub struct Monster {
    pub strength: int,
    pub life:     int,
    pub charisma: int,
    pub weapon:   int,
    pub name:     String
}

impl Monster {
    pub fn new(s: int, l: int, c: int, w: int, n: String) -> Monster {
        Monster { life: l, strength: s, charisma: c, weapon: w, name: n }
    }

    pub fn monkey() -> Monster {
        Monster::new(35, 46, 91, 2, "Industrial Raver Monkey".to_string())
    }

    pub fn angel() -> Monster {
        Monster::new(6, 540, 144, 50, "Dwarven Angel".to_string())
    }

    pub fn ombudsman() -> Monster {
        Monster::new(6, 320, 144, 50, "Assistant Vice Tentacle And Ombudsman".to_string())
    }

    pub fn deer() -> Monster {
        Monster::new(192, 655, 19, 109, "Teeth Deer".to_string())
    }

    pub fn cyclist() -> Monster {
        Monster::new(560, 901, 422, 105, "Intrepid Decomposed Cyclist".to_string())
    }

    pub fn dragon() -> Monster {
        Monster::new(451, 1340, 1020, 939, "Dragon".to_string())
    }

    pub fn hit(&mut self, dmg: int) {
        let p_up = std::rand::task_rng().gen_range(0i, self.charisma);
        let mut life = self.life;
        if p_up % 9 == 7 {
            life += p_up / 4;
            println!("[{:s} magick powers up {:d}!]", self.name, p_up);
        }
        life -= dmg;
        if life < 0 {
            println!("[{:s} has died.]", self.name);
        }
        self.life = life;
    }

}

impl Clone for Monster {
    fn clone(&self) -> Monster {
        Monster { life: self.life, strength: self.strength, charisma: self.charisma, weapon:
        self.weapon, name: self.name.clone() }
    }
}
