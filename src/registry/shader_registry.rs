use std::collections::HashMap;
use std::sync::RwLock;

use once_cell::sync::Lazy;

use crate::graphics::program::ShaderProgram;

const ASSET_PATH: &str = "/home/lars/projects/rust/pocket-dimension/assets/shaders";

// TODO redo this

pub enum ShaderId {
    VertexBasic,
    FragmentBasic,
    // Orthographic
    VertexOrthographic,
    FragmentOrthographic,
    // Textured Orthographic
    TextureVertexOrthographic,
    TextureFragmentOrthographic,
    // Perspective
    VertexPerspective,
    FragmentPerspective,
    // Textured Perspective
    TextureVertexPerspective,
    TextureFragmentPerspective,

}

impl ShaderId {
    pub(crate) fn get_path(&self) -> &'static str {
        match self {
            ShaderId::VertexBasic => "basic/vertex.glsl",
            ShaderId::FragmentBasic => "basic/fragment.glsl",

            ShaderId::VertexOrthographic => "ortho/vertex.glsl",
            ShaderId::FragmentOrthographic => "ortho/fragment.glsl",

            ShaderId::TextureVertexOrthographic => "texture_ortho/vertex.glsl",
            ShaderId::TextureFragmentOrthographic => "texture_ortho/fragment.glsl",

            ShaderId::VertexPerspective => "perspective/vertex.glsl",
            ShaderId::FragmentPerspective => "perspective/fragment.glsl",

            ShaderId::TextureVertexPerspective => "texture_perspective/vertex.glsl",
            ShaderId::TextureFragmentPerspective => "texture_perspective/fragment.glsl",
        }
    }

}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum ShaderProgramId {
    Basic,
    Ortho,
    TextureOrtho,
    Perspective,
    TexturePerspective,
}

pub static SHADER_REGISTRY: Lazy<RwLock<HashMap<ShaderProgramId, ShaderProgram>>> = Lazy::new(|| {
    let mut registry = HashMap::new();

    // Orthographic shader
    let mut ortho_program = ShaderProgram::new(ASSET_PATH, ShaderId::VertexOrthographic, ShaderId::FragmentOrthographic);
    ortho_program.build().expect("Failed to build ortho program");
    registry.insert(ShaderProgramId::Ortho, ortho_program);

    // Texture orthographic shader
    let mut texture_ortho_program = ShaderProgram::new(ASSET_PATH, ShaderId::TextureVertexOrthographic, ShaderId::TextureFragmentOrthographic);
    texture_ortho_program.build().expect("Failed to build texture ortho program");
    registry.insert(ShaderProgramId::TextureOrtho, texture_ortho_program);

    // Perspective shader
    let mut perspective_program = ShaderProgram::new(ASSET_PATH, ShaderId::VertexPerspective, ShaderId::FragmentPerspective);
    perspective_program.build().expect("Failed to build perspective program");
    registry.insert(ShaderProgramId::Perspective, perspective_program);

    // Texture perspective shader
    let mut texture_perspective_program = ShaderProgram::new(ASSET_PATH, ShaderId::TextureVertexPerspective, ShaderId::TextureFragmentPerspective);
    texture_perspective_program.build().expect("Failed to build texture perspective program");
    registry.insert(ShaderProgramId::TexturePerspective, texture_perspective_program);

    RwLock::new(registry)
});