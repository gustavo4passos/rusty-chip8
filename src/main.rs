// So the linter will stop screaming at me
#![allow(dead_code)]

mod rom;
mod runtime;
mod state;
mod utils;
mod exec;
mod display;
mod logger;
mod window;
mod debug;
mod input;

use std::env;
use crate::state::InternalState;
// use crate::window::Window;

fn main() {
    // Window::new(300, 300).run();
    let cwd_p = env::current_dir();
    let mut cwd: String = String::from("");
    
    if let Ok(result) = cwd_p {
        if let Some(s) = result.to_str() {
            cwd = String::from(s);
        }
    } else {
        println!("Unable to retrieve current working directory");
    };
    
    println!("Current dir is: {}", cwd);
    
    let data = utils::read_file_to_u8("C:/dev/chip8-roms/pong_1_player.ch8")
        .unwrap();
    let mut main_state: InternalState = InternalState::new();
    main_state.load_rom_to_memory(&data);
    main_state.run();
}
