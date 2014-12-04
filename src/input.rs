#![allow(non_snake_case)]
extern crate tcod;

use self::tcod::{KeyState};

pub enum Key {
    Printable(char),
    SpecialKey(KeyCode)
}

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
                self::tcod::Special(tcod::KeyCode::Number5) => Key::Printable('%'),
                self::tcod::Special(tcod::KeyCode::Number6) => Key::Printable('^'),
                self::tcod::Special(tcod::KeyCode::Number8) => Key::Printable('*'),
                _                                           => Key::SpecialKey(KeyCode::None)
            }
        } else {
            match key_state.key {
                self::tcod::Printable('/')                 => Key::Printable('/'),
                self::tcod::Special(tcod::KeyCode::Up)     => Key::SpecialKey(KeyCode::Up),
                self::tcod::Special(tcod::KeyCode::Down)   => Key::SpecialKey(KeyCode::Down),
                self::tcod::Special(tcod::KeyCode::Left)   => Key::SpecialKey(KeyCode::Left),
                self::tcod::Special(tcod::KeyCode::Right)  => Key::SpecialKey(KeyCode::Right),
                self::tcod::Special(tcod::KeyCode::Shift)  => Key::SpecialKey(KeyCode::Shift),
                self::tcod::Special(tcod::KeyCode::Escape) => Key::SpecialKey(KeyCode::Escape),
                _                                          => Key::SpecialKey(KeyCode::None)
            }
        };

        KeyboardInput { key: key }
    }
}
