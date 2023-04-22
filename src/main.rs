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
mod renderer;
mod opengl;
mod shader;

use std::env;
use crate::state::InternalState;

fn main() {
    let cwd_p = env::current_dir();
    let mut cwd: String = String::from("");
    
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Error: Rom file name missing.");
        return;
    }

    let file_name = &args[1];

    if let Ok(result) = cwd_p {
        if let Some(s) = result.to_str() {
            cwd = String::from(s);
        }
    } else {
        println!("Unable to retrieve current working directory");
    };
    
    println!("Current dir is: {}", cwd);
    
    let data = match utils::read_file_to_u8(file_name) {
        Ok(file_content) => file_content,
        Err(_) => {
            println!("Error: Unable to open file: {}", file_name);
            return;
        }
    };
    
    let mut main_state: InternalState = InternalState::new();
    main_state.load_rom_to_memory(&data);
    main_state.run();
}
