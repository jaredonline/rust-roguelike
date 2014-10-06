extern crate tcod;

use util::{Point, Bound};
use rendering::{RenderingComponent, WindowComponent};
use rendering::{
    TcodRenderingComponent,
    TcodStatsWindowComponent,
    TcodInputWindowComponent,
    TcodMapWindowComponent,
    TcodMessagesWindowComponent
};
use actor::Actor;
use input::{KeyboardInput, Printable, SpecialKey, KeyCode};

static mut LAST_KEYPRESS : Option<KeyboardInput> = None;
static mut CHAR_LOCATION : Point = Point { x: 40, y: 25 };

pub struct Windows<'a> {
    pub stats:    Box<WindowComponent + 'a>,
    pub map:      Box<WindowComponent + 'a>,
    pub input:    Box<WindowComponent + 'a>,
    pub messages: Box<WindowComponent + 'a>
}

impl<'a > Windows<'a > {
    fn all_windows(&'a mut self) -> Vec<&mut Box<WindowComponent>> {
        let windows = vec![
            &mut self.stats,
            &mut self.input,
            &mut self.messages,
            &mut self.map
        ];

        return windows;
    }
}

pub struct Game<'a> {
    pub exit:                bool,
    pub window_bounds:       Bound,
    pub rendering_component: Box<RenderingComponent       + 'a>,
    pub game_state:          Box<GameState                + 'a>,
    pub windows:             Windows<'a>
}

impl<'a> Game<'a> {
    pub fn get_last_keypress() -> Option<KeyboardInput> {
        unsafe { LAST_KEYPRESS }
    }

    pub fn set_last_keypress(ki: KeyboardInput) {
        unsafe { LAST_KEYPRESS = Some(ki); }
    }

    pub fn get_character_point() -> Point {
        unsafe { CHAR_LOCATION }
    }

    pub fn set_character_point(point: Point) {
        unsafe { CHAR_LOCATION = point; }
    }

    pub fn new() -> Game<'a> {
        let total_bounds   = Bound::new(0,  0, 99, 61);
        let stats_bounds   = Bound::new(79, 0, 99, 49);
        let input_bounds   = Bound::new(0, 50, 99, 52);
        let message_bounds = Bound::new(0, 53, 99, 61);
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

        Game {
            exit:                false,
            window_bounds:       total_bounds,
            rendering_component: rc,
            windows:             windows,
            game_state:          gs
        }
    }

    pub fn render(&mut self, npcs: &Vec<Box<Actor>>, c: &Actor) {
        self.game_state.render(&mut self.rendering_component, npcs, c, &mut self.windows);
    }

    pub fn update(&'a mut self, npcs: &mut Vec<Box<Actor>>, c: &mut Actor) {
        if self.game_state.should_update_state() {
            self.game_state.exit();
            self.update_state();
            self.game_state.enter(&mut self.windows);
        }

        self.game_state.update(npcs, c, &mut self.windows);
    }

    pub fn wait_for_keypress(&mut self) -> KeyboardInput {
        let key_state = self.rendering_component.wait_for_keypress();

        Game::set_last_keypress(key_state);
        return key_state;
    }

    fn update_state(&mut self) {
        match Game::get_last_keypress() {
            Some(ks) => {
                match ks.key {
                    Printable('/') => {
                        let mut is : Box<AttackInputGameState> = box GameState::new();
                        is.weapon = "Heroic Sword".to_string();
                        self.game_state = is as Box<GameState>;
                    },
                    Printable('^') => {
                        let mut is : Box<AttackInputGameState> = box GameState::new();
                        is.weapon = "Boomerang".to_string();
                        self.game_state = is as Box<GameState>;
                    },
                    Printable('*') => {
                        let mut is : Box<AttackInputGameState> = box GameState::new();
                        is.weapon = "Deadly Bomb".to_string();
                        self.game_state = is as Box<GameState>;
                    },
                    Printable('%') => {
                        let mut is : Box<AttackInputGameState> = box GameState::new();
                        is.weapon = "Delicious Lettuce".to_string();
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

pub trait GameState {
    fn new() -> Self;

    fn enter(&self, &mut Windows) {}
    fn exit(&self)  {}

    fn update(&mut self, npcs: &mut Vec<Box<Actor>>, character: &mut Actor, windows: &mut Windows);
    fn render(&mut self, renderer: &mut Box<RenderingComponent>, npcs: &Vec<Box<Actor>>, character: &Actor, windows: &mut Windows) {
        renderer.before_render_new_frame();
        let mut all_windows = windows.all_windows();
        for window in all_windows.iter_mut() {
            renderer.attach_window(*window);
        }
        for npc in npcs.iter() {
            npc.render(renderer);
        }
        character.render(renderer);
        renderer.after_render_new_frame();
    }

    fn should_update_state(&self) -> bool;
}

pub struct MovementGameState;

impl GameState for MovementGameState {
    fn new() -> MovementGameState {
        MovementGameState
    }

    fn should_update_state(&self) -> bool {
        true
    }

    fn enter(&self, windows: &mut Windows) {
        windows.input.flush_buffer();
    }

    fn update(&mut self, npcs: &mut Vec<Box<Actor>>, character: &mut Actor, windows: &mut Windows) {
        match Game::get_last_keypress() {
            Some(ks) => {
                match ks.key {
                    SpecialKey(KeyCode::Shift) => {},
                    _ => {
                        character.update(windows);
                        Game::set_character_point(character.position);
                        for npc in npcs.iter_mut() {
                            npc.update(windows);
                        }
                    }
                }
            },
            _    => {}
        }
    }
}

pub struct AttackInputGameState {
    should_update_state: bool,
    weapon: String
}

impl GameState for AttackInputGameState {
    fn new() -> AttackInputGameState {
        AttackInputGameState {
            should_update_state: false,
            weapon: "".to_string()
        }
    }

    fn should_update_state(&self) -> bool {
        self.should_update_state
    }

    fn enter(&self, windows: &mut Windows) {
        windows.input.flush_buffer();
        let mut msg = "Which direction do you want to attack with ".to_string();
        msg.push_str(self.weapon.as_slice());
        msg.push_str("? [Use the arrow keys to answer]");
        windows.input.buffer_message(msg.as_slice())
    }

    fn update(&mut self, _: &mut Vec<Box<Actor>>, _: &mut Actor, windows: &mut Windows) {
        match Game::get_last_keypress() {
            Some(ks) => {
                let mut msg = "You attack ".to_string();
                match ks.key {
                    SpecialKey(KeyCode::Up) => {
                        msg.push_str("up");
                        self.should_update_state = true;
                    },
                    SpecialKey(KeyCode::Down) => {
                        msg.push_str("down");
                        self.should_update_state = true;
                    },
                    SpecialKey(KeyCode::Left) => {
                        msg.push_str("left");
                        self.should_update_state = true;
                    },
                    SpecialKey(KeyCode::Right) => {
                        msg.push_str("right");
                        self.should_update_state = true;
                    },
                    _ => {}
                }

                if self.should_update_state {
                    msg.push_str(" with your ");
                    msg.push_str(self.weapon.as_slice());
                    msg.push_str("!");
                    windows.messages.buffer_message(msg.as_slice());
                }
            },
            _ => {}
        }
    }
}
