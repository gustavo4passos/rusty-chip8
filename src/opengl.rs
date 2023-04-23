use std::mem::{self};
use std::os::raw::c_void;
use gl;
use crate::renderer::Renderer;
use crate::shader::{self, ShaderProgram};
use crate::chip8;

pub struct Texture {
    width: i32,
    height: i32,
    id: u32,
    data: Vec<u8>
}

pub struct OpenGLRenderer {
    vbo: u32,
    vao: u32,
    screen_shader: shader::ShaderProgram,
    screen_framebuffer: [f32; (chip8::DISPLAYW * chip8::DISPLAYH) as usize],
    screen_texture: Texture
}


impl OpenGLRenderer {
    pub fn new() -> OpenGLRenderer {

        let vertex_shader = String::from("
            #version 450 core
            layout(location = 0) in vec2 iCoord;
            out vec2 fScreenCoordN;

            void main() {
                gl_Position = vec4(iCoord, 0, 1.0);
                fScreenCoordN = iCoord * 0.5 + 0.5;
            }
        ");

        let fragment_shader = String::from("
            #version 450 core 
            in vec2 fScreenCoordN;
            out vec4 oColor;
            uniform sampler2D screenTex;
            
            void main() {
                // vec3 black = vec3(0x0A / 255.0, 0x4D / 255.0, 0x68 / 255.0);
                // vec3 white = vec3(0x00 / 255.0, 0xFF / 255.0, 0xCA / 255.0);
                vec3 black = vec3(0x39 / 255.0, 0x36 / 255.0, 0x46 / 255.0);
                vec3 white = vec3(0xF4 / 255.0, 0xEE / 255.0, 0xE0 / 255.0);
                vec2 invertedCoords = vec2(fScreenCoordN.x, 1 - fScreenCoordN.y);
                float bufferColor = texture(screenTex, invertedCoords).r;
                
                vec3 finalColor = (1 - bufferColor) * black + bufferColor * white;
                oColor = vec4(vec3(finalColor), 1.0);
            }
        ");

        OpenGLRenderer {
            vbo: 0,
            vao: 0,
            screen_framebuffer: [0.5; (chip8::DISPLAYW * chip8::DISPLAYH) as usize],
            screen_shader: ShaderProgram::from_text(&vertex_shader, &fragment_shader),
            screen_texture: Texture::new(chip8::DISPLAYW as i32, chip8::DISPLAYH as i32)
        }
    }

    pub fn load_procs<F>(mut f: F) where
        F: FnMut(&'static str) -> *const c_void  {
        gl::load_with(|s: &str| f(s))
    }

    pub fn interpolate_to_fb(&mut self, src: &[u8]) {
        for (i, data) in src.iter().enumerate() {
            self.screen_framebuffer[i] = if *data == 255 { 1.0 } else {
                self.screen_framebuffer[i] * 0.8 + (*data as f32 / 255.0) * 0.2
            }
        }
    }
}

impl Texture {
    pub fn new(width: i32, height: i32) -> Texture {
        Texture {
            width,
            height,
            id: 0,
            data: Vec::new()
        }
    }

    pub fn init(&mut self) {
        unsafe {
            gl::GenTextures(1, &mut self.id);
            gl::BindTexture(gl::TEXTURE_2D, self.id);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::MIRRORED_REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::MIRRORED_REPEAT as i32);
            
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RED as i32, self.width, self.height, 
                0, gl::RED, gl::FLOAT, std::ptr::null());
        }
    }

    pub fn load(&mut self, data: &[f32]) {
        unsafe {
            self.bind();
            gl::TexSubImage2D(gl::TEXTURE_2D, 0, 0, 0, self.width, self.height, 
                gl::RED, gl::FLOAT, data.as_ptr() as *const c_void);
        }
    }

    pub fn bind(&mut self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }
}

struct Vertex2D {
    x: f32,
    y: f32
}

impl Renderer for OpenGLRenderer {
    fn init(&mut self) {
        unsafe {

            self.screen_shader.load();

            gl::GenBuffers(1, &mut self.vbo); 
            gl::GenVertexArrays(1, &mut self.vao);
            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);

            const SCREEN_QUAD_N_VERTICES: usize = 6;
            let screen_quad_vertices: [Vertex2D; SCREEN_QUAD_N_VERTICES] = [
                Vertex2D{ x: -1.0, y:  1.0 },
                Vertex2D{ x:  1.0, y:  1.0 },
                Vertex2D{ x: -1.0, y: -1.0 },
                Vertex2D{ x:  1.0, y:  1.0 },
                Vertex2D{ x: -1.0, y: -1.0 },
                Vertex2D{ x:  1.0, y: -1.0 },
            ];

            gl::BufferData(gl::ARRAY_BUFFER, mem::size_of::<[Vertex2D; SCREEN_QUAD_N_VERTICES]>() as isize, 
                screen_quad_vertices.as_ptr() as *const c_void, gl::STATIC_DRAW);

                gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
                gl::EnableVertexAttribArray(0);


            self.screen_texture.init();

            while gl::GetError() != gl::NO_ERROR {
                println!("Error!");
                panic!();
            }
        }
    }

    fn clear(&mut self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    fn set_clear_color(&mut self, r: f32, g: f32, b: f32, a: f32) {
        unsafe {
            gl::ClearColor(r, g, b, a);
        }
    }

    fn draw_screen(&mut self, fb: &[u8]) {
        unsafe {
            self.screen_shader.bind();
            self.interpolate_to_fb(fb);
            self.screen_texture.load(&self.screen_framebuffer);
            self.clear();
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
        }
    }

}
