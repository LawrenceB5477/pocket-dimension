
pub struct KeyListener {
    key_pressed: [bool; 350]
}

impl KeyListener {
    pub fn new() -> KeyListener {
        KeyListener {
            key_pressed: [false; 350]
        }
    }

    // pub fn is_key_pressed(&self, key: Key) -> bool {
    //     self.key_pressed[key as usize]
    // }

    pub fn handle_key_callback(&mut self, key: glfw::Key, action: glfw::Action) {
        if key as usize >= self.key_pressed.len() {
            panic!("Key out of range: {:?}", key);
        }

        if action == glfw::Action::Press {
            self.key_pressed[key as usize] = true;
        } else if action == glfw::Action::Release {
            self.key_pressed[key as usize] = false;
        }
    }

    pub fn is_key_pressed(&self, key: glfw::Key) -> bool {
        self.key_pressed[key as usize]
    }
}