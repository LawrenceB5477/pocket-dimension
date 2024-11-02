use std::path::Path;

use gl::types::{GLint, GLuint};
use image::GenericImageView;

use crate::registry::texture_registry::{TEXTURE_PATH, TextureId};

#[derive(Copy, Clone)]
pub struct Texture {
    id: GLuint,
}

pub enum TextureType {
    SPRITE,
    TEXTURE,
}

impl Texture {
    pub fn new(id: TextureId, texture_type: TextureType) -> Texture {

        // Open image
        let path = Path::new(TEXTURE_PATH).join(id.get_path());
        let mut image = image::open(path.as_path()).expect("Failed to load texture image");
        image = image.flipv();

        // Get raw data, figure out color channel format
        let (data, format, height, width) = match image {
            image::DynamicImage::ImageRgb8(_) => (image.to_rgb8().into_raw(), gl::RGB, image.height(), image.width()),
            image::DynamicImage::ImageRgba8(_) => (image.to_rgba8().into_raw(), gl::RGBA, image.height(), image.width()),
            _ => panic!("Unsupported image format")
        };


        let internal_format = if format == gl::RGB { gl::RGB8 } else { gl::RGBA8 };

        // Build the texture
        let mut texture_id = 0;
        unsafe {
            gl::GenTextures(1, &mut texture_id);

            // Bind texture
            gl::BindTexture(gl::TEXTURE_2D, texture_id);

            // Set wrapping
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as GLint);


            // Set filtering
            match texture_type {
                TextureType::SPRITE => {
                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as GLint);
                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as GLint);
                },
                TextureType::TEXTURE => {
                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);
                }
            }

            // Load image data
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                internal_format as i32,
                width as i32,
                height as i32,
                0,
                format,
                gl::UNSIGNED_BYTE,
                data.as_ptr().cast());

            gl::GenerateMipmap(gl::TEXTURE_2D);

            // Unbind texture
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }

        Texture {
            id: texture_id
        }
    }

    // Paramaterize what texture unit to bind to
    pub fn bind(&self, unit: u32) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + unit);
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }
}
