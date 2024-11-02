use std::collections::HashMap;
use std::sync::RwLock;

use once_cell::sync::Lazy;

use crate::graphics::mesh::Mesh;
use crate::registry::shader_registry::{SHADER_REGISTRY, ShaderProgramId};
use crate::registry::texture_registry::TextureId;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum MeshId {
    Rect,
    TexturedRect,
    Quad,
    Wall,
}


impl MeshId {
    pub fn get_vbo(&self) -> Vec<f32> {
        match self {
            MeshId::Quad | MeshId::Rect | MeshId::TexturedRect | MeshId::Wall => vec![
                -0.5, -0.5, 0.0, 0.0, 0.0,  // Bottom-left
                0.5, -0.5, 0.0, 1.0, 0.0,  // Bottom-right
                0.5, 0.5, 0.0, 1.0, 1.0,  // Top-right
                -0.5, 0.5, 0.0, 0.0, 1.0,   // Top-left
            ]
        }
    }

    pub fn get_ebo(&self) -> Vec<u32> {
        match self {
            MeshId::Quad | MeshId::Rect | MeshId::TexturedRect | MeshId::Wall => vec![
                0, 1, 2,
                2, 3, 0,
            ]
        }
    }
}

pub(crate) static MESH_REGISTRY: Lazy<RwLock<HashMap<MeshId, Mesh>>> = Lazy::new(|| {
    let mut registry = HashMap::new();

    // Ortho ----
    // Rect mesh
    let ortho_program = SHADER_REGISTRY.read()
        .unwrap()
        .get(&ShaderProgramId::Ortho)
        .expect("ShaderProgramId::ORTHO not found")
        .to_owned();

    let point_mesh = Mesh::new(MeshId::Rect.get_vbo(), MeshId::Rect.get_ebo(), ortho_program, None);
    registry.insert(MeshId::Rect, point_mesh);


    // Textured rect mesh
    let texture_program = SHADER_REGISTRY.read()
        .unwrap()
        .get(&ShaderProgramId::TextureOrtho)
        .expect("ShaderProgramId::TEXTURE_ORTHO not found")
        .to_owned();

    let texture_rect_mesh = Mesh::new(
        MeshId::TexturedRect.get_vbo(),
        MeshId::TexturedRect.get_ebo(),
        texture_program,
        Some(TextureId::Shotgun));
    registry.insert(MeshId::TexturedRect, texture_rect_mesh);


    // Perspective ----

    // Quad mesh
    let perspective_program = SHADER_REGISTRY.read()
        .unwrap()
        .get(&ShaderProgramId::Perspective)
        .expect("ShaderProgramId::PERSPECTIVE not found")
        .to_owned();

    let perspective_mesh = Mesh::new(MeshId::Quad.get_vbo(), MeshId::Quad.get_ebo(), perspective_program, None);
    registry.insert(MeshId::Quad, perspective_mesh);

    // Wall mesh
    let texture_perspective= SHADER_REGISTRY.read()
        .unwrap()
        .get(&ShaderProgramId::TexturePerspective)
        .expect("ShaderProgramId::Texture perspective not found")
        .to_owned();

    let wall_mesh= Mesh::new(MeshId::Quad.get_vbo(), MeshId::Quad.get_ebo(), texture_perspective, Some(TextureId::BrickWall));
    registry.insert(MeshId::Wall, wall_mesh);

    RwLock::new(registry)
});
