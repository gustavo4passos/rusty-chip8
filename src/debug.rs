use crate::state::InternalState;
use crate::state::Register;
use crate::state::{STACK_SIZE, STACK_START};
use crate::utils;
use crate::log_debug;


impl InternalState {
    pub fn reg_dump(&self) {
        println!("### REG DUMP ###");
        for n in 0..=0xF {
            println!("V{:#x}: {:#x}", n, self.registers[InternalState::get_vx_i(n)]);
        }
        println!("DT: {:#x}", self.get_register(Register::DT));
        println!("PC: {:#x}", self.get_register(Register::PC));
        println!("SP: {:#x}", self.get_register(Register::SP));
        println!("################");
        
    }

    pub fn stack_dump(&self) {
        log_debug!("### STCK DUMP ###");
        for n in 0..STACK_SIZE {
            if n % 2 != 0 { continue };
            let addr = STACK_START - (n as usize) - 2;
            let data = utils::concat_u8_to_u16(self.main_memory[addr - 1], self.main_memory[addr]);
            log_debug!("{:#x}: {:#x}", STACK_START - (n as usize), data);
        }
        log_debug!("#################");

    }
}