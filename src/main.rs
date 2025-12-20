use ray_tracer::ray::Ray;
use ray_tracer::simple_camera::Camera;
use ray_tracer::vec3::{Point3, Vec3};

fn main() {
    
    // init camera 
    let camera_dir = Ray{
        origin: Vec3{x: 0.0, y: 0.0, z: 0.0},
        direction: Vec3{x: 1.0, y: 1.0, z: 1.0},
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
    
    // render 
    for i in 0..(image_height as i32) {
        for j in 0..(camera.image_width as i32) {
            let pixel_center = pixel00_loc 
                + (pixel_delta_u * i)
                + (pixel_delta_v * j);
            
            let ray_direction = pixel_center - camera.camera_center;
            
            let r = Ray {
                origin: camera.camera_center,
                direction: ray_direction,
            };
            
            
        }
    }
}
