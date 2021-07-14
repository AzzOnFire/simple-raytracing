mod vec3;
mod ray;
mod objects;
mod scene;
mod camera;
mod material;

use vec3::Vec3;
use ray::{Ray, Intersection};
use scene::Scene;
use camera::Camera;
use rand::prelude::*;

use image::{RgbImage, Rgb};


fn main() {
    let width = 800;
    let height = 600;
    let mut img = RgbImage::new(width, height);
    let mut rng = rand::thread_rng();

    let rays_count = 4;

    let camera: Camera<f64> = Camera::new(Vec3::new(3.5, 0.5, 2.0), Vec3::new(0.0, 0.0, -1.0), 70.0, width as f64 / height as f64);
    let mut scene: Scene<f64> = Scene::new();
    scene.add_demo_sphere();

    for x in 0..width {
        for y in 0..height {
            let mut result_color: Vec3<f64> = Vec3::zero();

            for _ in 0..rays_count {
                let u: f64 = ((x as f64) + rng.gen::<f64>()) / (width as f64);
                let v: f64 = (((height - y) as f64) + rng.gen::<f64>()) / (height as f64);
                let ray = camera.get_ray(u, v);
                
                let color = scene.color(&ray, 0);
                result_color = result_color + color;
            }

            result_color = result_color / rays_count as f64;
            img.put_pixel(x, y, Rgb([(255.99 * result_color.x) as u8, (255.99 * result_color.y) as u8, (255.99 * result_color.z) as u8]))
        }
    }

    img.save("image.png").expect("Can't save image");
}
