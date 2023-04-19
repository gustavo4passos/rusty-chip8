use crate::state::InternalState;
use crate::window::Window;
use crate::input::InputBackend;
use std::time::{Duration, Instant};


impl InternalState {
    pub fn run(&mut self) {
        self.setup();
        self.run_main_loop();
    }

    pub fn run_main_loop(&mut self) {
        let mut w = Window::new(800, 600);
        w.init();

        loop {
            if w.should_close() { break };
            w.get_keyboard_state(&mut self.keyboard_state);
            
            const INSTRUCTION_TIME_US: u32 = 1400000;
            while !self.halted_for_keypress && self.time_since_last_op.as_nanos() > INSTRUCTION_TIME_US.into() {
                let next_inst = self.fetch_next();
                let next_inst_decoded = InternalState::decode_instr(next_inst);         
                self.advance_pc();
                self.execute_instruction(&next_inst_decoded);
                self.time_since_last_op -= Duration::new(0, INSTRUCTION_TIME_US);
            }

            if self.halted_for_keypress {
                self.time_since_last_op = Duration::new(0, 0);
                for (i, status) in self.keyboard_state.keys.iter().enumerate() {
                    if *status {
                        self.registers[self.halted_keypress_store_reg] = i as u16;
                        self.halted_for_keypress = false;
                        break;
                    }
                }
            }
            
            let now = Instant::now();
            let elapsed: Duration = now - self.previous_tick;
            self.time_since_last_op += elapsed;
            self.previous_tick = now;
            
            let t_since_last_vsync = now - self.previous_vsync;
            if t_since_last_vsync.as_millis() > 16 {
                // display::draw_console(&self.framebuffer);
                self.previous_vsync = Instant::now();
            }

            self.handle_timer(&elapsed);
            w.draw(&self.framebuffer);
            w.update();

            // log_debug!("PC: {:#x}", self.registers[Register::PC as usize] - 0x200);
        }
    }    
}