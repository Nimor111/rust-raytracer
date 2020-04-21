use crate::framework::ray::Ray;
use crate::geometry::hittable::{HitRecord, Hittable};

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new(h: Box<dyn Hittable>) -> Self {
        Self { objects: vec![h] }
    }

    pub fn add_object(&mut self, h: Box<dyn Hittable>) {
        self.objects.push(h)
    }

    pub fn clear(&mut self) {
        self.objects.clear()
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut record = None;

        for object in self.objects.iter() {
            match object.hit(ray, t_min, closest_so_far) {
                Some(rec) => {
                    closest_so_far = rec.time;
                    record = Some(rec);
                }
                None => {}
            }
        }

        record
    }
}
