extern crate glfw;
use glfw::{Action, Context, Key};

pub struct Window {
    width: u32,
    height: u32,

    glfw: glfw::Glfw,
    window: glfw::Window,
}

impl Window {
    pub fn new(width: u32, height: u32) -> Window {
        let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        let (mut window, events) = glfw
            .create_window(width, height, "Hello Glfw", glfw::WindowMode::Windowed)
            .expect("Failed do create window.");

        window.set_key_polling(true);
        window.make_current();

        Window {
            width,
            height,
            glfw,
            window,
        }
    }

    pub fn init() {}

    pub fn run(&mut self) {
        while !self.window.should_close() {
            self.glfw.poll_events();
        }
    }
}
