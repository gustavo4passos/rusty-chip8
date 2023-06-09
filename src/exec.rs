use std::time::{ Duration, Instant };

use crate::chip8::{ DISPLAYH, DISPLAYW, PROGRAM_START, STACK_START };
use crate::chip8::Chip8;
use crate::chip8::Register;
use crate::chip8::Color;
use crate::chip8::FONTS;
use crate::utils;
use crate::utils::nibbles_to_tuple;
use rand::random;

pub enum InstructionType {
    CLS,
    JP { addr: u16 },
    LDV { vx: u8, value: u8  },
    ADDV { vx: u8, value: u8 },
    LDI { value: u16 },
    LDR { vx: u8, vy: u8 },
    DRW { vx: u8, vy: u8, bytes: u8 },
    CALL { addr: u16 },
    RET,
    SKEQV { vx: u8, value: u8 },
    SKNEQV { vx: u8, value: u8 },
    RND { vx: u8, value: u8 },
    ADD { vx: u8, vy: u8 },
    ADDI { vx: u8 },
    SUB { vx: u8, vy: u8 },
    SHR { vx: u8, vy: u8 },
    SUBN { vx: u8, vy: u8 },
    SHL { vx: u8, vy: u8 },
    SETDT { vx: u8 },
    SKNP { vx: u8 },
    SKP { vx: u8 },
    SNER { vx: u8, vy: u8 },
    SER { vx: u8, vy: u8 },
    HALTKP { vx: u8 },
    LDDT { vx: u8 },
    LDBCD { vx: u8 },
    LDHEX { vx: u8 },
    OR { vx: u8, vy: u8 },
    AND { vx: u8, vy: u8 },
    XOR { vx: u8, vy: u8 },
    LDST { vx: u8 },
    LDVXI { vx: u8 },
    LDRI { vx: u8 },
    UNKNOWN
}

impl Chip8 {
    pub fn setup(&mut self) {
        self.registers[Register::PC as usize] = PROGRAM_START as u16;
        self.registers[Register::SP as usize] = STACK_START as u16;
        // Copy fonts to memory
        self.main_memory[0..FONTS.len()].clone_from_slice(&FONTS);
        self.previous_tick = Instant::now();
        self.previous_vsync = Instant::now();
    }

    pub fn run_cycles(&mut self) {
        const INSTRUCTION_TIME_NS: u32 = 1400000;
        while !self.halted_for_keypress && self.time_since_last_op.as_nanos() > INSTRUCTION_TIME_NS.into() {
            let next_inst = self.fetch_next();
            let next_inst_decoded = Chip8::decode_instr(next_inst);         
            self.advance_pc();
            self.execute_instruction(&next_inst_decoded);
            self.time_since_last_op -= Duration::new(0, INSTRUCTION_TIME_NS);
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
            self.previous_vsync = Instant::now();
        }

        self.handle_timer(&elapsed);
    }

    pub fn fetch_next(&self) -> u16 {
        let nxt_inst_addr: u16 = self.registers[Register::PC as usize];
        utils::concat_u8_to_u16(
            self.main_memory[nxt_inst_addr as usize],
            self.main_memory[(nxt_inst_addr + 1) as usize],
        )
    }

