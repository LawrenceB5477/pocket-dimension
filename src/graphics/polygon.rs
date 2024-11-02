use std::hash::{Hash, Hasher};

use glam::Vec3;
use ordered_float::NotNan;

use crate::graphics::mesh::Mesh;
use crate::graphics::program::ShaderProgram;

pub struct Polygon {
    pub mesh: Mesh,
    hash: u64,
}

impl Polygon {
    // Assume vertices are in counter-clockwise order
    // TODO sort vertices in counter-clockwise order if needed
    pub fn new(vertices: &[Vec3], program: ShaderProgram) -> Self {
        if vertices.len() < 3 {
            panic!("Polygon must have at least 3 vertices");
        }

        // Build VBO
        let vbo: Vec<f32> = vertices.iter()
            .flat_map(|v| {
                let mut temp = Vec::from(v.to_array());
                temp.extend([0.0, 0.0]);
                temp
            })
            .collect();

        // Build EBO
        // Triangulate using fan method
        let mut ebo: Vec<u32> = Vec::new();
        for i in 1..vertices.len() - 1 {
            ebo.push(0);
            ebo.push(i as u32);
            ebo.push((i + 1) as u32);
        }

        Polygon {
            mesh: Mesh::new(vbo, ebo, program, None),
            hash: Polygon::get_vertex_list_hash(vertices),
        }
    }

    pub fn get_vertex_list_hash(vertices: &[Vec3]) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        let ordered_vertices: Vec<NotNan<f32>> = vertices.iter()
            .flat_map(|v| {
                vec![
                    NotNan::new(v.x).expect("Invalid vertex"),
                    NotNan::new(v.y).expect("Invalid vertex"),
                    NotNan::new(v.z).expect("Invalid vertex"),
                ]
            }).collect();
        ordered_vertices.hash(&mut hasher);
        hasher.finish()
    }
}


impl PartialEq for Polygon {
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash
    }
}

impl Eq for Polygon {}
