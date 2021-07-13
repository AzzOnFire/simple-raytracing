mod vec3;
mod ray;
mod objects;
mod scene;

use vec3::Vec3;
use ray::Ray;
use scene::Scene;

use image::{RgbImage, Rgb};


fn main() {
    let width = 200;
    let height = 100;
    let mut img = RgbImage::new(width, height);
    
    let origin = Vec3::new(0.0, 0.0, 0.0);
    let corner = Vec3::new(-2.0, -1.0, -1.0);

    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);

    let mut scene: Scene<f64> = Scene::new();
    scene.add_demo_sphere();

    for x in 0..width {
        for y in 0..height {
            let u: f64 = (x as f64) / (width as f64);
            let v: f64 = ((height - y) as f64) / (height as f64);
            let ray = Ray::new(origin, corner + horizontal*u + vertical*v);
            
            let color = scene.color(&ray);
            
            img.put_pixel(x, y, Rgb([(255.99 * color.x) as u8, (255.99 * color.y) as u8, (255.99 * color.z) as u8]))
        }
    }

    img.save("image.jpeg").expect("Can't save image");
}
