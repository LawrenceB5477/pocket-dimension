use glam::{Mat4, Vec3};

pub struct Camera {
    front: Vec3,
    up: Vec3,
    right: Vec3,
    pitch: f32,
    yaw: f32,

    // Movement
    pub position: Vec3,
    movement_speed: f32,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            position: Vec3::new(0.0, 0.0, 0.0),
            front: Vec3::new(0.0, 0.0, -1.0).normalize(),
            up: Vec3::new(0.0, 1.0, 0.0),
            right: Default::default(),
            pitch: 0.0,
            yaw: 0.0,
            movement_speed: 4.0,
        }
    }

    pub fn get_view_matrix(&self) -> Mat4 {
        Mat4::look_to_rh(self.position, self.front, Vec3::new(0.0, 1.0, 0.0))
    }

    pub fn move_main_axis(&mut self, forward: bool) {
        let forward_ground = Vec3::new(self.front.x, 0.0, self.front.z).normalize();
        if forward {
            self.position += forward_ground * self.movement_speed;
        } else {
            self.position -= forward_ground * self.movement_speed;
        }
    }

    pub fn move_cross_axis(&mut self, move_right: bool) {
        let forward_ground = Vec3::new(self.front.x, 0.0, self.front.z).normalize();
        let right = forward_ground.cross(Vec3::new(0.0, 1.0, 0.0)).normalize();

        if move_right{
            self.position += right * self.movement_speed;
        } else {
            self.position -= right * self.movement_speed;
        }
    }

    pub fn rotate_yaw(&mut self, angle: f32) {
        self.yaw += angle;
        self.update_front();
    }

    pub fn rotate_pitch(&mut self, angle: f32) {
        self.pitch += angle;

        // Clamp pitch to avoid flipping
        self.pitch = self.pitch.clamp(-89.0_f32.to_radians(), 89.0_f32.to_radians());

        self.update_front();
    }

    fn update_front(&mut self) {
        // Calculate the new front vector based on yaw and pitch
        self.front = Vec3::new(
            self.yaw.cos() * self.pitch.cos(),
            self.pitch.sin(),
            self.yaw.sin() * self.pitch.cos(),
        )
            .normalize();

        // Update right and up vectors for stability
        self.right = self.front.cross(Vec3::new(0.0, 1.0, 0.0)).normalize();
        self.up = self.right.cross(self.front).normalize();
    }
}
