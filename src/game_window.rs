use glfw::{Context, GlfwReceiver, Key, PWindow, WindowEvent, WindowHint};
use crate::graphics::render::Renderer;

const SCREEN_WIDTH: u32 = 1920;
const SCREEN_HEIGHT: u32 = 1080;

pub struct GameWindow {
    glfw: Option<glfw::Glfw>,
    window: Option<PWindow>,
    events: Option<GlfwReceiver<(f64, WindowEvent)>>,
    renderer: Option<Renderer>
}

impl GameWindow {
    pub fn new() -> GameWindow {
        GameWindow {
            glfw: Some(glfw::init(glfw::fail_on_errors).unwrap()),
            window: None,
            events: None,
            renderer: Some(Renderer::new())
        }
    }

    pub fn create_window(&mut self) {
        // TODO blow up if glfw is None

        // Set window hints
        let glfw = self.glfw.as_mut().unwrap();

        glfw.window_hint(WindowHint::ContextVersion(3, 3));
        glfw.window_hint(WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        glfw.window_hint(WindowHint::OpenGlForwardCompat(true));
        glfw.window_hint(WindowHint::Resizable(true));
        glfw.window_hint(WindowHint::Visible(true));
        glfw.window_hint(WindowHint::Focused(true));

        // Create actual window
        let (window, events) = glfw.create_window(SCREEN_WIDTH, SCREEN_HEIGHT, "Hello, Triangle", glfw::WindowMode::Windowed).unwrap();
        self.window = Some(window);
        self.events = Some(events);

        // Set up polling
        let window = self.window.as_mut().unwrap();
        window.make_current();
        window.set_key_polling(true);
        window.set_cursor_pos_polling(true);
    }

    pub fn init_renderer(&mut self) {
        let window = self.window.as_mut().unwrap();
        let renderer = self.renderer.as_mut().unwrap();
        renderer.init(window);
    }

    pub fn run_loop(&mut self) {
        let window = self.window.as_mut().unwrap();
        let events = self.events.as_ref().unwrap();
        let glfw = self.glfw.as_mut().unwrap();
        let renderer = self.renderer.as_ref().unwrap();

        while !window.should_close() {

            // Polling
            glfw.poll_events();
            for (_, event) in glfw::flush_messages(&events) {
                GameWindow::handle_window_event(window, event);
            }

            renderer.render();
            window.swap_buffers();
        }
    }

    fn handle_window_event(window: &mut PWindow,  event: WindowEvent) {
        match event {
            glfw::WindowEvent::Key(Key::Escape, _, glfw::Action::Press, _) => {
                window.set_should_close(true)
            },
            glfw::WindowEvent::CursorPos(xpos, ypos) => {
                // println!("Cursor position: ({}, {})", xpos, ypos);
            },
            _ => {}
        }
    }
}