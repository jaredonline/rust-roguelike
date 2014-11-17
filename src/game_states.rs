use std::rc::Rc;
use std::cell::RefCell;

use rendering::windows::Windows;
use rendering::renderers::RenderingComponent;
use map::Maps;
use game::MoveInfo;
use input::{KeyCode, SpecialKey};
use combat::{Weapon, Boomerang};

pub trait GameState<'a> {
    fn new() -> Self;
    fn new_with_weapon(Box<Weapon + 'a>) -> Self;

    fn enter(&self, &mut Windows) {}
    fn exit(&self)  {}

    fn update(&mut self, maps: &mut Maps, windows: &mut Windows, Rc<RefCell<MoveInfo>>);
    fn render<'r>(&mut self, renderer: &mut Box<RenderingComponent>, maps: &mut Maps, windows: &'r mut Windows<'r>) {
        renderer.before_render_new_frame();
        let mut all_windows = windows.all_windows();
        for window in all_windows.iter_mut() {
            renderer.attach_window(*window);
        }
        maps.render(renderer);
        renderer.after_render_new_frame();
    }

    fn should_update_state(&self) -> bool;
}

pub struct MovementGameState;

impl<'a> GameState<'a> for MovementGameState {
    fn new() -> MovementGameState {
        MovementGameState
    }

    fn new_with_weapon(_: Box<Weapon + 'a>) -> MovementGameState {
        MovementGameState
    }

    fn should_update_state(&self) -> bool {
        true
    }

    fn enter(&self, windows: &mut Windows) {
        windows.input.flush_buffer();
    }

    fn update(&mut self, maps: &mut Maps, windows: &mut Windows, move_info: Rc<RefCell<MoveInfo>>) {
        let last_keypress = {
            move_info.borrow().deref().last_keypress
        };
        match last_keypress {
            Some(ks) => {
                match ks.key {
                    // Because Shift is used for attack keys we don't want to do
                    // anything when it's pushed. We can check for shift when we
                    // process the next keypress
                    SpecialKey(KeyCode::Shift) => {},
                    _ => {
                        maps.update(windows);
                    }
                }
            },
            _    => {}
        }
    }
}

pub struct AttackInputGameState<'a> {
    should_update_state: bool,
    pub weapon: Box<Weapon + 'a>
}

impl<'a> GameState<'a> for AttackInputGameState<'a> {
    fn new() -> AttackInputGameState<'a> {
        let weapon : Box<Boomerang> = box Weapon::new();
        AttackInputGameState {
            should_update_state: false,
            weapon: weapon
        }
    }

    fn new_with_weapon(weapon: Box<Weapon + 'a>) -> AttackInputGameState<'a> {
        AttackInputGameState {
            should_update_state: false,
            weapon: weapon
        }
    }

    fn should_update_state(&self) -> bool {
        self.should_update_state
    }

    fn enter(&self, windows: &mut Windows) {
        windows.input.flush_buffer();
        let mut msg = "Which direction do you want to attack with ".to_string();
        msg.push_str(self.weapon.get_name().as_slice());
        msg.push_str("? [Use the arrow keys to answer]");
        windows.input.buffer_message(msg.as_slice())
    }

    fn update(&mut self, maps: &mut Maps, windows: &mut Windows, move_info: Rc<RefCell<MoveInfo>>) {
        let last_keypress = {
            move_info.borrow().deref().last_keypress
        };
        match last_keypress {
            Some(ks) => {
                let mut msg = "You attack ".to_string();
                let mut point = {
                    move_info.borrow().deref().char_location.clone()
                };
                match ks.key {
                    SpecialKey(KeyCode::Up) => {
                        point = point.offset_y(-1);
                        msg.push_str("up");
                        self.should_update_state = true;
                    },
                    SpecialKey(KeyCode::Down) => {
                        point = point.offset_y(1);
                        msg.push_str("down");
                        self.should_update_state = true;
                    },
                    SpecialKey(KeyCode::Left) => {
                        point = point.offset_x(-1);
                        msg.push_str("left");
                        self.should_update_state = true;
                    },
                    SpecialKey(KeyCode::Right) => {
                        point = point.offset_x(1);
                        msg.push_str("right");
                        self.should_update_state = true;
                    },
                    _ => {}
                }

                if self.should_update_state {
                    match maps.enemy_at(point) {
                        Some(_) => {
                            msg.push_str(" with your ");
                            msg.push_str(self.weapon.get_name().as_slice());
                            msg.push_str(" for ");
                            msg.push_str(self.weapon.deal_damage().to_string().as_slice());
                            msg.push_str(" points of damage!");
                            windows.messages.buffer_message(msg.as_slice());
                        },
                        None => {
                            windows.messages.buffer_message("No enemy in that direction!");
                        }
                    }
                }
            },
            _ => {}
        }
    }
}
