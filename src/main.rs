// So the linter will stop screaming at me
#![allow(dead_code)]

mod rom;
mod runtime;
mod chip8;
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

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut file_name: Option<String> = None;

    if args.len() < 2 {
        println!("No rom file provided. Drag roms to window to load them.");
    }
    else {
        file_name = Some(String::from(&args[1]));
    }

    let mut runtime = runtime::Runtime::new(file_name);
    runtime.run();
}