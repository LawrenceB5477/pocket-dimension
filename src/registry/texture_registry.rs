use std::collections::HashMap;
use std::sync::RwLock;

use once_cell::sync::Lazy;

use crate::graphics::texture::{Texture, TextureType};

pub const TEXTURE_PATH: &str = "/home/lars/projects/rust/pocket-dimension/assets/textures";

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub(crate) enum TextureId {
    BrickWall,
    Shotgun,
}

impl TextureId {
    pub fn get_path(&self) -> &'static str {
        match self {
            TextureId::BrickWall => "doom_wall.png",
            TextureId::Shotgun => "SHTFC0.png",
        }
    }
}

pub static TEXTURE_REGISTRY: Lazy<RwLock<HashMap<TextureId, Texture>>> = Lazy::new(|| {
    let mut registry = HashMap::new();

    let brick_wall = Texture::new(TextureId::BrickWall, TextureType::SPRITE);
    registry.insert(TextureId::BrickWall, brick_wall);

    let shotgun = Texture::new(TextureId::Shotgun, TextureType::SPRITE);
    registry.insert(TextureId::Shotgun, shotgun);

    RwLock::new(registry)
});