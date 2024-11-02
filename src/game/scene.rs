use glam::{Vec2, Vec3, Vec4};

use crate::game::camera::Camera;
use crate::game::key_listener::KeyListener;
use crate::game::mouse_listener::MouseListener;
use crate::graphics;

pub trait Scene {
    // 60 fps
    fn update_fixed(&mut self, camera: &mut Camera, key_listener: &KeyListener, mouse_listener: &mut MouseListener);

    fn draw(&mut self, camera: &Camera, renderer: &mut graphics::render::Renderer);
}


pub struct MainScene {
    rotation: f32,
}

impl MainScene {
    pub fn new() -> MainScene {
        MainScene {
            rotation: 0.0,
        }
    }

    fn draw_crosshair(&self, renderer: &mut graphics::render::Renderer) {
        // // Vertical
        renderer.draw_line(Vec2::new(renderer.window_size.x / 2.0, renderer.window_size.y / 2.0 - 10.0),
                           Vec2::new(renderer.window_size.x / 2.0, renderer.window_size.y / 2.0 + 10.0),
                           2.0,
                           Vec4::new(1.0, 0.0, 0.0, 1.0));
        // // Horizontal
        renderer.draw_line(Vec2::new(renderer.window_size.x / 2.0 - 10.0, renderer.window_size.y / 2.0),
                           Vec2::new(renderer.window_size.x / 2.0 + 10.0, renderer.window_size.y / 2.0),
                           2.0,
                           Vec4::new(1.0, 0.0, 0.0, 1.0));
    }

    fn draw_ui(&self, renderer: &mut graphics::render::Renderer) {
        renderer.draw_rect_textured(
            Vec2::new(0., 0.0),
            Vec2::new(renderer.window_size.x, renderer.window_size.y),
            0.0,
            Vec4::new(0.1, 0.1, 0.1, 0.2),
        );

        // renderer.draw_polygon(&ShapeBuilder::get_hexagon_vertices(),
        //                       Vec3::new(200.0, 200.0, 0.0),
        //                       self.rotation,
        //                       30.0,
        //                       Vec4::new(1.0, 0.0, 0.0, 1.0));
        //
        // renderer.draw_polygon(&ShapeBuilder::get_octagon_vertices(),
        //                       Vec3::new(400.0, 200.0, 0.0),
        //                       self.rotation,
        //                       30.0,
        //                       Vec4::new(1.0, 0.0, 0.0, 1.0));
        //
        // renderer.draw_polygon(&ShapeBuilder::get_octagon_vertices(),
        //                       Vec3::new(960.0, 200.0, 0.0),
        //                       self.rotation,
        //                       30.0,
        //                       Vec4::new(1.0, 0.0, 0.0, 1.0));

        // renderer.draw_polygon(&ShapeBuilder::get_high_res_cirlce_vertices(),
        //                       Vec3::new(renderer.window_size.x / 2.0,renderer.window_size.y / 2.0, 0.0),
        //                       self.rotation,
        //                       renderer.window_size.y,
        //                       Vec4::new(1.0, 0.0, 0.0, 1.0));

        // let size = 20.0;
        // let x = renderer.window_size.x / 2.0 - (size / 2.0);
        // let y = renderer.window_size.y / 2.0 - (size / 2.0);
        //
        // renderer.draw_point(Vec2::new(x, y), size, Vec4::new(0.5, 1.0, 0.0, 1.0));

        //
        // renderer.draw_line(Vec2::new(50.0, 50.0), Vec2::new(renderer.window_size.x - 50.0, renderer.window_size.y - 50.0),
        //                    5.0,
        //                    Vec4::new(0.0, 1.0, 0.0, 1.0));
        //
        // renderer.draw_line(Vec2::new(50.0, renderer.window_size.y - 50.0), Vec2::new(renderer.window_size.x - 50.0, 50.0),
        //                    5.0,
        //                    Vec4::new(0.0, 1.0, 0.0, 1.0));
    }
}

impl Scene for MainScene {
    // TODO I'm not sure if scene should own the camera or not.
    fn update_fixed(&mut self, camera: &mut Camera, key_listener: &KeyListener, mouse_listener: &mut MouseListener) {
        // self.rotation += 3.0;

        // Handle movement
        if key_listener.is_key_pressed(glfw::Key::W) {
            camera.move_main_axis(true);
        } else if key_listener.is_key_pressed(glfw::Key::S) {
            camera.move_main_axis(false);
        }

        if key_listener.is_key_pressed(glfw::Key::D) {
            camera.move_cross_axis(true);
        } else if key_listener.is_key_pressed(glfw::Key::A) {
            camera.move_cross_axis(false);
        }

        // Handle mouse movement
        let mouse_delta = mouse_listener.get_mouse_delta();
        if mouse_delta.x != 0.0 {
            camera.rotate_yaw(mouse_delta.x * 0.01);
        }

        if mouse_delta.y != 0.0 {
            camera.rotate_pitch(mouse_delta.y * -0.01);
        }

        // Reset mouse delta
        mouse_listener.end_frame();
    }

    fn draw(&mut self, camera: &Camera, renderer: &mut graphics::render::Renderer) {
        renderer.draw_wall(camera.get_view_matrix(),
                           Vec3::new(0.0, 0.0, -20.0),
                           Vec2::new(100.0, 10.0),
                           self.rotation, Vec4::new(1.0, 1.0, 1.0, 1.0));

        // Draw ui
        self.draw_ui(renderer);
    }
}