    pub fn decode_instr(data: u16) -> InstructionType {
        match utils::nibbles_to_tuple(data) {
            (0x0, 0x0, 0xE, 0x0) => InstructionType::CLS,
            (0x0, 0x0, 0xE, 0xE) => InstructionType::RET,
            (0x1, n2, n1, n0) => InstructionType::JP{ addr: utils::concat_nib_to_u16(0, n2, n1, n0) },
            (0x2, n2, n1, n0) => InstructionType::CALL{ addr: utils::concat_nib_to_u16(0, n2, n1, n0) },
            (0x3, vx, n1, n0) => InstructionType::SKEQV{ vx, value: utils::concat_nib_to_u8(n1, n0)},
            (0x4, vx, n1, n0) => InstructionType::SKNEQV{ vx, value: utils::concat_nib_to_u8(n1, n0)},
            (0x5, vx, vy, 0x0) => InstructionType::SER{ vx, vy },
            (0x6, vx, n1, n0) => InstructionType::LDV{ vx, value: utils::concat_nib_to_u8(n1, n0) },
            (0x7, vx, n1, n0) => InstructionType::ADDV{ vx, value: utils::concat_nib_to_u8(n1, n0)},
            (0x8, vx, vy, 0x0) => InstructionType::LDR{ vx, vy },
            (0x8, vx, vy, 0x1) => InstructionType::OR{ vx, vy },
            (0x8, vx, vy, 0x2) => InstructionType::AND{ vx, vy },
            (0x8, vx, vy, 0x3) => InstructionType::XOR{ vx, vy },
            (0x8, vx, vy, 0x4) => InstructionType::ADD{ vx, vy },
            (0x8, vx, vy, 0x5) => InstructionType::SUB{ vx, vy },
            (0x8, vx, vy, 0x6) => InstructionType::SHR{ vx, vy },
            (0x8, vx, vy, 0x7) => InstructionType::SUBN{ vx, vy },
            (0x8, vx, vy, 0xe) => InstructionType::SHL{ vx, vy },
            (0x9, vx, vy, 0x0) => InstructionType::SNER{ vx, vy },
            (0xA, n2, n1, n0) => InstructionType::LDI{ value: utils::concat_nib_to_u16(0, n2, n1, n0) },
            (0xC, vx, n1, n0) => InstructionType::RND{ vx, value: utils::concat_nib_to_u8(n1, n0)},
            (0xD, vx, vy, bytes) => InstructionType::DRW{ vx, vy, bytes },
            (0xE, vx, 0xA, 0x1) => InstructionType::SKNP{ vx },
            (0xE, vx, 0x9, 0xE) => InstructionType::SKP{ vx },
            (0xF, vx, 0x0, 0x7) => InstructionType::LDDT{ vx },
            (0xF, vx, 0x0, 0xA) => InstructionType::HALTKP { vx },
            (0xF, vx, 0x1, 0x5) => InstructionType::SETDT{ vx },
            (0xF, vx, 0x1, 0x8) => InstructionType::LDST{ vx },
            (0xF, vx, 0x1, 0xE) => InstructionType::ADDI{ vx },
            (0xF, vx, 0x2, 0x9) => InstructionType::LDHEX{ vx },
            (0xF, vx, 0x3, 0x3) => InstructionType::LDBCD{ vx },
            (0xF, vx, 0x5, 0x5) => InstructionType::LDVXI{ vx },
            (0xF, vx, 0x6, 0x5) => InstructionType::LDRI{ vx },
            _ => {
                let d = nibbles_to_tuple(data);
                println!("Unknown instruction: {:#x} {:#x} {:#x} {:#x}", d.0, d.1, d.2, d.3);
                // std::io::stdin().read_line(&mut String::new());
                InstructionType::UNKNOWN
            }
        }
    }

    pub fn execute_instruction(&mut self, instruction: &InstructionType) {
        match instruction {
            InstructionType::CLS => self.cls(),
            InstructionType::RET => self.ret(),
            InstructionType::JP{ addr} => self.jmp(*addr),
            InstructionType::CALL{ addr } => self.call(*addr),
            InstructionType::SKEQV{ vx, value } => self.skpeqv(*vx, *value),
            InstructionType::SER{ vx, vy} => self.ser(*vx, *vy),
            InstructionType::SKNEQV{ vx, value } => self.skpneqv(*vx, *value),
            InstructionType::LDV{ vx, value } => self.ldv(*vx, *value),
            InstructionType::ADDV{ vx, value } => self.addv(*vx, *value),
            InstructionType::ADD{ vx, vy } => self.add(*vx, *vy),
            InstructionType::SUB{ vx, vy } => self.sub(*vx, *vy),
            InstructionType::SHR{ vx, vy } => self.shr(*vx, *vy),
            InstructionType::SUBN{ vx, vy } => self.subn(*vx, *vy),
            InstructionType::SHL{ vx, vy } => self.shl(*vx, *vy),
            InstructionType::LDR{ vx, vy } => self.ldr(*vx, *vy),
            InstructionType::OR{ vx, vy } => self.or(*vx, *vy),
            InstructionType::AND{ vx, vy } => self.and(*vx, *vy),
            InstructionType::XOR{ vx, vy } => self.xor(*vx, *vy),
            InstructionType::SNER{ vx, vy} => self.sner(*vx, *vy),
            InstructionType::LDI{ value } => self.ldi(*value),
            InstructionType::RND{ vx, value} => self.rnd(*vx, *value),
            InstructionType::DRW{ vx, vy, bytes} => self.drw(*vx, *vy, *bytes),
            InstructionType::SKNP{ vx} => self.sknp(*vx),
            InstructionType::SKP{ vx} => self.skp(*vx),
            InstructionType::HALTKP{ vx} => self.haltkp(*vx),
            InstructionType::LDDT{ vx} => self.lddt(*vx),
            InstructionType::SETDT{ vx } => self.set_dt(*vx),
            InstructionType::LDST{ vx } => self.ldst(*vx),
            InstructionType::ADDI{ vx } => self.addi(*vx),
            InstructionType::LDHEX{ vx } => self.ldhex(*vx),
            InstructionType::LDBCD{ vx } => self.ldbcd(*vx),
            InstructionType::LDVXI{ vx } => self.ldvxi(*vx),
            InstructionType::LDRI{ vx } => self.ldri(*vx),
            _ => ()
        }
    }

