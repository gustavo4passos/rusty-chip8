use std::fs;
use std::io;

pub fn read_file_to_u8(file_path: &str) -> io::Result<Vec<u8>> {
    println!("Reading file: {}", file_path);
    fs::read(file_path)
}

pub fn concat_u8_to_u16(l1: u8, l0: u8) -> u16 {
    ((l1 as u16) << 8) | (l0 as u16)
}

pub fn concat_nib_to_u16(n3: u8, n2: u8, n1: u8, n0: u8) -> u16 {
    ((n3 & 0xF) as u16) << 12 |
    ((n2 & 0xF) as u16) << 8  |
    ((n1 & 0xF) as u16) << 4  |
    (n0 & 0xF) as u16
}

pub fn concat_nib_to_u8(n1: u8, n0: u8) -> u8 {
    ((n1 & 0xF)) << 4  |
    (n0 & 0xF)
}

/// Get bit in pos bit_pos. It should be zero-indexed.
pub fn get_nth_bit_u16(data: u16, bit_pos: u8) -> u8 {
    ((data & ((1 << bit_pos) as u16)) >> bit_pos) as u8
}

pub fn get_nth_nibble(data: u16, nibble_pos: u8) -> u8 {
    const NIBBLES_IN_WORD: u8 = 4;
    assert!(nibble_pos < 4);
    ((data << ((NIBBLES_IN_WORD - nibble_pos - 1) * 4)) >> 12) as u8
}

pub fn get_nth_byte(data: u16, byte_pos: u8) -> u8 {
    const BYTES_IN_WORD: u8 = 2;
    assert!(byte_pos < 2);
    ((data << ((BYTES_IN_WORD - byte_pos - 1) * 8)) >> 8) as u8
}

pub fn nibbles_to_tuple(data: u16) -> (u8, u8, u8, u8) {
    (
        get_nth_nibble(data, 3),
        get_nth_nibble(data, 2),
        get_nth_nibble(data, 1),
        get_nth_nibble(data, 0),
    )
}
