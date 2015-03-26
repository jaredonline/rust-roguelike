use std::sync::{Arc, RwLock};
use std::sync::mpsc;
use std::sync::mpsc::Sender;

use util::{Bound, Point};
use rendering::{RenderingComponent, TcodRenderingComponent, RenderAction};
use actor::{Actors,};
use input::{KeyboardInput, KeyCode};
use input::Key::{SpecialKey, Printable};
use game_state::{GameState, MovementGameState, AttackInputGameState};

pub struct GameInfo {
    pub window_bounds: Bound,
    pub keypress:      KeyboardInput,
    pub exit:          bool,
    pub char_position: Point,
    pub shift:         bool,
    pub sender:        Sender<RenderAction>
}

unsafe impl Send for GameInfo {}
unsafe impl Sync for GameInfo {}
pub type SafeGameInfo = Arc<RwLock<GameInfo>>;

pub struct Game {
    renderer:  Box<RenderingComponent + 'static>,
    game_info: SafeGameInfo,
    state:     Box<GameState + 'static>
}

impl Game {
    pub fn new() -> Game {
        // set our bounds
        let window_bounds = Bound::new(0,  0, 99, 61);
        let map_bounds    = Bound::new(0,  0, 78, 49);
        let stats_bounds  = Bound::new(79, 0, 99, 49);
        let mes_bounds    = Bound::new(0, 53, 99, 61);
        let ui_bounds     = Bound::new(0, 50, 99, 52);

        let (s, r) = mpsc::channel();

        let renderer      = Box::new(TcodRenderingComponent::new(&window_bounds, &map_bounds, &stats_bounds, &mes_bounds, &ui_bounds, r));
        let input         = KeyboardInput { key: SpecialKey(KeyCode::None), shift: false };

        let gi = Arc::new(RwLock::new(GameInfo {
            window_bounds: map_bounds,
            keypress:      input,
            exit:          false,
            char_position: Point { x: 0, y: 0 },
            shift:         false,
            sender:        s
        }));

        let state = Box::new(MovementGameState::new());

        Game {
            renderer:  renderer,
            game_info: gi,
            state:     state
        }
    }

    pub fn exit(&self) -> bool {
        self.renderer.closed() || self.game_info.read().unwrap().exit
    }

    pub fn render(&mut self, pcs: &Actors, actors: &Actors) {
        self.renderer.before_render_new_frame();
        self.render_actors(actors);
        self.render_actors(pcs);
        self.renderer.after_render_new_frame();
    }

    fn render_actors(&mut self, actors: &Actors) {
        for a in actors {
            let actor = a.lock().unwrap();
            self.renderer.render_object(&actor.position, actor.display_char);
        }
    }

    pub fn update(&mut self, pcs: &Actors, actors: &Actors) {
        let shift = { self.game_info.read().unwrap().shift };
        if !shift {
            if self.state.should_exit() {
                self.state.exit(self.game_info.clone());
                self.update_state();
                self.state.enter(self.game_info.clone());
            }

            self.state.update(pcs, actors, self.game_info.clone());
        }
    }

    fn update_state(&mut self) {
        let game_info = self.game_info.read().unwrap();
        let gs : Box<GameState> = match game_info.keypress.key {
            Printable('/') => {
                Box::new(AttackInputGameState::new())
            },
            Printable('%') => {
                Box::new(AttackInputGameState::new())
            },
            Printable('*') => {
                Box::new(AttackInputGameState::new())
            },
            Printable('^') => {
                Box::new(AttackInputGameState::new())
            },
            _ => {
                Box::new(MovementGameState::new())
            }
        };

        self.state = gs;
    }

    pub fn wait_for_keypress(&mut self) {
        let mut game_info = self.game_info.write().unwrap();
        let key = self.renderer.wait_for_keypress();
        game_info.shift = false;

        match key.key {
            SpecialKey(KeyCode::Esc)  => { game_info.exit = true },
            SpecialKey(KeyCode::None) => {
                if key.shift {
                    game_info.shift = true
                }
            },
            _ => { }
        }

        game_info.keypress = key;
    }
}