    pub fn get_vx_i(vx: u8) -> usize {
        // Only registers V0 to VF are valid
        assert!(vx <= 0xF);
        ((Register::V0 as u8) + vx) as usize
    }

    pub fn jmp(&mut self, address: u16) {
        self.registers[Register::PC as usize] = address;
    }

    pub fn cls(&mut self) {
        self.framebuffer.iter_mut().for_each(|e| *e = 0);
    }

    pub fn ldv(&mut self, vx: u8, value: u8) {
        // log_debug!("Loading to register {}", InternalState::get_vx_i(vx));
        self.registers[Chip8::get_vx_i(vx)] = value as u16;
    }

    pub fn addv(&mut self, vx: u8, value: u8) {
        let sum: u16 = self.registers[Chip8::get_vx_i(vx)] + value as u16;
        self.registers[Chip8::get_vx_i(vx)] = sum & 0xFF;
    }

    pub fn ldi(&mut self, value: u16) {
        self.registers[Register::I as usize] = value;
    }

    pub fn drw(&mut self, vx: u8, vy: u8, bytes: u8) {
        // log_debug!("Drawing from {} to {} for {} bytes", vx, vy, bytes);

        let x_coord = self.registers[Chip8::get_vx_i(vx)] % (DISPLAYW as u16);
        let y_coord = self.registers[Chip8::get_vx_i(vy)] % (DISPLAYH as u16);

        // Set VF initially to 0. If any pixel drawn clears a pixel that was previously
        // white, VF will be set to 1.
        self.set_register(Register::VF, 0);
        for i in 0..bytes {
            let i_value = self.registers[Register::I as usize] + i as u16;
            let data = self.main_memory[i_value as usize];
            // log_debug!("vx {} vy {} bytes {} I {:#x} data {:#x}", x_coord, y_coord, bytes, i_value, data);

            let circ_y_coord = (y_coord + i as u16) % DISPLAYH as u16;

            for j in 0..8 {
                let circ_x_coord = (x_coord + j as u16) % DISPLAYW as u16;

                let fb_index = Chip8::get_fb_i_from_coord_in_fb(circ_x_coord, circ_y_coord);
                let color = match utils::get_nth_bit_u16(data as u16, (7 - j) as u8) {
                    0x0 => Color::Black as u8,
                    0x1 => Color::White as u8,
                    _ => panic!("Invalid sprite pixel value {}", data)
                };

                let previous_color = self.framebuffer[fb_index];
                self.framebuffer[fb_index] ^= color;
                if previous_color == Color::White as u8 && self.framebuffer[fb_index] == Color::Black as u8{
                    self.set_register(Register::VF, 1);
                }
            }
        }
    }

    pub fn call(&mut self, addr: u16) {
        self.push_stack(self.get_register(Register::PC));
        self.set_register(Register::PC, addr);
    }

    pub fn ret(&mut self) {
        let ret_addr = self.pop_stack();
        self.set_register(Register::PC, ret_addr);
    }

