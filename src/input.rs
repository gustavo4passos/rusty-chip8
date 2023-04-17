use crate::state::KeyboardKey;

pub struct KeyboardState {
    keys: [bool; KeyboardKey::Total as usize]
}

impl KeyboardState {
    pub fn new() -> KeyboardState {
        KeyboardState { 
            keys: [false; KeyboardKey::Total as usize]
        }
    }

    pub fn get_key_state(&self, key: KeyboardKey) -> bool {
        self.keys[key as usize]
    }

    pub fn get_key_state_u8(&self, key: u8) -> bool {
        if key >= KeyboardKey::Total as u8 { false }
        else { self.keys[key as usize] }
    }

    pub fn set_key_state(&mut self, key: KeyboardKey, state: bool) {
        self.keys[key as usize] = state;
    }
}

pub trait InputBackend {
    fn get_keyboard_state(&mut self, keyboard_state: &mut KeyboardState);
}