use ray_tracer::{ppm, vec3};

fn main() {
    println!("Creating PPM image");
    ppm::create_ppm(400, 600);
}
