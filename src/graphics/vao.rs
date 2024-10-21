use gl::types::GLuint;

// Store vao index, maybe store vbo and ebo and attribute indices
// Store raw data
pub struct Vao {
    id: GLuint,
}

impl Vao {

    pub fn new() -> Self {
        // Raw data
        let verts = [
            -0.5f32, -0.5, 0.0,
            0.5, -0.5, 0.0,
            0.0, 0.5, 0.0,
        ];

        let mut id = 0;

        // Create VAO
        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }


        // Create VBO
        let mut vbo = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo);
        }

        unsafe {
            // Bind vao to current context
            gl::BindVertexArray(id);

            // Bind vbo to vao and fill with data
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            // Needs size of buffer in bytes
            // TODO what is static_draw?
            gl::BufferData(gl::ARRAY_BUFFER, std::mem::size_of_val(&verts) as isize, verts.as_ptr().cast(), gl::STATIC_DRAW);


            // Create vertex attribute arrays
            let attrib_index = 0;
            // attribute index, number of components per attribute, type, is normalized, size of stride, offset
            gl::VertexAttribPointer(attrib_index, 3, gl::FLOAT, gl::FALSE, (3 * std::mem::size_of::<f32>()) as i32, 0 as *const _);
            gl::EnableVertexAttribArray(attrib_index);

            // Unbind
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }


        Self {
            id
        }
    }

    pub fn draw(&self) {

        self.bind();
        unsafe {
            // Can also do gl::DrawElements
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
        self.unbind();
    }

    fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }

    fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }
}