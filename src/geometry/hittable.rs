use crate::framework::ray::Ray;
use crate::framework::vec3::Vec3;

pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub time: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(p: Vec3, normal: Vec3, time: f32) -> Self {
        Self {
            p,
            normal,
            time,
            front_face: false,
        }
    }

    pub fn set_face_normal(&self, ray: &Ray, outward_normal: Vec3) -> Self {
        let front_face = ray.direction.dot(&outward_normal) < 0.0;

        let normal = match front_face {
            true => outward_normal,
            false => -outward_normal,
        };

        Self {
            p: self.p,
            normal: normal,
            front_face: front_face,
            time: self.time,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}
