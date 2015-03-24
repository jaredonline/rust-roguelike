extern crate tcod;

pub enum Key {
    SpecialKey(KeyCode)
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
    pub key: Key
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
        let key = match key_state.key {
            // other
            self::tcod::Key::Special(tcod::KeyCode::Escape) => Key::SpecialKey(KeyCode::Esc),

            // directions
            self::tcod::Key::Special(tcod::KeyCode::Up)    => Key::SpecialKey(KeyCode::Up),
            self::tcod::Key::Special(tcod::KeyCode::Down)  => Key::SpecialKey(KeyCode::Down),
            self::tcod::Key::Special(tcod::KeyCode::Left)  => Key::SpecialKey(KeyCode::Left),
            self::tcod::Key::Special(tcod::KeyCode::Right) => Key::SpecialKey(KeyCode::Right),

            // default
            _ => Key::SpecialKey(KeyCode::None)
        };

        KeyboardInput { key: key }
    }
}
