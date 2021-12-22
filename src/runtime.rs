use crate::state::InternalState;
use crate::state::Register;
use crate::display;
use std::time::{Duration, Instant};

impl InternalState {
    pub fn run(&mut self) {
        self.setup();
        self.run_main_loop();
    }

    pub fn run_main_loop(&mut self) {
        let mut i = 0;
        loop {
            let before = Instant::now();
            
            let next_inst = self.fetch_next();
            let next_inst_decoded = InternalState::decode_instr(next_inst);         
            self.advance_pc();
            // println!("Next instruction is: {}", next_inst_decoded as u8);
            self.execute_instruction(&next_inst_decoded);

            display::draw_console(&self.framebuffer);
            let later = Instant::now();
            let elapsed: Duration = later - before;
            println!("PC: {:#x}", self.registers[Register::PC as usize] - 0x200);
            // println!("Time passed: {}", elapsed.as_nanos());
        }
    }    
}