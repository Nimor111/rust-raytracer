#![allow(dead_code)]

mod framework;
mod geometry;

use crate::framework::camera::Camera;
use crate::framework::ray::Ray;
use crate::framework::util::{random_float, INFINITY};
use crate::framework::vec3::Vec3;
use crate::geometry::hittable::Hittable;
use crate::geometry::hittable_list::HittableList;
use crate::geometry::sphere::Sphere;

const IMAGE_WIDTH: i32 = 200;
const IMAGE_HEIGHT: i32 = 100;
const SAMPLES_PER_PIXEL: i32 = 100;
const MAX_DEPTH: i32 = 50;

fn ray_color(ray: &Ray, world: &impl Hittable, depth: i32) -> Vec3 {
    if depth <= 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }

    match world.hit(ray, 0.001, INFINITY) {
        Some(rec) => {
            let target = rec.p + rec.normal + Vec3::random_unit_vector();
            //let target = rec.p + rec.normal + Vec3::random_in_unit_sphere();
            //let target = rec.p + Vec3::random_in_hemisphere(&rec.normal);
            ray_color(&Ray::new(rec.p, target - rec.p), world, depth - 1) * 0.5
        }
        None => {
            let unit_direction = ray.direction.unit();
            let t = 0.5 * (unit_direction.y + 1.0);
            Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
        }
    }
}

fn main() {
    println!("P3\n {} {} \n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    let mut world = HittableList::default();

    world.add_object(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add_object(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    let camera = Camera::default();

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("\nScanlines remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            let mut color = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f32 + random_float()) / IMAGE_WIDTH as f32;
                let v = (j as f32 + random_float()) / IMAGE_HEIGHT as f32;
                let r = camera.get_ray(u, v);
                color = color + ray_color(&r, &world, MAX_DEPTH);
            }
            color.write_color(SAMPLES_PER_PIXEL);
        }
    }
    eprintln!("\nDone\n");
}
