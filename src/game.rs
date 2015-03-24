use std::sync::{Arc, RwLock};
use std::thread;

use util::{Bound, Point};
use rendering::{RenderingComponent, TcodRenderingComponent};
use actor::{Actors, SafeActor};
use input::{KeyboardInput, KeyCode};
use input::Key::SpecialKey;

pub struct GameInfo {
    pub window_bounds: Bound,
    pub keypress:      KeyboardInput,
    pub exit:          bool,
    pub char_position: Point
}

pub type SafeGameInfo = Arc<RwLock<GameInfo>>;

pub struct Game {
    renderer:  Box<RenderingComponent + 'static>,
    game_info: SafeGameInfo
}

impl Game {
    pub fn new() -> Game {
        let window_bounds = Bound { min: Point { x: 0, y: 0 }, max: Point { x: 79, y: 49 } };
        let renderer      = Box::new(TcodRenderingComponent::new(&window_bounds));
        let input         = KeyboardInput { key: SpecialKey(KeyCode::None) };

        let gi = Arc::new(RwLock::new(GameInfo {
            window_bounds: window_bounds,
            keypress:      input,
            exit:          false,
            char_position: Point { x: 0, y: 0 }
        }));

        Game {
            renderer:  renderer,
            game_info: gi
        }
    }

    pub fn exit(&self) -> bool {
        self.renderer.closed() || self.game_info.read().unwrap().exit
    }

    pub fn render(&mut self, pcs: &Actors, actors: &Actors) {
        self.renderer.before_render_new_frame();
        self._render(actors);
        self._render(pcs);
        self.renderer.after_render_new_frame();
    }

    fn _render(&mut self, actors: &Actors) {
        for a in actors {
            let actor = a.lock().unwrap();
            self.renderer.render_object(&actor.position, actor.display_char);
        }
    }

    pub fn update(&mut self, pcs: &Actors, actors: &Actors) {
        self._update(pcs);
        self._update(actors);

        self._update_game_info(&pcs[0]);
    }

    fn _update_game_info(&mut self, a: &SafeActor) {
        let actor = a.clone();
        let game  = self.game_info.clone();

        game.write().unwrap().char_position = actor.lock().unwrap().position.clone();
    }

    fn _update(&mut self, actors: &Actors) {
        let _ : Vec<_> = actors.iter().map(|a| {
            let actor = a.clone();
            let game  = self.game_info.clone();

            thread::spawn(move || {
                actor.lock().unwrap().update(game);
            })
        }).collect();
    }

    pub fn wait_for_keypress(&mut self) {
        let mut game_info = self.game_info.write().unwrap();
        let key = self.renderer.wait_for_keypress();

        match key.key {
            SpecialKey(KeyCode::Esc) => { game_info.exit = true },
            _ => {}
        }

        game_info.keypress = key;
    }
}
