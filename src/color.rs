use crate::vec3::Color;

pub fn write_color(color: &Color) {
    let c = 255.999;
    let ir = c * color.x();
    let ig = c * color.y();
    let ib = c * color.z();

    println!("{:?} {:?} {:?}", ir, ig, ib)
}
