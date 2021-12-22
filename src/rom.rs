use crate::state;
use crate::state::InternalState;

impl InternalState {
    /// Loads the respective bytes into the program area
    /// in memory.
    /// 
    /// It will panic if rom_bytes.len() > 0xDFF
    pub fn load_rom_to_memory(&mut self, rom_bytes: &Vec<u8>) {
        println!("Rom size: {} bytes", rom_bytes.len());
        // Assure rom size can fit in memory
        if rom_bytes.len() > state::MEMSIZE - state::PROGRAM_START {
            panic!("Rom is too large.");
        }
        
        // Copies rom to program area in memory
        self.main_memory[state::PROGRAM_START..(rom_bytes.len() + state::PROGRAM_START)]
            .clone_from_slice(rom_bytes);
    }
}