use ray_tracer::color::{color_gradient, write_color};
use ray_tracer::ray::Ray;
use ray_tracer::simple_camera::Camera;
use ray_tracer::vec3::Vec3;
use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    // write to file
    let mut f = match OpenOptions::new()
    .write(true)
    .truncate(true)
    .create(true)
    .open("./images/gradient.ppm") {
        Ok(f) => f,
        Err(e) => {
            eprintln!("could not create file: {}", e);
            return
        }
    };
    
    let camera = Camera {
        aspect_ratio: 16.0 / 9.0,
        image_width: 400.0,
        viewport_height: 250.0,
        focal_length: 1.0,
        camera_direction: Vec3{x: 0.0, y: 0.0, z: 1.0},
        camera_center: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
    };

    let image_height = camera.get_image_height();
    let viewport_width = camera.get_viewport_width();
    
    // define u v coordinates
    let viewport_u = Vec3 {
        x: viewport_width,
        y: 0.0,
        z: 0.0,
    };
    let viewport_v = Vec3 {
        x:0.0,
        y: -camera.viewport_height,
        z:0.0,
    };
    
    // calculate horizontal and vertical delta v from pixel to pixel
    let pixel_delta_u = viewport_u / camera.image_width;
    let pixel_delta_v = viewport_v / image_height;

    // location of upper left pixel
    let focal_length_vec = Vec3{x: 0.0, y: 0.0, z: camera.focal_length};
    let viewport_upper_left = camera.camera_center 
    - focal_length_vec
    - viewport_u/2.0
    - viewport_v/2.0;
    
    let pixel00_loc = viewport_upper_left 
        + (pixel_delta_u + pixel_delta_v) * 0.5;
    
    // file header
    if let Err(e) = writeln!(f, "P3") {
        eprintln!("couldn't write to file: {}", e)
    };
    if let Err(e) = writeln!(f, "{} {}", camera.image_width as i32, image_height as i32){
         eprintln!("couldn't write to file: {}", e)
    };
    if let Err(e) = writeln!(f, "255") {
        eprintln!("couldn't write to file: {}", e)
    };
    
    // render 
    for i in 0..(image_height as i32) {
        for j in 0..(camera.image_width as i32) {
            let pixel_center = pixel00_loc 
                + (pixel_delta_v * i)
                + (pixel_delta_u * j);
            
            let ray_direction = pixel_center - camera.camera_center;
            
            let r = Ray {
                origin: camera.camera_center,
                direction: ray_direction,
            };
            
            let mut p = color_gradient(&r);
            p = write_color(&p);
            
            if let Err(e) = writeln!(
                f,
                "{} {} {}",
                p.x as i32,
                p.y as i32,
                p.z as i32
            ) {
                eprintln!("couldn't write to file: {}", e)
            };
        }
    }
}
