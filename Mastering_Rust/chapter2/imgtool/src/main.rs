use std::env;
use std::path::Path;

fn main() {
    let image_path = env::args().skip(1).next().unwrap();
    let path = Path::new(&image_path);

    let img = image::open(path).unwrap();

    let rotated = img.rotate90();
    let v = format!("{}_rotated.{}",path.file_stem().unwrap().to_str().unwrap(),path.extension().unwrap().to_str().unwrap());
    let r_path = Path::new(&v);
    rotated.save(r_path).unwrap();
}
