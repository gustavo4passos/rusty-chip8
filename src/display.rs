use crate::state::{DISPLAYW, DISPLAYH, Color};

pub fn draw_console(framebuffer: &[u8; (DISPLAYW * DISPLAYH) as usize]) {
    println!("-------------------------------");
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

    println!("-------------------------------");
}