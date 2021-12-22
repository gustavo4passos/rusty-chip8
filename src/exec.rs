use crate::state::PROGRAM_START;
use crate::state::InternalState;
use crate::state::Register;
use crate::state::Color;
use crate::state::FONTS;
use crate::utils;
use crate::utils::nibbles_to_tuple;

pub enum InstructionType {
    CLS,
    JP { addr: u16 },
    LDV { vx: u8, value: u8  },
    ADDV { vx: u8, value: u8 },
    LDI { value: u16 },
    DRW { vx: u8, vy: u8, bytes: u8 },
    UNKNOWN
}

impl InternalState {
    pub fn setup(&mut self) {
        self.registers[Register::PC as usize] = PROGRAM_START as u16;
        // Copy fonts to memory
        self.main_memory[0..FONTS.len()].clone_from_slice(&FONTS);
    }

    pub fn fetch_next(&self) -> u16 {
        let nxt_inst_addr: u16 = self.registers[Register::PC as usize];
        utils::concat_u8_to_u16(
            self.main_memory[nxt_inst_addr as usize],
            self.main_memory[(nxt_inst_addr + 1) as usize],
        )
    }

    pub fn decode_instr(data: u16) -> InstructionType {
        let d = nibbles_to_tuple(data);
        println!("Nibbles: {:#x} {:#x} {:#x} {:#x}", d.0, d.1, d.2, d.3);

        match utils::nibbles_to_tuple(data) {
            (0x0, 0, 0xE, 0) => InstructionType::CLS,
            (0x1, n2, n1, n0) => InstructionType::JP{ addr: utils::concat_nib_to_u16(0, n2, n1, n0) },
            (0x6, vx, n1, n0) => InstructionType::LDV{ vx, value: utils::concat_nib_to_u8(n1, n0) },
            (0x7, vx, n1, n0) => InstructionType::ADDV{ vx, value: utils::concat_nib_to_u8(n1, n0)},
            (0xA, n2, n1, n0) => InstructionType::LDI{ value: utils::concat_nib_to_u16(0, n2, n1, n0) },
            (0xD, vx, vy, bytes) => InstructionType::DRW{ vx, vy, bytes },
            _ => InstructionType::UNKNOWN
        }
    }

    pub fn execute_instruction(&mut self, instruction: &InstructionType) {
        match instruction {
            InstructionType::JP{ addr} => self.jmp(*addr),
            InstructionType::CLS => self.cls(),
            InstructionType::LDV{ vx, value} => self.ldv(*vx, *value),
            InstructionType::ADDV{ vx, value} => self.addv(*vx, *value),
            InstructionType::LDI{ value } => self.ldi(*value),
            InstructionType::DRW{ vx, vy, bytes} => self.drw(*vx, *vy, *bytes),
            _ => ()
        }
    }

    pub fn get_vx_i(vx: u8) -> usize {
        // Only registers V0 to VF are valid
        assert!(vx <= 0xF);
        ((Register::V0 as u8) + vx) as usize
    }

    pub fn jmp(&mut self, address: u16) {
        println!("Jumping to {:#x}", address);
        self.registers[Register::PC as usize] = address;
    }

    pub fn cls(&mut self) {
        println!("Clearing screen.");
        self.framebuffer.iter_mut().for_each(|e| *e = 0);
    }

    pub fn ldv(&mut self, vx: u8, value: u8) {
        println!("Loading to register {}", InternalState::get_vx_i(vx));
        self.registers[InternalState::get_vx_i(vx)] = value as u16;
    }

    pub fn addv(&mut self, vx: u8, value: u8) {
        self.registers[InternalState::get_vx_i(vx)] += value as u16;
    }

    pub fn ldi(&mut self, value: u16) {
        self.registers[Register::I as usize] = value;
    }

    pub fn drw(&mut self, vx: u8, vy: u8, bytes: u8) {
        println!("Drawing from {} to {} for {} bytes", vx, vy, bytes);

        let x_coord = self.registers[InternalState::get_vx_i(vx)];
        let y_coord = self.registers[InternalState::get_vx_i(vy)];
        
        
        for i in (0..bytes).rev() {
            let i_value = self.registers[Register::I as usize] + i as u16;
            let data = self.main_memory[i_value as usize];
            println!("vx {} vy {} bytes {} I {:#x} data {:#x}", x_coord, y_coord, bytes, i_value, data);
            for j in (0..8).rev() {
                println!("Bit {}: {}", j, utils::get_nth_bit_u16(data as u16, j as u8));
                let fb_index = InternalState::get_fb_i_from_coord_in_fb(x_coord + j, y_coord + i as u16);
                self.framebuffer[fb_index] = match utils::get_nth_bit_u16(data as u16, (7 - j) as u8) {
                    0x0 => Color::Black as u8,
                    0x1 => Color::White as u8,
                    _ => panic!("Invalid sprite pixel value {}", data)
                }
            }
        }
    }

    pub fn advance_pc(&mut self) {
        self.registers[Register::PC as usize] += 2; 
    }
}
