use std::thread;

use actor::{Actors, SafeActor};
use game::SafeGameInfo;
use rendering::{print_action, RenderAction, Window};
use input::Key::{SpecialKey, Printable};
use input::KeyCode;

pub trait GameState {
    fn update(&mut self, pcs: &Actors, npcs: &Actors, SafeGameInfo);

    fn should_exit(&self) -> bool;
    fn exit(&self, SafeGameInfo) {}
    fn enter(&self, SafeGameInfo) {}
}

pub struct MovementGameState;

impl MovementGameState {
    pub fn new() -> MovementGameState { MovementGameState }

    fn update_game_info(&self, a: &SafeActor, game_info: &SafeGameInfo) {
        let actor = a.clone();
        let game  = game_info.clone();

        game.write().unwrap().char_position = actor.lock().unwrap().position.clone();
    }

    fn update_actors(&self, actors: &Actors, game_info: &SafeGameInfo) {
        let _ : Vec<_> = actors.iter().map(|a| {
            let actor = a.clone();
            let game  = game_info.clone();

            thread::spawn(move || {
                actor.lock().unwrap().update(game);
            })
        }).collect();
    }
}

impl GameState for MovementGameState {
    fn should_exit(&self) -> bool { true }

    fn update(&mut self, pcs: &Actors, npcs: &Actors, game_info: SafeGameInfo) {
        self.update_actors(pcs, &game_info);
        self.update_game_info(&pcs[0], &game_info);
        self.update_actors(npcs, &game_info);
    }
}

pub struct AttackInputGameState {
    should_exit: bool,
    should_scold: bool
}

impl AttackInputGameState {
    pub fn new() -> AttackInputGameState {
        AttackInputGameState {
            should_exit: false,
            should_scold:  false
        }
    }
}

impl GameState for AttackInputGameState {
    fn should_exit(&self) -> bool {
        self.should_exit
    }

    fn enter(&self, game_info: SafeGameInfo) {
        let game_info = game_info.read().unwrap();
        let _ = game_info.sender.send(print_action("Which direction do you want to attack? [Use arrow keys to answer]", Window::Input));
    }

    fn update(&mut self, _: &Actors, _: &Actors, game_info: SafeGameInfo) {
        let game_info    = game_info.read().unwrap();

        match game_info.keypress.key {
            Printable('w') |
            SpecialKey(KeyCode::Up) => {
                let _ = game_info.sender.send(print_action("You attack up!", Window::Messages));
                self.should_exit = true;
            },
            Printable('s') |
            SpecialKey(KeyCode::Down) => {
                let _ = game_info.sender.send(print_action("You attack down!", Window::Messages));
                self.should_exit = true;
            },
            Printable('a') |
            SpecialKey(KeyCode::Left) => {
                let _ = game_info.sender.send(print_action("You attack left!", Window::Messages));
                self.should_exit = true;
            },
            Printable('d') |
            SpecialKey(KeyCode::Right) => {
                let _ = game_info.sender.send(print_action("You attack right!", Window::Messages));
                self.should_exit = true;
            },
            Printable('/') => {
                if self.should_scold {
                    let _ = game_info.sender.send(print_action("You're already attacking, pick a direction.", Window::Messages));
                }
            },
            _ => {
                if self.should_scold {
                    let _ = game_info.sender.send(print_action("That's not a direction.", Window::Messages));
                }
            }
        }

        self.should_scold = true;
    }

    fn exit(&self, game_info: SafeGameInfo) {
        let game_info = game_info.read().unwrap();
        let _ = game_info.sender.send(RenderAction::Flush(Window::Input));
    }
}
