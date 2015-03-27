use std::thread;

use actor::{Actors,};
use game::SafeGameInfo;
use rendering::{print_action, RenderAction, Window};
use input::Key::{SpecialKey, Printable};
use input::KeyCode;
use map::MapType;

pub trait GameState {
    fn update(&mut self, SafeGameInfo);

    fn should_exit(&self) -> bool;
    fn exit(&self, SafeGameInfo) {}
    fn enter(&self, SafeGameInfo) {}
}

pub struct MovementGameState;

impl MovementGameState {
    pub fn new() -> MovementGameState { MovementGameState }

    fn update_game_info(&self, actors: &Actors, game_info: &SafeGameInfo, map_type: MapType) {
        let mut game = game_info.write().unwrap();
        for actor in actors {
            let m_type = map_type.clone();
            match m_type {
                MapType::Pcs => game.char_position = actor.lock().unwrap().position.clone(),
                _            => {}
            }

            game.map.push_actor(actor.clone(), m_type);
        }
    }

    fn update_actors(&self, actors: &Actors, game_info: &SafeGameInfo, map_type: MapType) {
        let _ : Vec<_> = actors.iter().map(|a| {
            let actor = a.clone();
            let game  = game_info.clone();

            thread::scoped(move || {
                let a = actor.clone();
                a.lock().unwrap().update(game);
            })
        }).collect();

        self.update_game_info(actors, game_info, map_type);
    }
}

impl GameState for MovementGameState {
    fn should_exit(&self) -> bool { true }

    fn update(&mut self, game_info: SafeGameInfo) {
        let (enemies, pcs, terrain, friends) = {
            let mut game_info = game_info.write().unwrap();
            let e = game_info.map.pop_actors(MapType::Enemies);
            let p = game_info.map.pop_actors(MapType::Pcs);
            let t = game_info.map.pop_actors(MapType::Terrain);
            let f = game_info.map.pop_actors(MapType::Friends);
            (e, p, t, f)
        };
        self.update_actors(&pcs, &game_info, MapType::Pcs);
        self.update_actors(&friends, &game_info, MapType::Friends);
        self.update_actors(&enemies, &game_info, MapType::Enemies);
        self.update_actors(&terrain, &game_info, MapType::Enemies);
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

    fn update(&mut self, game_info: SafeGameInfo) {
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
