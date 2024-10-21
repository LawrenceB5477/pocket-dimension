use glfw::PWindow;
use crate::graphics::shader_program::{FragmentShaderId, ShaderProgram, VertexShaderId};
use crate::graphics::vao::Vao;

// TODO put program here for now, restructure later
// TODO maybe make stateless?
pub struct Renderer {
    program: Option<ShaderProgram>,
    vao: Option<Vao>
}

// Draws a scene with a camera
impl Renderer {
    pub fn new() -> Renderer {
        Renderer {
            program: None,
            vao: None
        }
    }

    pub fn init(&mut self, window: &mut PWindow) {
        // Load OpenGL functions dynamically
        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        // Set viewport
        unsafe {
            let (width, height) = window.get_framebuffer_size();
            gl::Viewport(0, 0, width, height);
            gl::ClearColor(0.0, 0.0, 0.4, 1.0);
        }

        // Create shader program
        let mut program = ShaderProgram::new(VertexShaderId::Basic, FragmentShaderId::Basic);
        program.build();
        self.program.replace(program);


        // Create VAOs
        let vao = Vao::new();
        self.vao = Some(vao);
    }

    pub fn render(&self) {
        let vao = self.vao.as_ref().unwrap();
        let program = self.program.as_ref().unwrap();

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        program.use_program();
        vao.draw();
    }
}