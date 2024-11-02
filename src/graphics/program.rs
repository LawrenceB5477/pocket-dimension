use std::ffi::CString;
use std::path::{Path, PathBuf};

use glfw::Error;

pub use crate::graphics::shader::{Shader, ShaderType};
use crate::registry::shader_registry::ShaderId;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ShaderProgram {
    id: u32,
    vertex_shader: Shader,
    fragment_shader: Shader,
}

pub enum Uniform {
    Matrix4f(String, glam::Mat4),
    Vec4(String, glam::Vec4),
    Int(String, i32),
    Float(String, f32),
}

impl ShaderProgram {
    pub fn new(shader_folder_path: impl AsRef<Path>, vertex_shader_id: ShaderId, fragment_shader_id: ShaderId) -> Self {
        let folder_path = shader_folder_path.as_ref();
        if !folder_path.exists() {
            panic!("Shader folder not found: {:?}", folder_path);
        }

        let program_id = unsafe {
            gl::CreateProgram()
        };

        let vertex_shader = Shader::new(ShaderType::Vertex, Self::resolve_shader_path(folder_path, vertex_shader_id));
        let fragment_shader = Shader::new(ShaderType::Fragment, Self::resolve_shader_path(folder_path, fragment_shader_id));

        Self {
            id: program_id,
            vertex_shader,
            fragment_shader,
        }
    }

    fn resolve_shader_path(shader_folder_path: impl AsRef<Path>, shader_id: ShaderId) -> PathBuf {
        let mut folder_path = shader_folder_path.as_ref().to_path_buf();
        folder_path.push(shader_id.get_path());
        println!("Resolved shader path: {:?}", folder_path);
        return folder_path;
    }

    pub fn build(&self) -> Result<(), Error> {
        // Attach vertex shader
        self.vertex_shader.compile()?;
        unsafe {
            gl::AttachShader(self.id, self.vertex_shader.id);
        }

        // Attach fragment shader
        self.fragment_shader.compile()?;
        unsafe {
            gl::AttachShader(self.id, self.fragment_shader.id);
        }

        // Link program
        unsafe {
            gl::LinkProgram(self.id);
        }

        let mut link_status = 0;
        unsafe {
            gl::GetProgramiv(self.id, gl::LINK_STATUS, &mut link_status);
        }

        if link_status == 0 {
            unsafe {
                let mut log_length: i32 = 0;
                gl::GetProgramiv(self.id, gl::INFO_LOG_LENGTH, &mut log_length);

                if log_length > 0 {
                    let mut log: Vec<u8> = Vec::with_capacity(log_length as usize);
                    gl::GetProgramInfoLog(self.id,
                                          log_length,
                                          std::ptr::null_mut(),
                                          log.as_mut_ptr().cast());
                    log.set_len(log_length as usize);
                    let error_log = String::from_utf8_lossy(&log);
                    panic!("Program Link Error: {}", error_log);
                }
            }
            panic!("Program Link Error");
        }

        // Clean up shaders
        unsafe {
            gl::DetachShader(self.id, self.vertex_shader.id);
            gl::DetachShader(self.id, self.fragment_shader.id);
        }
        self.vertex_shader.delete();
        self.fragment_shader.delete();
        Ok(())
    }

    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn set_uniform_matrix_4f(&self, name: &str, matrix: &glam::Mat4) {
        let location = self.check_uniform(name);
        unsafe {
            gl::UniformMatrix4fv(location, 1, gl::FALSE, matrix.to_cols_array().as_ptr());
        }
    }

    pub fn set_uniform_vec4(&self, name: &str, vec4: &glam::Vec4) {
        let location = self.check_uniform(name);
        unsafe {
            gl::Uniform4fv(location, 1, vec4.as_ref().as_ptr());
        }
    }

    pub fn set_uniform_int(&self, name: &str, value: i32) {
        let location = self.check_uniform(name);
        unsafe {
            gl::Uniform1i(location, value);
        }
    }

    pub fn set_uniform_float(&self, name: &str, value: f32) {
        let location = self.check_uniform(name);
        unsafe {
            gl::Uniform1f(location, value);
        }
    }


    fn check_uniform(&self, name: &str) -> i32 {
        let location = unsafe {
            gl::GetUniformLocation(self.id, CString::new(name).unwrap().as_ptr())
        };

        if location == -1 {
            panic!("Uniform not found: {}", name);
        }

        return location;
    }
}