use std::io::BufReader;
use std::fs::File;
use std::path::Path;

fn load_mesh() {
    // Load model
    let mut tank = BufReader::new(File::open("assets/models/Bench.obj").unwrap());
    let (models, materials_result) = tobj::load_obj_buf(&mut tank, &tobj::LoadOptions::default(), |p| {
        let mut mtl = BufReader::new(File::open(Path::new("assets/models/").join(p)).unwrap());
        tobj::load_mtl_buf(&mut mtl)
    }).unwrap();

    let materials = materials_result.unwrap();

    let model = models.get(0).unwrap();
    let material = materials.get(0).unwrap();
}