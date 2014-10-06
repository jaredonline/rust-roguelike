#![allow(non_snake_case)]
extern crate tcod;

use self::tcod::{key_code, KeyState};

pub enum Key {
    Printable(char),
    SpecialKey(KeyCode)
}

pub type KeyCode = self::KeyCode::KeyCode;
pub mod KeyCode {
    pub enum KeyCode {
        // Arrow keys
        Up,
        Down,
        Left,
        Right,

        // Special
        Shift,
        Escape,

        // Default
        None
    }
}

pub struct KeyboardInput {
    pub key: Key
}

pub trait InputComponent<T> {
    fn new() -> Self;
    fn translate_input(&self, T) -> KeyboardInput;
}

pub struct TcodInputComponent;

impl InputComponent<KeyState> for TcodInputComponent {
    fn new() -> TcodInputComponent { TcodInputComponent }

    fn translate_input(&self, key_state: KeyState) -> KeyboardInput {
        let key : Key = if key_state.shift {
            match key_state.key {
                self::tcod::Special(key_code::Number5) => Printable('%'),
                self::tcod::Special(key_code::Number6) => Printable('^'),
                self::tcod::Special(key_code::Number8) => Printable('*'),
                _                                      => SpecialKey(KeyCode::None)
            }
        } else {
            match key_state.key {
                self::tcod::Printable('/')            => Printable('/'),
                self::tcod::Special(key_code::Up)     => SpecialKey(KeyCode::Up),
                self::tcod::Special(key_code::Down)   => SpecialKey(KeyCode::Down),
                self::tcod::Special(key_code::Left)   => SpecialKey(KeyCode::Left),
                self::tcod::Special(key_code::Right)  => SpecialKey(KeyCode::Right),
                self::tcod::Special(key_code::Shift)  => SpecialKey(KeyCode::Shift),
                self::tcod::Special(key_code::Escape) => SpecialKey(KeyCode::Escape),
                _                                     => SpecialKey(KeyCode::None)
            }
        };

        KeyboardInput { key: key }
    }
}
