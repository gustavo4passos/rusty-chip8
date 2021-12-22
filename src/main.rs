// So the linter will stop screaming at me
#![allow(dead_code)]

mod rom;
mod runtime;
mod state;
mod utils;
mod exec;
mod display;

use std::env;
use crate::state::InternalState;


fn main() {
    let cwd_p = env::current_dir();
    let mut cwd: String = String::from("");
    
    if let Ok(result) = cwd_p {
        if let Some(s) = result.to_str() {
            cwd = String::from(s);
        }
    };
    println!("Current dir is: {}", cwd);
    
    let data = utils::read_file_to_u8("../IBM Logo.ch8")
        .unwrap();
    let mut main_state: InternalState = InternalState::new();
    main_state.load_rom_to_memory(&data);
    main_state.run();
}
