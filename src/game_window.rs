use glfw::{Context, GlfwReceiver, Key, PWindow, WindowEvent, WindowHint};

use crate::game::camera::Camera;
use crate::game::key_listener::KeyListener;
use crate::game::mouse_listener::MouseListener;
use crate::game::scene::{MainScene, Scene};
use crate::graphics::render::Renderer;

const SCREEN_WIDTH: u32 = 1920;
const SCREEN_HEIGHT: u32 = 1080;

pub struct GameWindow {
    glfw: glfw::Glfw,
    window: PWindow,
    events: GlfwReceiver<(f64, WindowEvent)>,

    // Game info
    renderer: Renderer,
    scene: Box<dyn Scene>,
    camera: Camera,
    key_listener: KeyListener,
    mouse_listener: MouseListener,

}

impl GameWindow {
    pub fn new() -> GameWindow {
        let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

        glfw.window_hint(WindowHint::ContextVersion(3, 3));
        glfw.window_hint(WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        glfw.window_hint(WindowHint::OpenGlForwardCompat(true));
        glfw.window_hint(WindowHint::Resizable(true));
        glfw.window_hint(WindowHint::Visible(true));
        glfw.window_hint(WindowHint::Focused(true));

        // Create actual window
        let (mut window, events) = glfw.create_window(
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            "Hello, Triangle",
            glfw::WindowMode::Windowed)
            .unwrap();

        // Set up polling
        window.make_current();
        window.set_key_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_cursor_mode(glfw::CursorMode::Disabled);
        // glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

        let mut renderer = Renderer::new(&mut window);
        let scene = Box::new(MainScene::new());
        GameWindow {
            glfw: glfw,
            window: window,
            events: events,
            renderer: renderer,
            scene: scene,
            camera: Camera::new(),
            key_listener: KeyListener::new(),
            mouse_listener: MouseListener::new(),
        }
    }

    pub fn run_loop(&mut self) {
        let mut last_time = self.glfw.get_time();
        let frame_time = 1.0 / 60.0;
        let mut elapsed_frames = 0.;

        while !self.window.should_close() {
            let now = self.glfw.get_time();
            let mut delta_time_s = now - last_time;
            elapsed_frames += (delta_time_s / frame_time).floor();
            last_time = now;

            // Polling
            self.glfw.poll_events();
            for (_, event) in glfw::flush_messages(&self.events) {
                GameWindow::handle_window_event(&mut self.window,
                                                event,
                                                &mut self.key_listener,
                                                &mut self.mouse_listener
                );
            }

            // Perform update logic, ideally at 60 fps
            while elapsed_frames >= 1.0 {
                self.scene.update_fixed(&mut self.camera, &self.key_listener, &mut self.mouse_listener);
                elapsed_frames -= 1.0;
            }

            // Clear screen
            unsafe {
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            }

            self.scene.draw(&self.camera, &mut self.renderer);

            // Swap buffers
            self.window.swap_buffers();
        }
    }

    fn handle_window_event(window: &mut PWindow,
                           event: WindowEvent,
                           key_listener: &mut KeyListener,
                           mouse_listener: &mut MouseListener
    ) {
        match event {
            // If escape
            WindowEvent::Key(Key::Escape, _, glfw::Action::Press, _) => {
                window.set_should_close(true)
            }
            // Any other key
            WindowEvent::Key(key, _, event, _) => {
                key_listener.handle_key_callback(key, event);
            }
            WindowEvent::CursorPos(xpos, ypos) => {
                mouse_listener.mouse_pos_callback(xpos, ypos);
            }
            _ => {}
        }
    }
}