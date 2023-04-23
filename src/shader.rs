use std::{fs, ffi::{CStr, CString}};

pub struct ShaderProgram {
    vs_path: String,
    fs_path: String,
    id: u32,
    vs_content: String,
    fs_content: String
}

impl ShaderProgram {
    pub fn from_files (vs_file_path: &str, fs_file_path: &str) -> ShaderProgram {
        ShaderProgram { 
            vs_path: String::from(vs_file_path), 
            fs_path: String::from(fs_file_path), 
            id: 0, 
            vs_content: String::new(), 
            fs_content: String::new()
        }
    }

    pub fn from_text(vs_content: &str, fs_content: &str) -> ShaderProgram {
        ShaderProgram { 
            vs_path: String::new(), 
            fs_path: String::new(), 
            id: 0, 
            vs_content: vs_content.to_string(),
            fs_content: fs_content.to_string() 
        }
    }

    pub fn bind(&self) {
        unsafe { gl::UseProgram(self.id); }
    }

    pub fn load_shader_files(&mut self) -> bool {
        if self.vs_path.is_empty() || self.fs_path.is_empty() {
            println!("Unable to load shaders. At least of the shader paths is missing.");
            return false;
        }
        
        self.vs_content = fs::read_to_string(&self.vs_path).unwrap();
        self.fs_content = fs::read_to_string(&self.fs_path).unwrap();

        true
    }

    pub fn load(&mut self) -> bool {
        unsafe {
            const LOG_SIZE: usize = 200;

            let vs_id = gl::CreateShader(gl::VERTEX_SHADER);
            let fs_id = gl::CreateShader(gl::FRAGMENT_SHADER);
            self.id = gl::CreateProgram();

            let vs_cstring = CString::new(self.vs_content.as_bytes()).unwrap();
            let fs_cstring = CString::new(self.fs_content.as_bytes()).unwrap();
            gl::ShaderSource(vs_id, 1, &vs_cstring.as_c_str().as_ptr(), std::ptr::null());
            gl::ShaderSource(fs_id, 1, &fs_cstring.as_c_str().as_ptr(), std::ptr::null());

            gl::CompileShader(vs_id);
            let mut vs_compile_status = 0;
            gl::GetShaderiv(vs_id, gl::COMPILE_STATUS, &mut vs_compile_status);

            if vs_compile_status as u8 != gl::TRUE {

                let mut log: [i8; LOG_SIZE] = [0; LOG_SIZE];
                gl::GetShaderInfoLog(vs_id, LOG_SIZE as i32, std::ptr::null_mut(), log.as_mut_ptr());

                let log_c_str = CStr::from_ptr(log.as_ptr()).to_str().unwrap();
                println!("Unable to compile vertex shader!");
                println!("Shader error log: {}", log_c_str);

                return false;
            } 

            gl::CompileShader(fs_id);
            let mut fs_compile_status = 0;
            gl::GetShaderiv(fs_id, gl::COMPILE_STATUS, &mut fs_compile_status);
            
            if fs_compile_status as u8 != gl::TRUE {
                let mut shader_log: [i8; LOG_SIZE] = [0; LOG_SIZE];
                gl::GetShaderInfoLog(fs_id, LOG_SIZE as i32, std::ptr::null_mut(), shader_log.as_mut_ptr());

                let shader_log_cstr = CStr::from_ptr(shader_log.as_ptr());
                println!("Unable to compile fragment shader!");
                println!("Shader error log: {}", shader_log_cstr.to_str().unwrap());

                return false;
            }

            gl::AttachShader(self.id, vs_id);
            gl::AttachShader(self.id, fs_id);
            gl::LinkProgram(self.id);

            return true;
        }

    }
}