mod vec3;
mod color;
mod ray;

use std::ops::Div;
use crate::color::{Color};
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

fn ray_color(r: Ray) -> Vec3 {
    let unit_direction = Vec3::unit_vector(r.dir());
    let a = 0.5 * (unit_direction.y() + 1.0);
    return (1.0-a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0);
}

fn main() {
    // Image
    let image_width: i64 = 256;
    let aspect_ratio = 16.0/9.0;
    // let image_height: i16 = 256;

    let mut image_height = image_width / aspect_ratio as i64;

    // make sure image height is at least one.
    image_height = if image_height < 1 { 1 } else {image_height};

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width / image_height) as f64;
    let camera_center = Point3::new(0.0,0.0,0.0);
    
    // Calculate the vectors across the horizontal and down the vertical viewport edges
    let viewport_u = Vec3::new(viewport_width, 0.0,0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);
    
    // calculate the horizontal and vertical delta vectors from pixel to pixel
    let pixel_delta_u = viewport_u / (image_width as f64);
    let pixel_delta_v = viewport_v / (image_height as f64);
    
    // calculate the location of the upper left pixel
    let viewport_upper_left = camera_center - Vec3::new(0.0, 0.0, focal_length)
                                    - viewport_u / 2.0 - viewport_v / 2.0;
    
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);
    
    // Render
    print!("P3\n{image_width} {image_height}\n255\n");
    for j in 0..image_height {
        eprintln!("\rScanlines remaining: {}", image_height - j);
        for i in 0..image_width {
            let pixel_center = pixel00_loc + ((i as f64) * pixel_delta_u)
                + ((j as f64) * pixel_delta_v);
           
           let ray_direction = pixel_center - camera_center;
            
           let r = Ray::new(camera_center, ray_direction);
            
           let pixel_color = ray_color(r);
    
           pixel_color.write_color(&mut std::io::stdout())
             .expect("error: failed to print to stdout");
        }
    }

    // Old Method
    // ------------------------------------------------------------
    // for j in 0..image_height {
    //     eprintln!("\rScanlines remaining: {}", image_height - j);
    //     for i in 0..image_width {
    //         let r = (i as f32) / ((image_width - 1) as f32);
    //         let g = (j as f32) / ((image_height - 1) as f32);
    //         let b = 0.0;
    //
    //         let ir = (255.999 * r) as i16;
    //         let ig = (255.999 * g) as i16;
    //         let ib = (255.999 * b) as i16;
    //
    //         print!("{ir} {ig} {ib}\n");
    //     }
    // }
    // -------------------------------------------------------------

    // Basically the hello world of raytracer, which is creating a rectangle of color gradients
    // ------------------------------------------------------------------------------------------
    // for j in 0..image_height {
    //     eprintln!("\rScanlines remaining: {}", image_height - j);
    //     for i in 0..image_width {
    //        let pixel_color = Color::new(
    //            (i as f64) / ((image_width - 1) as f64),
    //            (j as f64) / ((image_height - 1) as f64),
    //            0.0);
    //
    //        pixel_color.write_color(&mut std::io::stdout())
    //          .expect("error: failed to print to stdout");
    //     }
    // }
    //-------------------------------------------------------------------------------------------

}
