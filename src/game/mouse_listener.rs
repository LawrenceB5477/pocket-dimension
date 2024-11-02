use glam::Vec2;

pub struct MouseListener {
    mouse_position: Vec2,
    prev_mouse_position: Vec2,
}

impl MouseListener {
    pub fn new() -> Self {
        MouseListener {
            mouse_position: Vec2::default(),
            prev_mouse_position: Vec2::default(),
        }
    }

    pub fn mouse_pos_callback(&mut self, x: f64, y: f64) {
        self.prev_mouse_position = self.mouse_position;
        self.mouse_position = Vec2::new(x as f32, y as f32);
    }

    pub fn get_mouse_delta(&self) -> Vec2 {
        self.mouse_position - self.prev_mouse_position
    }

    pub fn end_frame(&mut self) {
        self.prev_mouse_position = self.mouse_position;
    }
}