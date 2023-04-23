use crate::chip8;
use crate::chip8::Chip8;

impl Chip8 {
    /// Loads the respective bytes into the program area
    /// in memory.
    /// 
    /// It will panic if rom_bytes.len() > 0xDFF
    pub fn load_rom_to_memory(&mut self, rom_bytes: &Vec<u8>) {
        println!("Rom size: {} bytes", rom_bytes.len());
        // Assure rom size can fit in memory
        if rom_bytes.len() > chip8::MEMSIZE - chip8::PROGRAM_START {
            panic!("Rom is too large.");
        }
       
        // Copies rom to program area in memory
        self.main_memory[chip8::PROGRAM_START..(rom_bytes.len() + chip8::PROGRAM_START)]
            .clone_from_slice(rom_bytes);
    }
}