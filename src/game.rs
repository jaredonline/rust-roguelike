extern crate tcod;

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
use input::{KeyboardInput, Printable,};
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

        let rc  : Box<TcodRenderingComponent>      = box RenderingComponent::new(total_bounds);

        let sw  : Box<TcodStatsWindowComponent>    = box WindowComponent::new(stats_bounds);
        let iw  : Box<TcodInputWindowComponent>    = box WindowComponent::new(input_bounds);
        let mw  : Box<TcodMessagesWindowComponent> = box WindowComponent::new(message_bounds);
        let maw : Box<TcodMapWindowComponent>      = box WindowComponent::new(map_bounds);

        let windows = Windows {
            input:    iw,
            messages: mw,
            map:      maw,
            stats:    sw
        };

        let gs : Box<MovementGameState> = box GameState::new();

        let move_info = Rc::new(RefCell::new(MoveInfo::new(map_bounds)));
        let maps = Maps::new(move_info.clone());

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
        self.game_state.render(&mut self.rendering_component, &mut self.maps, &mut self.windows);
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
                        let w : Box<Sword> = box Weapon::new();
                        let is : Box<AttackInputGameState> = box GameState::new_with_weapon(w);
                        self.game_state = is as Box<GameState>;
                    },
                    Printable('^') => {
                        let w : Box<Boomerang> = box Weapon::new();
                        let is : Box<AttackInputGameState> = box GameState::new_with_weapon(w);
                        self.game_state = is as Box<GameState>;
                    },
                    Printable('*') => {
                        let w : Box<Boomerang> = box Weapon::new();
                        let is : Box<AttackInputGameState> = box GameState::new_with_weapon(w);
                        self.game_state = is as Box<GameState>;
                    },
                    Printable('%') => {
                        let w : Box<Boomerang> = box Weapon::new();
                        let is : Box<AttackInputGameState> = box GameState::new_with_weapon(w);
                        self.game_state = is as Box<GameState>;
                    },
                    _ => {
                        let ms : Box<MovementGameState> = box GameState::new();
                        self.game_state = ms as Box<GameState>;
                    }
                }
            },
            _ => {}
        }
    }
}
