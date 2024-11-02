use std::collections::HashMap;

use glam::{Mat4, Quat, Vec2, Vec3, Vec4};
use glfw::PWindow;

use crate::graphics::polygon::Polygon;
use crate::graphics::program::Uniform;
use crate::registry::mesh_registry;
use crate::registry::mesh_registry::MeshId;
use crate::registry::shader_registry::{SHADER_REGISTRY, ShaderProgramId};

// TODO factor out to config info
pub struct Renderer {
    pub window_size: Vec2,
    polygon_cache: HashMap<u64, Polygon>,
    orthographic_projection: Mat4,
    perspective_projection: Mat4,
}

// Draws a scene with a camera
impl Renderer {
    pub fn new(window: &mut PWindow) -> Renderer {
        // Load OpenGL functions dynamically
        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        // Set viewport size
        // TODO handle updates to window size
        let mut window_size = Vec2::default();
        unsafe {
            let (width, height) = window.get_framebuffer_size();
            window_size = Vec2::new(width as f32, height as f32);
            gl::Viewport(0, 0, width, height);
            gl::ClearColor(0.8, 0.87, 0.95, 1.0);
            gl::Enable(gl::DEPTH_TEST);

            // TODO research this
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }

        Renderer {
            window_size: window_size,
            polygon_cache: HashMap::new(),
            orthographic_projection: Mat4::orthographic_rh_gl(
                0.0,
                window_size.x,
                0.0,
                window_size.y,
                -1.0,
                1.0,
            ),
            perspective_projection: Mat4::perspective_rh_gl(
                90.0_f32.to_radians(),
                window_size.x / window_size.y,
                0.1,
                1000.0,
            ),
        }
    }

    pub fn draw_point(&self, position: Vec2, size: f32, color: Vec4) {
        // let half_size = size / 2.0;
        let model = Mat4::from_scale_rotation_translation(Vec3::splat(size), Quat::IDENTITY, Vec3::new(position.x, position.y, 0.0));

        let uniforms = vec![
            Uniform::Matrix4f("model".to_string(), model),
            Uniform::Matrix4f("projection".to_string(), self.orthographic_projection.clone()),
            Uniform::Vec4("diffuse".to_string(), color),
        ];

        mesh_registry::MESH_REGISTRY.read()
            .unwrap()
            .get(&MeshId::Rect).expect("MeshId::Quad not found")
            .draw(&uniforms);
    }

    // Position is bottom left corner
    pub fn draw_rect(&self, position: Vec2, size: Vec2, rotation_deg: f32, color: Vec4) {
        let width_offset = size.x / 2.0;
        let height_offset = size.y / 2.0;

        let model = Mat4::from_scale_rotation_translation(
            Vec3::new(size.x, size.y, 1.0),
            Quat::from_rotation_z(rotation_deg.to_radians()),
            Vec3::new(position.x + width_offset, position.y + height_offset, 0.0));

        let uniforms = vec![
            Uniform::Matrix4f("model".to_string(), model),
            Uniform::Matrix4f("projection".to_string(), self.orthographic_projection.clone()),
            Uniform::Vec4("diffuse".to_string(), color),
        ];

        mesh_registry::MESH_REGISTRY.read()
            .unwrap()
            .get(&MeshId::Rect).expect("MeshId::Quad not found")
            .draw(&uniforms);
    }

    pub fn draw_rect_textured(&self, position: Vec2, size: Vec2, rotation_deg: f32, color: Vec4) {
        let width_offset = size.x / 2.0;
        let height_offset = size.y / 2.0;

        let x_scale = 1.0;

        let model = Mat4::from_scale_rotation_translation(
            Vec3::new(size.x, size.y, 1.0),
            Quat::from_rotation_z(rotation_deg.to_radians()),
            Vec3::new(position.x + width_offset, position.y + height_offset, 0.0));

        let uniforms = vec![
            Uniform::Matrix4f("model".to_string(), model),
            Uniform::Matrix4f("projection".to_string(), self.orthographic_projection.clone()),
            // Uniform::Vec4("diffuse".to_string(), color),
            Uniform::Int("texture1".to_string(), 0),
            Uniform::Float("texWidthScale".to_string(), x_scale),
        ];

        mesh_registry::MESH_REGISTRY.read()
            .unwrap()
            .get(&MeshId::TexturedRect).expect("MeshId::Textured not found")
            .draw(&uniforms);
    }

    pub fn draw_line(&self, start: Vec2, end: Vec2, thickness: f32, color: Vec4) {
        let length = (end - start).length();
        let midpoint = Vec2::new((start.x + end.x) / 2.0, (start.y + end.y) / 2.0);
        let angle = (end - start).y.atan2(end.x - start.x);


        let model = Mat4::from_scale_rotation_translation(
            Vec3::new(length, thickness, 1.0),
            Quat::from_rotation_z(angle),
            Vec3::new(midpoint.x, midpoint.y, 0.0));

        let uniforms = vec![
            Uniform::Matrix4f("model".to_string(), model),
            Uniform::Matrix4f("projection".to_string(), self.orthographic_projection.clone()),
            Uniform::Vec4("diffuse".to_string(), color),
        ];

        mesh_registry::MESH_REGISTRY.read()
            .unwrap()
            .get(&MeshId::Rect).expect("MeshId::Quad not found")
            .draw(&uniforms);
    }

    pub fn draw_wall(&self, view_matrix: Mat4, position: Vec3, size: Vec2, rotation_y_deg: f32, color: Vec4) {
        let x_scale = size.x / size.y * 2.0;
        let model = Mat4::from_scale_rotation_translation(
            Vec3::new(size.x, size.y, 1.0),
            Quat::from_rotation_y(rotation_y_deg.to_radians()),
            position);

        let uniforms = vec![
            Uniform::Matrix4f("model".to_string(), model),
            Uniform::Matrix4f("view".to_string(), view_matrix),
            Uniform::Matrix4f("projection".to_string(), self.perspective_projection.clone()),
            // Uniform::Vec4("diffuse".to_string(), color),
            Uniform::Int("texture1".to_string(), 0),
            Uniform::Float("texWidthScale".to_string(), x_scale),
        ];

        mesh_registry::MESH_REGISTRY.read()
            .unwrap()
            .get(&MeshId::Wall).expect("MeshId::Quad not found")
            .draw(&uniforms);
    }

    pub fn draw_polygon(&mut self, vertices: &[Vec3], position: Vec3, rotation_deg: f32, scale: f32, color: Vec4) {
        let model = Mat4::from_scale_rotation_translation(
            Vec3::splat(scale),
            Quat::from_rotation_z(rotation_deg.to_radians()),
            position,
        );

        let uniforms = vec![
            Uniform::Matrix4f("model".to_string(), model),
            Uniform::Matrix4f("projection".to_string(), self.orthographic_projection.clone()),
            Uniform::Vec4("diffuse".to_string(), color),
        ];

        let polygon = self.get_polygon_from_cache_or_store(vertices);
        polygon.mesh.draw(&uniforms);
    }

    fn get_polygon_from_cache_or_store(&mut self, vertices: &[Vec3]) -> &Polygon {
        let hash = Polygon::get_vertex_list_hash(vertices);

        if !self.polygon_cache.contains_key(&hash) {
            let ortho_program = SHADER_REGISTRY.read()
                .unwrap()
                .get(&ShaderProgramId::Ortho)
                .expect("ShaderProgramId::ORTHO not found")
                .clone()
                .into();

            let polygon = Polygon::new(vertices, ortho_program);
            self.polygon_cache.insert(hash, polygon);
        }
        self.polygon_cache.get(&hash).unwrap()
    }
}