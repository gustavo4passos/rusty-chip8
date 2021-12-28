use std::io::Write;

use crate::state::{DISPLAYW, DISPLAYH, Color};

pub fn draw_console(framebuffer: &[u8; (DISPLAYW * DISPLAYH) as usize]) {
    // println!("-------------------------------");
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    for i in (0..(DISPLAYW * DISPLAYH)).rev() {
        if framebuffer[(DISPLAYW * DISPLAYH - 1 - i) as usize] == Color::White as u8 {
            print!("█");
        } else {
            print!(" ");
        }
        
        if i % DISPLAYW == 0 {
            print!("\n");
        }
    }
    std::io::stdout().flush().expect("Unable to flush stdout.");

    // for i in (0..DISPLAYH).rev() {
    //     print!("\u{033}");
    // }
    // println!("-------------------------------");
}