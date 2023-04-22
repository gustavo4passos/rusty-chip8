extern crate glfw;
use gl;
use glfw::ffi::glfwSetFramebufferSizeCallback;
use glfw::{Action, Context, Key, WindowEvent };
use crate::input::{InputBackend, KeyboardState};
use crate::renderer::Renderer;
use crate::state::KeyboardKey;
use std::sync::mpsc::{ Receiver };
use crate::opengl::{self, OpenGLRenderer };

pub struct Window {
    width: u32,
    height: u32,

    glfw: glfw::Glfw,
    window: glfw::Window,
    events: Receiver<(f64, WindowEvent)>,
    opengl_renderer: opengl::OpenGLRenderer
}

impl Window {
    pub fn new(width: u32, height: u32) -> Window {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersion(4, 5));
        let (mut window, events) = glfw
            .create_window(width, height, "Chip-8", glfw::WindowMode::Windowed)
            .expect("Failed do create window.");

        window.set_key_polling(true);
        window.set_drag_and_drop_polling(true);
        window.set_framebuffer_size_polling(true);
        window.make_current();
    
        Window {
            width,
            height,
            glfw,
            window,
            events,
            opengl_renderer: OpenGLRenderer::new()
        }
    }

    pub fn init(&mut self) {
        gl::load_with(|s| self.glfw.get_proc_address_raw(s));
        opengl::OpenGLRenderer::load_procs(|s: &str| self.glfw.get_proc_address_raw(s));
        self.opengl_renderer.init();
        self.opengl_renderer.set_clear_color(0.2, 0.2, 0.2, 1.0);
        self.window.make_current();

    }

    pub fn run(&mut self) {
        
    }

    pub fn update(&mut self) {
        self.window.swap_buffers()
    }

    pub fn draw(&mut self, fb: &[u8]) {
        self.opengl_renderer.draw_screen(fb);
    }

    pub fn should_close(&mut self) -> bool {
        self.window.should_close()
    }

    pub fn set_key_state(keyboard_state: &mut KeyboardState, key: KeyboardKey, action: Action) {
        if action == Action::Press {
            keyboard_state.set_key_state(key, true);
        }
        else if action == Action::Release {
            keyboard_state.set_key_state(key, false);
        }
    }
}

impl InputBackend for Window {
    fn get_keyboard_state(&mut self, keyboard_state: &mut KeyboardState) {
        self.glfw.poll_events();
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                glfw::WindowEvent::Key(key, _, action, _) => {
                    match key {
                        Key::R => { Window::set_key_state(keyboard_state, KeyboardKey::One,   action); },
                        Key::T => { Window::set_key_state(keyboard_state, KeyboardKey::Two,   action); },
                        Key::Y => { Window::set_key_state(keyboard_state, KeyboardKey::Three, action); },
                        Key::F => { Window::set_key_state(keyboard_state, KeyboardKey::Four,  action); },
                        Key::G => { Window::set_key_state(keyboard_state, KeyboardKey::Five,  action); },
                        Key::H => { Window::set_key_state(keyboard_state, KeyboardKey::Six,   action); },
                        Key::V => { Window::set_key_state(keyboard_state, KeyboardKey::Seven, action); },
                        Key::B => { Window::set_key_state(keyboard_state, KeyboardKey::Eight, action); },
                        Key::N => { Window::set_key_state(keyboard_state, KeyboardKey::Nine,  action); },
                        Key::M => { Window::set_key_state(keyboard_state, KeyboardKey::E,     action); },
                        Key::U => { Window::set_key_state(keyboard_state, KeyboardKey::C,     action); },
                        Key::J => { Window::set_key_state(keyboard_state, KeyboardKey::D,     action); },
                        Key::W => { Window::set_key_state(keyboard_state, KeyboardKey::A,     action); },
                        Key::E => { Window::set_key_state(keyboard_state, KeyboardKey::Zero,  action); },
                        Key::S => { Window::set_key_state(keyboard_state, KeyboardKey::B,     action); },
                        Key::D => { Window::set_key_state(keyboard_state, KeyboardKey::F,     action); },
                        Key::Escape => { self.window.set_should_close(true); }
                        _ => { }
                    }
                },
                glfw::WindowEvent::FileDrop(path) => {
                    for p in path {
                        println!("p: {}", p.as_path().extension().expect("File has no extension").to_str().expect("File has no extension"));
                    }
                },
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    unsafe{ gl::Viewport(0, 0, width, height); }
                }
                _ => { }
            }
        }
    }
}

