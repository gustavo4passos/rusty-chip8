use crate::chip8::KeyboardKey;

pub struct KeyboardState {
    pub keys: [bool; KeyboardKey::Total as usize]
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
    fn process_input(&mut self, keyboard_state: &mut KeyboardState);
}