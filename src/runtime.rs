use crate::{chip8::Chip8, utils};
use crate::window::Window;
use crate::input::InputBackend;

pub struct Runtime {
    chip8: Chip8,
    current_rom_path: Option<String>,
    rom_loaded: bool,
    paused: bool
}

impl Runtime {
    pub fn new(rom_path: Option<String>) -> Runtime {
        Runtime {
            chip8: Chip8::new(),
            current_rom_path: rom_path,
            rom_loaded: false,
            paused: false
        }
    }

    pub fn run(&mut self) {
        self.chip8.setup();

        if let Some(file_path) = &self.current_rom_path {
            let file_path_copy = file_path.clone();
            self.load_from_from_file(&file_path_copy);
        }

        self.run_main_loop();
    }

    pub fn load_from_from_file(&mut self, file_path: &str) -> bool {
        let data = utils::read_file_to_u8(&file_path);
        match data {
            Ok(data) => {
                self.chip8.load_rom_to_memory(&data);
                self.rom_loaded = true;
                true
            },
            Err(e) => {
                println!("Error: Unable to road rom file: {}. {}", file_path, e);
                self.rom_loaded = false;
                false
            }
        }
    }

    pub fn run_main_loop(&mut self) {
        let mut w = Window::new(1200, 600);
        w.init();

        loop {
            w.process_input(&mut self.chip8.keyboard_state);
            if w.should_close() { break };
            if w.has_drag_and_drop() {
                // Restart chip8 internal state
                self.chip8 = Chip8::new();
                self.chip8.setup();
                // Load new rom, if possible
                self.rom_loaded = false;
                self.current_rom_path = Some(w.get_drag_and_drop());
                self.load_from_from_file(&w.get_drag_and_drop());
                w.clear_drag_and_drop();
            }

            if self.rom_loaded {
                self.chip8.run_cycles();
            }
            w.draw(&self.chip8.framebuffer);
            w.update();
        }
    }    
}