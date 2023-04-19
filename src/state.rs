use std::time::{ Duration, Instant };
use crate::input::{ KeyboardState };
use crate::utils;

pub const DISPLAYW: u32 = 64;
pub const DISPLAYH: u32 = 32;
pub const MEMSIZE: usize = 0xFFF;
pub const PROGRAM_START: usize = 0x200;
pub const STACK_START: usize = PROGRAM_START;
pub const STACK_SIZE: u32 = 32; // Stack can store 16 u16
pub const CYCLE_DURATION_NS: u32 = 1000;

pub enum KeyboardKey {
    Zero = 0,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    A,
    B,
    C,
    D,
    E,
    F,
    Total
}

pub enum Color {
    White = 0xFF,
    Black = 0x00,
}

#[derive(PartialEq)]
pub enum Register {
    V0 = 0,
    V1,
    V2,
    V3,
    V4,
    V5,
    V6,
    V7,
    V8,
    V9,
    VA,
    VB,
    VC,
    VD,
    VE,
    VF,
    I,
    PC,
    SP,
    DT,
    Sound,
    Total
}

pub struct InternalState {
    pub framebuffer: [u8; (DISPLAYW * DISPLAYH) as usize],
    pub main_memory: [u8; MEMSIZE as usize],
    pub registers: [u16; Register::Total as usize],
    pub keyboard_state: KeyboardState,
    pub previous_tick: Instant,
    pub previous_vsync: Instant,
    pub timer_accumulator: Duration,
    pub time_since_last_op: Duration,
    pub halted_for_keypress: bool,
    pub halted_keypress_store_reg: usize
}

pub const FONTS: [u8; 5 * 16] = [0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
                                0x20, 0x60, 0x20, 0x20, 0x70, // 1
                                0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
                                0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
                                0x90, 0x90, 0xF0, 0x10, 0x10, // 4
                                0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
                                0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
                                0xF0, 0x10, 0x20, 0x40, 0x40, // 7
                                0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
                                0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
                                0xF0, 0x90, 0xF0, 0x90, 0x90, // A
                                0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
                                0xF0, 0x80, 0x80, 0x80, 0xF0, // C
                                0xE0, 0x90, 0x90, 0x90, 0xE0, // D
                                0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
                                0xF0, 0x80, 0xF0, 0x80, 0x80 ]; // F

impl InternalState {
    pub fn new() -> InternalState {
        InternalState {
            framebuffer: [0; (DISPLAYW * DISPLAYH) as usize],
            main_memory: [0; MEMSIZE],
            registers: [0; Register::Total as usize],
            keyboard_state: KeyboardState::new(),

            previous_tick: Instant::now(),
            previous_vsync: Instant::now(),
            timer_accumulator: Duration::new(0, 0),
            time_since_last_op: Duration::new(0, 0),
            halted_for_keypress: false,
            halted_keypress_store_reg: 0
        }
    }

    pub fn get_fb_i_from_coord_in_fb(x: u16, y: u16) -> usize {
        (y as u32 * DISPLAYW + x as u32) as usize
    }

    
    pub fn set_register(&mut self, register: Register, value: u16) {
        assert!(register != Register::Total);
        self.registers[register as usize] = value;
    }
    
    pub fn get_register(&self, register: Register) -> u16 {
        self.registers[register as usize]
    }
    
    pub fn advance_pc(&mut self) -> u16 {
        self.registers[Register::PC as usize] += 2;
        self.registers[Register::PC as usize]
    }
    
    /// Advance SP by n units. Negative values can be used to advance the SP in the 
    /// opposite direction.
    /// The stack is composed of 16 bit values, and since the interpreter
    /// area is used (which is a [u8]), each unit advances the SP by 2.
    pub fn advance_sp(&mut self, n: i32) -> u16 {
        assert!((self.registers[Register::SP as usize] as i32) > (STACK_START as i32 - STACK_SIZE as i32));
        let mut sp_v = self.get_register(Register::SP) as i32;
        sp_v -= n * 2; // Advance n * 2 bytes
        assert!(sp_v >= 0); // Sanity check
        self.set_register(Register::SP, sp_v as u16);
        self.registers[Register::SP as usize]
    }

    pub fn peek_stack(&self) -> u16 {
        let stack_top = self.get_register(Register::SP) as usize;
        self::utils::concat_u8_to_u16(self.main_memory[stack_top - 1], self.main_memory[stack_top])
    }

    pub fn pop_stack(&mut self) -> u16 {
        let top_stack = self.peek_stack();
        self.advance_sp(-1);
        top_stack
    }

    pub fn push_stack(&mut self, value: u16) {
        let stack_addr = self.advance_sp(1) as usize;
        let split = utils::split_u16_to_u8(value);
        self.main_memory[stack_addr] = split.1;
        self.main_memory[stack_addr - 1] = split.0;
    }
}