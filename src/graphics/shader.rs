use std::path::Path;

use gl::types::GLuint;
use glfw::Error;

use crate::assets::shader_loader;

pub enum ShaderType {
    Vertex,
    Fragment,
}


fn compile_shader(id: GLuint, source: &str) -> Result<(), Error> {
    unsafe {
        gl::ShaderSource(id, 1, &source.as_bytes().as_ptr().cast(), &source.len().try_into().unwrap());
        gl::CompileShader(id);
        let mut success = 0;

        // integer vector (or address)
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut log_length: i32 = 0;
            gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut log_length);

            if log_length > 0 {
                let mut log: Vec<u8> = Vec::with_capacity(log_length as usize);
                gl::GetShaderInfoLog(id,
                                     log_length,
                                     std::ptr::null_mut(),
                                     log.as_mut_ptr().cast());

                log.set_len(log_length as usize);
                let error_log = String::from_utf8_lossy(&log);
                panic!("Vertex Shader Compile Error: {}", error_log);
            }
            panic!("Vertex Shader Compile Error: Unknown");
        }
    }

    Ok(())
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Shader {
    pub id: GLuint,
    pub source: String,
}

impl Shader {
    pub fn new(shader_type: ShaderType, shader_path: impl AsRef<Path> ) -> Self {
        let id= unsafe {
            match shader_type {
                ShaderType::Fragment => {
                    gl::CreateShader(gl::FRAGMENT_SHADER)
                },
                ShaderType::Vertex => {
                    gl::CreateShader(gl::VERTEX_SHADER)
                }
            }
        };


        Self {
            id: id,
            source: shader_loader::load_shader_source(shader_path),
        }
    }

    pub fn compile(&self) -> Result<(), Error> {
        compile_shader(self.id, &self.source)
    }

    pub fn delete(&self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }

}

