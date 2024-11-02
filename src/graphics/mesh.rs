use std::collections::HashMap;

use gl::types::GLuint;

use crate::graphics::program::{ShaderProgram, Uniform};
use crate::registry::texture_registry::{TEXTURE_REGISTRY, TextureId};

pub struct Mesh {
    vao_id: GLuint,
    vbo_id: GLuint,
    ebo_id: GLuint,

    vertices: Vec<f32>,
    indices: Vec<u32>,

    // TODO make below easier to work with
    program: ShaderProgram,
    textures: HashMap<u32, TextureId>
}

impl Mesh {

    // TODO exand for more textures
    pub fn new(vbo: Vec<f32>, ebo: Vec<u32>, program: ShaderProgram, texture_id: Option<TextureId>) -> Self {
        let mut vao_id = 0;

        // Create VAO
        unsafe {
            gl::GenVertexArrays(1, &mut vao_id);
            gl::BindVertexArray(vao_id);
        }


        // Create VBO
        let mut vbo_id = 0;
        unsafe {
            // Create vbo
            gl::GenBuffers(1, &mut vbo_id);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo_id);
            gl::BufferData(gl::ARRAY_BUFFER, (vbo.len() * std::mem::size_of::<f32>()) as isize, vbo.as_ptr().cast(), gl::STATIC_DRAW);


            // Create vertex attribute arrays
            let attrib_index = 0;
            // attribute index, number of components per attribute, type, is normalized, size of stride, offset
            gl::VertexAttribPointer(attrib_index, 3, gl::FLOAT, gl::FALSE, (5 * std::mem::size_of::<f32>()) as i32, 0 as *const _);
            gl::EnableVertexAttribArray(attrib_index);
        }


        // Create EBO
        let mut ebo_id = 0;
        unsafe {
            gl::GenBuffers(1, &mut ebo_id);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo_id);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (ebo.len() * std::mem::size_of::<u32>()) as isize, ebo.as_ptr().cast(), gl::STATIC_DRAW);
        }


        let mut texture_unit_to_texture_id = HashMap::new();

        // Create texture if needed!
        match texture_id {
            Some(textureId) => {
                // Configure attribute for texture coordinates
                unsafe {
                    // attribute index, number of components per attribute, type, is normalized, size of stride, offset
                    gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, (5 * std::mem::size_of::<f32>()) as i32, (3 * std::mem::size_of::<f32>()) as *const _);
                    gl::EnableVertexAttribArray(1);
                    texture_unit_to_texture_id.insert(0, textureId);
                }
            }
            _ => {}
        };


        unsafe {
            // Unbind vao
            gl::BindVertexArray(0);
            // Unbind buffers
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }


        Self {
            vao_id,
            vbo_id: vbo_id,
            ebo_id: ebo_id,
            vertices: vbo,
            indices: ebo,
            program,
            textures: texture_unit_to_texture_id
        }
    }

    pub fn draw(&self, uniforms: &[Uniform]) {
        unsafe {

            // Bind textures to texture units
            for (texture_unit, texture_id) in &self.textures {
                TEXTURE_REGISTRY.read()
                    .expect("Texture registry lock poisoned")
                    .get(texture_id)
                    .expect("Texture not found")
                    .bind(*texture_unit);
            }

            // Bind program and VAO
            self.program.use_program();
            gl::BindVertexArray(self.vao_id);


            // Set uniforms
            for uniform in uniforms {
                match uniform {
                    Uniform::Matrix4f(name, matrix) => {
                        self.program.set_uniform_matrix_4f(name, matrix);
                    }
                    Uniform::Vec4(name, vec4) => {
                        self.program.set_uniform_vec4(name, vec4);
                    }
                    Uniform::Int(name, value) => {
                        self.program.set_uniform_int(name, *value);
                    },
                    Uniform::Float(name, value) => {
                        self.program.set_uniform_float(name, *value);
                    }
                }
            }


            // Draw mesh
            gl::DrawElements(gl::TRIANGLES, self.indices.len() as i32, gl::UNSIGNED_INT, std::ptr::null());

            // Unbind
            gl::BindVertexArray(0);
            gl::UseProgram(0);
        }
    }
}