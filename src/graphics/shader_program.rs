use std::slice;
use gl::types::{GLchar, GLsizei, GLuint};
use glfw::Error;

const VERT_SHADER: &str = "#version 330 core
    layout (location = 0) in vec3 position;

    void main()
    {
        gl_Position = vec4(position, 1.0);
        // gl_Position = vec4(position.xyz, 1.0);
        // gl_Position = vec4(position.x, position.y, position.z, 1.0);
    }";


const FRAG_SHADER: &str = "#version 330 core
    out vec4 Color;
    void main()
    {
        Color = vec4(0.9, 0.5, 0.2, 1.0);
    }";

pub enum VertexShaderId {
    Basic,
}

pub enum FragmentShaderId {
    Basic,
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

pub struct VertexShader {
    id: GLuint,
    source: String,
}

impl VertexShader {
    pub fn new(name: VertexShaderId) -> Self {
        let shader_id = unsafe {
            gl::CreateShader(gl::VERTEX_SHADER)
        };

        match name {
            VertexShaderId::Basic => {
                Self {
                    id: shader_id,
                    source: VERT_SHADER.to_string(),
                }
            }
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

struct FragmentShader {
    id: u32,
    source: String,
}

impl FragmentShader {
    pub fn new(name: FragmentShaderId) -> Self {
        let shader_id = unsafe {
            gl::CreateShader(gl::FRAGMENT_SHADER)
        };
        match name {
            FragmentShaderId::Basic => {
                Self {
                    id: shader_id,
                    source: FRAG_SHADER.to_string(),
                }
            }
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


pub struct ShaderProgram {
    id: u32,
    vertex_shader: VertexShader,
    fragment_shader: FragmentShader
}

impl ShaderProgram {
    pub fn new(vertex_shader_id: VertexShaderId, fragment_shader_id: FragmentShaderId) -> Self {

        let program_id = unsafe {
            gl::CreateProgram()
        };

        let vertex_shader = VertexShader::new(vertex_shader_id);
        let fragment_shader = FragmentShader::new(fragment_shader_id);

        Self {
            id: program_id,
            vertex_shader,
            fragment_shader
        }
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
}