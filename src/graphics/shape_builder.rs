use std::f32::consts::PI;

use glam::Vec3;

pub struct ShapeBuilder {
}

impl ShapeBuilder {
    pub fn get_hexagon_vertices() -> Vec<Vec3> {
        ShapeBuilder::get_ngon_vertices(6)
    }

    pub fn get_octagon_vertices() -> Vec<Vec3> {
        ShapeBuilder::get_ngon_vertices(8)
    }

    pub fn get_low_res_cirlce_vertices() -> Vec<Vec3> {
        ShapeBuilder::get_ngon_vertices(30)
    }

    pub fn get_high_res_cirlce_vertices() -> Vec<Vec3> {
        ShapeBuilder::get_ngon_vertices(60)
    }

    pub fn get_ngon_vertices(n: usize) -> Vec<Vec3> {
        let mut vertices: Vec<Vec3> = Vec::new();
        let step = PI / (n as f32 / 2.0);
        for i in  0..n {
            vertices.push(Vec3::new(0.5 * (step * i as f32).cos(), 0.5 * (step * i as f32).sin(), 0.0));
        }
        vertices
    }
}