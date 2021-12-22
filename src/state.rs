
pub const DISPLAYW: u32 = 64;
pub const DISPLAYH: u32 = 32;
pub const MEMSIZE: usize = 0xFFF;
pub const PROGRAM_START: usize = 0x200;
pub enum Keyboard {
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
    Total
}

pub enum Color {
    White = 0xFF,
    Black = 0x00,
}

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
    Sound,
    Delay,
    Total
}

pub struct InternalState {
    pub framebuffer: [u8; (DISPLAYW * DISPLAYH) as usize],
    pub main_memory: [u8; MEMSIZE as usize],
    pub registers: [u16; Register::Total as usize],
    keyboard_state: [bool; Keyboard::Total as usize]
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
            keyboard_state: [false; Keyboard::Total as usize]
        }
    }

    pub fn get_fb_i_from_coord_in_fb(x: u16, y: u16) -> usize {
        (y as u32 * DISPLAYW + x as u32) as usize
    }
}