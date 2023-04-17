extern crate glfw;
extern crate gl;
use glfw::{Action, Context, Key, WindowEvent };
use crate::input::{InputBackend, KeyboardState};
use crate::state::KeyboardKey;
use std::sync::mpsc::{ Receiver };
use gl::types::*;

pub struct Window {
    width: u32,
    height: u32,

    glfw: glfw::Glfw,
    window: glfw::Window,
    events: Receiver<(f64, WindowEvent)>,
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
            events,
        }
    }

    pub fn init(&mut self) {
        gl::load_with(|s| self.glfw.get_proc_address_raw(s));
        unsafe { gl::ClearColor(1.0, 0.0, 0.0, 1.0) };
    }

    pub fn run(&mut self) {
        
    }

    pub fn update(&mut self) {
        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT) };
        self.window.swap_buffers()
    }

    pub fn should_close(&mut self) -> bool {
        self.window.should_close()
    }
}

impl InputBackend for Window {
    fn get_keyboard_state(&mut self, keyboard_state: &mut KeyboardState) {
        self.glfw.poll_events();
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                glfw::WindowEvent::Key(Key::Up, _, Action::Press, _) => {
                    keyboard_state.set_key_state(KeyboardKey::One, true)
                },
                glfw::WindowEvent::Key(Key::Up, _, Action::Release, _) => {
                    keyboard_state.set_key_state(KeyboardKey::One, false)
                },
                glfw::WindowEvent::Key(Key::Down, _, Action::Press, _) => {
                    keyboard_state.set_key_state(KeyboardKey::Four, true)
                },
                glfw::WindowEvent::Key(Key::Down, _, Action::Release, _) => {
                    keyboard_state.set_key_state(KeyboardKey::Four, false)
                },
                glfw::WindowEvent::Key(Key::Escape, _, Action::Release, _) => {
                    self.window.set_should_close(true)
                },
                _ => { }
            }
        }
    }
}