    pub fn skpeqv(&mut self, vx: u8, value: u8) {
        let vx_value = self.registers[Chip8::get_vx_i(vx)];
        if value == (vx_value as u8) {
            self.advance_pc();
        }
    }

    pub fn skpneqv(&mut self, vx: u8, value: u8) {
        let vx_value = self.registers[Chip8::get_vx_i(vx)];
        if value != (vx_value as u8) {
            self.advance_pc();
        }
    }

    pub fn rnd(&mut self, vx: u8, value: u8) {
        let rndm: u8 = random();
        let rndm = rndm & value;
        self.registers[Chip8::get_vx_i(vx)] = rndm as u16;
    }

    pub fn addi(&mut self, vx: u8) {
        let vx_value = self.registers[Chip8::get_vx_i(vx)] as u8;
        let vi_value = self.get_register(Register::I);
        let sum = vx_value as u16 + vi_value;
        self.set_register(Register::I, sum);
    }

    pub fn set_dt(&mut self, vx: u8) {
        self.set_register(Register::DT, self.registers[Chip8::get_vx_i(vx) & 0xFF]);
    }

    /// handler_timer will decrease DT and ST by 1 at 60hz (1 for every 16 ms that
    /// passed, to be precise)
    pub fn handle_timer(&mut self, elapsed: &Duration) {
        // If DT == 0, timer is disabled
        if self.get_register(Register::DT) == 0 { 
            self.timer_accumulator = Duration::new(0, 0);
            return;
        }

        self.timer_accumulator += *elapsed;

        // TODO: This is the lazy way of doing this
        // Just to something like dt -= (acummulator / 16)
        while self.timer_accumulator.as_millis() > 16 {
            let current_dt = self.get_register(Register::DT);
            if current_dt > 0 {
                self.set_register(Register::DT, current_dt - 1);
                self.timer_accumulator = self.timer_accumulator - Duration::from_millis(16);
            }
            else {
                self.timer_accumulator = Duration::new(0, 0);
                break;
            }
        }
    }

    pub fn haltkp(&mut self, vx: u8) {
        self.halted_for_keypress = true;
        self.halted_keypress_store_reg = Chip8::get_vx_i(vx);
    }

    pub fn lddt(&mut self, vx: u8) {
        self.registers[Chip8::get_vx_i(vx)] = self.get_register(Register::DT) & 0xFF;
    }

    pub fn ldbcd(&mut self, vx: u8) {
        let value = self.registers[Chip8::get_vx_i(vx)] as u8;
        let hundreds = value / 100;
        let value = value - (hundreds * 100);
        let tens = value / 10;
        let ones = value - (tens * 10);

        let i_value = self.get_register(Register::I) as usize;
        self.main_memory[i_value] = hundreds;
        self.main_memory[i_value + 1] = tens;
        self.main_memory[i_value + 2] = ones;
    }

    pub fn ldri(&mut self, vx: u8) {
        let i_value = self.get_register(Register::I);
        for offset in 0..=vx {
            self.registers[Chip8::get_vx_i(offset)] = self.main_memory[(i_value as usize) + (offset as usize)] as u16;
        }
    }

    pub fn ldr(&mut self, vx: u8, vy: u8) {
        self.registers[Chip8::get_vx_i(vx)] = self.registers[Chip8::get_vx_i(vy)];
    }

    pub fn ser(&mut self, vx: u8, vy: u8) {
        if self.registers[Chip8::get_vx_i(vx)] == self.registers[Chip8::get_vx_i(vy)] {
            self.advance_pc();
        }
    }

    pub fn sner(&mut self, vx: u8, vy: u8) {
        if self.registers[Chip8::get_vx_i(vx)] != self.registers[Chip8::get_vx_i(vy)] {
            self.advance_pc();
        }
    }

    pub fn ldhex(&mut self, vx: u8) {
        let vx_value: u16 = self.registers[Chip8::get_vx_i(vx)];
        let sprite_location: u16 = vx_value * 5;
        self.set_register(Register::I, sprite_location);
    }

    pub fn or(&mut self, vx: u8, vy: u8) {
        let vx_value = self.registers[Chip8::get_vx_i(vx)];
        let vy_value = self.registers[Chip8::get_vx_i(vy)];
        let result = (vx_value | vy_value) & 0xFF;
        self.registers[Chip8::get_vx_i(vx)] = result & 0xFF;
    }

