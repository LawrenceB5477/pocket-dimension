use std::path::Path;

pub fn load_shader_source(location: impl AsRef<Path>) -> String {
    let path = location.as_ref();
    if !path.exists() {
        panic!("Shader file not found: {:?}", path);
    }
    std::fs::read_to_string(path).expect("Failed to read shader file")
}