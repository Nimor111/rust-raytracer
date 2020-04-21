use crate::framework::ray::Ray;
use crate::framework::vec3::Vec3;

pub struct Camera {
    pub origin: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lower_left_corner: Vec3,
}

impl Camera {
    pub fn new(origin: Vec3, horizontal: Vec3, vertical: Vec3, lower_left_corner: Vec3) -> Self {
        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin,
        )
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            lower_left_corner: Vec3::new(-2.0, -1.0, -1.0),
            origin: Vec3::new(0.0, 0.0, 0.0),
            vertical: Vec3::new(0.0, 2.0, 0.0),
            horizontal: Vec3::new(4.0, 0.0, 0.0),
        }
    }
}