    pub fn and(&mut self, vx: u8, vy: u8) {
        let vx_value = self.registers[Chip8::get_vx_i(vx)];
        let vy_value = self.registers[Chip8::get_vx_i(vy)];
        let result = (vx_value & vy_value) & 0xFF;
        self.registers[Chip8::get_vx_i(vx)] = result & 0xFF;
    }

    pub fn xor(&mut self, vx: u8, vy: u8) {
        let vx_value = self.registers[Chip8::get_vx_i(vx)];
        let vy_value = self.registers[Chip8::get_vx_i(vy)];
        self.registers[Chip8::get_vx_i(vx)] = (vx_value ^ vy_value) & 0xFF;
    }

    pub fn add(&mut self, vx: u8, vy: u8) {
        let x_value = self.registers[Chip8::get_vx_i(vx)];
        let y_value = self.registers[Chip8::get_vx_i(vy)];
        let result = x_value + y_value;
        
        // Cast removes any 1's after the eight bit
        self.registers[Chip8::get_vx_i(vx)] = result as u8 as u16;
        // Check if carry bit should be on 
        self.set_register(Register::VF, if result > 0xFF { 1 } else { 0 });

    }

    pub fn sub(&mut self, vx: u8, vy: u8) {
        // TODO: Check the behavior of casting an i8 to u8 when the value is negative
        let vx_value = self.registers[Chip8::get_vx_i(vx)] as i16;
        let vy_value = self.registers[Chip8::get_vx_i(vy)] as i16;

        let result = (vx_value - vy_value) as u8;
        self.registers[Chip8::get_vx_i(vx)] = result as u16;
        self.set_register(Register::VF, if vx_value > vy_value { 1 } else { 0 });
    }

    pub fn shr(&mut self, vx: u8, _vy: u8) {
        let vx_value = self.registers[Chip8::get_vx_i(vx)];
        let lest_significant_bit = utils::get_nth_bit_u16(vx_value, 0);
        self.registers[Chip8::get_vx_i(vx)] = (vx_value >> 1) & 0xFF;
        self.set_register(Register::VF, if lest_significant_bit == 1 { 1 } else { 0 });
    }

    pub fn subn(&mut self, vx: u8, vy: u8) {
        let vx_value = self.registers[Chip8::get_vx_i(vx)] as u8 as i16;
        let vy_value = self.registers[Chip8::get_vx_i(vy)] as u8 as i16;
        let result = vy_value - vx_value;
        self.registers[Chip8::get_vx_i(vx)] = (result & 0xFF) as u16;
        self.set_register(Register::VF, if vy_value > vx_value { 1 } else { 0 });
    }

    pub fn shl(&mut self, vx: u8, _vy: u8) {
        let vx_value = self.registers[Chip8::get_vx_i(vx)];
        let most_significant_bit = utils::get_nth_bit_u16(vx_value, 7);
        self.registers[Chip8::get_vx_i(vx)] = (vx_value << 1) & 0xFF;
        self.set_register(Register::VF, if most_significant_bit == 1 { 1 } else { 0 });
    }

    // TODO: Sound not yet implemented
    pub fn ldst(&mut self, _vx: u8) {
    }
 
    // TODO: Input not yet implemented
    pub fn sknp(&mut self, vx: u8) {
        let key = self.registers[Chip8::get_vx_i(vx)];
        if !self.keyboard_state.get_key_state_u8(key as u8) {
            self.advance_pc();
        }
    }
    
    pub fn skp(&mut self, vx: u8) {
        let key = self.registers[Chip8::get_vx_i(vx)];
        if self.keyboard_state.get_key_state_u8(key as u8) {
            self.advance_pc();
        }
    }

    pub fn ldvxi(&mut self, vx: u8) {
        let i_value = self.get_register(Register::I);
        for i in 0..(vx + 1) {
            let vx_value = self.registers[Chip8::get_vx_i(i as u8)] as u8;
            self.main_memory[(i_value + i as u16) as usize] = vx_value; 
        }
        self.set_register(Register::I, (i_value + vx as u16 + 1).into());
    }
}
