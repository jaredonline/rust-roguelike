extern crate tcod;
extern crate core;

use std::cell::RefCell;
use std::rc::Rc;

use util::{Point, Bound,};
use rendering::renderers::{RenderingComponent, TcodRenderingComponent};
use rendering::windows::{
    Windows,
    WindowComponent,
    TcodStatsWindowComponent,
    TcodInputWindowComponent,
    TcodMapWindowComponent,
    TcodMessagesWindowComponent
};
use input::{KeyboardInput,};
use input::Key::{Printable,};
use map::Maps;
use game_states::{
    GameState,
    MovementGameState,
    AttackInputGameState
};
use combat::{
    Weapon,
    Boomerang,
    Lettuce,
    Sword,
    Bomb
};
use actor::Actor;

use self::core::ops::{Deref, DerefMut};

pub struct MoveInfo {
    pub last_keypress: Option<KeyboardInput>,
    pub char_location: Point,
    pub bounds: Bound
}

impl MoveInfo {
    pub fn new(bound: Bound) -> MoveInfo {
        MoveInfo {
            last_keypress: None,
            char_location: Point::new(40, 25),
            bounds: bound
        }
    }
}

pub struct Game {
    pub move_info:           Rc<RefCell<MoveInfo>>,
    pub exit:                bool,
    pub window_bounds:       Bound,
    pub rendering_component: Box<RenderingComponent + 'static>,
    pub game_state:          Box<GameState      + 'static>,
    pub windows:             Windows,
    pub maps:                Maps
}

impl Game {
    pub fn new() -> Game {
        let total_bounds   = Bound::new(0,  0, 99, 61);
        let stats_bounds   = Bound::new(79, 0, 99, 49);
        let input_bounds   = Bound::new(0, 50, 99, 51);
        let message_bounds = Bound::new(0, 52, 99, 61);
        let map_bounds     = Bound::new(0,  0, 78, 49);

        let rc = Box::new(TcodRenderingComponent::new(total_bounds));

        let sw  = Box::new(TcodStatsWindowComponent::new(stats_bounds));
        let iw  = Box::new(TcodInputWindowComponent::new(input_bounds));
        let mw  = Box::new(TcodMessagesWindowComponent::new(message_bounds));
        let maw = Box::new(TcodMapWindowComponent::new(map_bounds));

        let windows = Windows {
            input:    iw,
            messages: mw,
            map:      maw,
            stats:    sw
        };

        let gs = Box::new(MovementGameState::new());

        let move_info = Rc::new(RefCell::new(MoveInfo::new(map_bounds)));
        let mut maps = Maps::new(move_info.clone());

        maps.friends.push_actor(Point::new(10, 10), Box::new(Actor::dog(10, 10, move_info.clone())));
        maps.friends.push_actor(Point::new(40, 25), Box::new(Actor::cat(40, 25, move_info.clone())));
        maps.enemies.push_actor(Point::new(20, 20), Box::new(Actor::kobold(20, 20, move_info.clone())));

        let char_location = {
            move_info.borrow().deref().char_location
        };
        maps.pcs.push_actor(char_location, Box::new(Actor::heroine(move_info.clone())));

        Game {
            exit:                false,
            window_bounds:       total_bounds,
            rendering_component: rc,
            windows:             windows,
            game_state:          gs,
            maps:                maps,
            move_info:           move_info
        }
    }

    pub fn render(&mut self) {
        let ref mut render_component = self.rendering_component;
        self.game_state.render(render_component, &mut self.maps, &mut self.windows);
    }

    pub fn update(&mut self) {
        if self.game_state.should_update_state() {
            self.game_state.exit();
            self.update_state();
            self.game_state.enter(&mut self.windows);
        }

        self.game_state.update(&mut self.maps, &mut self.windows, self.move_info.clone());
    }

    pub fn wait_for_keypress(&mut self) -> KeyboardInput {
        let key_state = self.rendering_component.wait_for_keypress();

        {
            self.move_info.borrow_mut().deref_mut().last_keypress = Some(key_state);
        }
        return key_state;
    }

    fn update_state(&mut self) {
        let last_keypress = {
            self.move_info.borrow().deref().last_keypress
        };
        match last_keypress {
            Some(ks) => {
                match ks.key {
                    Printable('/') => {
                        let w = Box::new(Sword::new());
                        let is = Box::new(AttackInputGameState::new_with_weapon(w));
                        self.game_state = is;
                    },
                    Printable('^') => {
                        let w = Box::new(Boomerang::new());
                        let is = Box::new(AttackInputGameState::new_with_weapon(w));
                        self.game_state = is;
                    },
                    Printable('*') => {
                        let w = Box::new(Boomerang::new());
                        let is = Box::new(AttackInputGameState::new_with_weapon(w));
                        self.game_state = is;
                    },
                    Printable('%') => {
                        let w = Box::new(Boomerang::new());
                        let is = Box::new(AttackInputGameState::new_with_weapon(w));
                        self.game_state = is;
                    },
                    _ => {
                        let ms = Box::new(MovementGameState::new());
                        self.game_state = ms;
                    }
                }
            },
            _ => {}
        }
    }
}
