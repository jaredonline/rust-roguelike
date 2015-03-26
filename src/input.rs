extern crate tcod;

pub enum Key {
    SpecialKey(KeyCode),
    Printable(char)
}

pub enum KeyCode {
    // Arrow keys
    Up,
    Down,
    Left,
    Right,

    // Other
    Esc,

    // None
    None
}

pub struct KeyboardInput {
    pub key: Key,
    pub shift: bool
}

pub trait InputComponent<T> {
    fn translate_key(&self, T) -> KeyboardInput;
}

pub struct TcodInputComponent;

impl TcodInputComponent {
    pub fn new() -> TcodInputComponent { TcodInputComponent }
}

impl InputComponent<self::tcod::KeyState> for TcodInputComponent {
    fn translate_key(&self, key_state: self::tcod::KeyState) -> KeyboardInput {
        let mut shift = false;
        let key = if key_state.shift {
            shift = true;
            match key_state.key {
                // shift + numbers = printable
                self::tcod::Key::Special(tcod::KeyCode::Number5) => Key::Printable('%'),
                self::tcod::Key::Special(tcod::KeyCode::Number6) => Key::Printable('^'),
                self::tcod::Key::Special(tcod::KeyCode::Number8) => Key::Printable('*'),

                _ => Key::SpecialKey(KeyCode::None)
            }
        } else {
            match key_state.key {
                // other
                self::tcod::Key::Special(tcod::KeyCode::Escape) => Key::SpecialKey(KeyCode::Esc),

                // directions
                self::tcod::Key::Special(tcod::KeyCode::Up)    => Key::SpecialKey(KeyCode::Up),
                self::tcod::Key::Special(tcod::KeyCode::Down)  => Key::SpecialKey(KeyCode::Down),
                self::tcod::Key::Special(tcod::KeyCode::Left)  => Key::SpecialKey(KeyCode::Left),
                self::tcod::Key::Special(tcod::KeyCode::Right) => Key::SpecialKey(KeyCode::Right),

                // printable
                self::tcod::Key::Printable('/') => Key::Printable('/'),
                self::tcod::Key::Printable('a') => Key::Printable('a'),
                self::tcod::Key::Printable('s') => Key::Printable('s'),
                self::tcod::Key::Printable('d') => Key::Printable('d'),
                self::tcod::Key::Printable('w') => Key::Printable('w'),

                // default
                _ => Key::SpecialKey(KeyCode::None)
            }
        };

        KeyboardInput { key: key, shift: shift }
    }
}
