extern crate dwemthys;
extern crate tcod;

use std::io;
use std::rand::Rng;

use dwemthys::monster::Monster;
use dwemthys::player::Player;
use dwemthys::vm::VM;
use tcod::{Console, background_flag, key_code, Special};

struct Game {
    monsters:      Vec<Monster>,
    player:        Player,
    monster:       Option<Monster>,
    player_weapon: int,
    over:          bool
}

impl Game {
    fn new() -> Game {
        let monsters: Vec<Monster> = vec![
            Monster::dragon(),
            Monster::cyclist(),
            Monster::deer(),
            Monster::ombudsman(),
            Monster::angel(),
            Monster::monkey()
        ];
        let player = Player::rabbit();

        Game { monsters: monsters, player: player, monster: None, player_weapon: -1, over: false }
    }

    fn introduce_foe(&mut self) {
        let should_get_new = match self.monster {
            Some(ref m) => m.life < 1,
            None    => true
        };
        if should_get_new {
            let opt : Option<Monster> = self.monsters.pop();
            self.monster = opt.clone();
            let name = match opt {
                Some(monster) => monster.name,
                None          => { self.over = true; return }
            };

            println!("[Get ready. {:s} has emerged.]", name)
        }
    }

    fn process_input(&mut self) {
        self.player_weapon = -1;
        println!("How would you like to attack? Boomerang, Hero's Sword, Lettuce or Bombs?");
        println!("^ for Boomerang, / for Sword, % for Lettuce, * for Bombs or h for an
        explanation.");

        let mut reader = io::stdin();
        let input = reader.read_line().ok().expect("Failed to read line.");
        let input_char: Option<String> = from_str(input.as_slice().trim());

        let command = match input_char {
            Some(string) => string,
            None         => fail!("fuck")
        };
        let sword_dmg = match self.monster.clone() {
            Some(monster) => (4 + ((monster.life % 10) ^ 2)),
            None          => fail!("Game over!")
        };

        if command == "*".to_string() {
            if self.player.bombs > 0 {
                self.player_weapon = 86;
                self.player.bombs -= 1;
            } else {
                println!("You're out of bombs!");
            }
        } else if command == "/".to_string() {
            self.player_weapon = sword_dmg;
        } else if command == "%".to_string() {
            self.player.lettuce();
            self.player_weapon = 0;
        } else if command == "^".to_string() {
            self.player_weapon = 13;
        } else if command == "h".to_string() {
            println!("Your weapons:
^ (Boomerang) has strength 13
/ (Hero's Sword) has variable strength based on your opponent ({:d})
% (Lettuce) Lettuce will build your strength and extra ruffage will fly in the face of your opponent!!
* (Bomb) You only have {:d} left!", 1i, self.player.bombs);
        } else {
            println!("That didn't make any sense!");
        }
    }

    fn display_intro(&self) {
        println!("A scalding SEETHING LAVA infiltrates the cacauphonous ENGORGED MINESHAFTS deep
within the ageless canopy of the DWEMTHY FOREST... chalky and nocturnal screams from the
belly of the RAVENOUS WILD STORKUPINE... who eats wet goslings RIGHT AFTER they've had a
few graham crackers and a midday nap... amidst starved hippos TECHNICALLY ORPHANED but
truthfully sheltered by umbrellas owned jointly by car dealership conglomerates... beneath
uncapped vials of mildly pulpy BLUE ELIXIR... which shall remain... heretofore...
UNDISTURBED... DWEMTHY!!!");

        println!("You have six foes.");

        println!("These are the living, breathing monstrosities of Dwemthy's Array. I don't know
how they got there. No one knows. Actually, I'm guessing the IntrepidDecomposedCyclist rode
his ten-speed. But the others: NO ONE knows.

If it's really important for you to know, let's just say the others were born there. Can we
move on??

As Dwemthy's Array gets deeper, the challenge becomes more difficult.");

        println!("Fight the Array and the monsters will appear as you go. Godspeed and may you
return with harrowing tales and nary an angel talon piercing through your shoulder.

Oh, and none of this \"I'm too young to die\" business. I'm sick of that crap. I'm not going
to have you insulting our undead young people. They are our future. After our future is
over, that is.");
        
        println!("\nPrepare for battle!\n\n=================\n\n")
    }

    fn update_game(&mut self) {
        let weapon = self.player_weapon;
        {
            let monster = match self.monster {
                Some(ref mut monster) => monster,
                None                  => { self.over = true; return }
            };
            self.player.fight(monster, weapon);
        }
        if self.player.life < 1i {
            self.revive_rabbit();
        }
    }

    fn revive_rabbit(&mut self) {
        println!("You have fallen in battle, but another foolish young rabbit rushes to take your
        place!");
        self.player = Player::rabbit();
    }

    fn render(&self) {
        println!("You have {:d} health!!", self.player.life);
    }
}

fn main() {
    let mut con = Console::init_root(80, 50, "libtcod Rust tutorial", false);
    let mut exit = false;
    while !(Console::window_closed() || exit) {
        con.clear();
        con.put_char(40, 25, '@', background_flag::Set);
        con.flush();
        let keypress = con.wait_for_keypress(true);
        match keypress.key {
            Special(key_code::Escape) => exit = true,
            _ => {}
        }
    }
    //let n : Option<u8> = from_str("05");
    //let num : u8 = match n {
        //Some(nu) => nu,
        //None     => fail!("couldn't parse")
    //};
    //let array = [0x05, 0x10, 0x00, 0x05, 10u8, 0x01, num, 25u8];
    //let mut vm = VM::new();
    //vm.interpret(array);
    //let mut game = Game::new();
    //game.display_intro();
    //game.introduce_foe();

    //loop {
        //&mut game.process_input();
        //if game.player_weapon != -1 {
            //&mut game.update_game();
            //&mut game.introduce_foe();
            //game.render();
        //}
        //if game.over { break; }
    //}
}
