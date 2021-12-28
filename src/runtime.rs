use crate::{state::InternalState, exec::InstructionType};
use crate::display;
use crate::state::Register;
use crate::state::{ DISPLAYH, DISPLAYW };
use crate::log_debug;
use std::time::{Duration, Instant};

impl InternalState {
    pub fn run(&mut self) {
        self.setup();
        self.run_main_loop();
    }

    pub fn run_main_loop(&mut self) {
        loop {
            
            while self.time_since_last_op.as_micros() > 1400 {
                let next_inst = self.fetch_next();
                let next_inst_decoded = InternalState::decode_instr(next_inst);         
                self.advance_pc();
                self.execute_instruction(&next_inst_decoded);
                self.time_since_last_op -= Duration::new(0, 1400000);
            } 
            
            let now = Instant::now();
            let elapsed: Duration = now - self.previous_tick;
            self.time_since_last_op += elapsed;
            self.previous_tick = now;
            
            let t_since_last_vsync = now - self.previous_vsync;
            if t_since_last_vsync.as_millis() > 16 {
                display::draw_console(&self.framebuffer);
                self.previous_vsync = Instant::now();
                
            }

            self.handle_timer(&elapsed);

            log_debug!("PC: {:#x}", self.registers[Register::PC as usize] - 0x200);
        }
    }    
}