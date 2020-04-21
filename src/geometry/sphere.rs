use crate::framework::ray::Ray;
use crate::framework::vec3::Vec3;
use crate::geometry::hittable::{HitRecord, Hittable};

pub struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    /// Does a ray intersect a sphere?
    /// t^2*b.b + 2t*b.(a - c) + (a-c).(a-c) - R^2 = 0
    /// t is the p(t) param of the ray
    /// b is the direction vector of the ray
    /// a is the origin vector of the ray
    /// c is the center vector of the sphere
    /// R is the radius of the sphere
    /// This is derived from the sphere equation:
    /// (x-cx)*(x-cx) + (y-cy)*(y-cy) + (z-cz)*(z-cz) = R^2
    /// (x,y,z) - a point
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        //let a = ray.direction.dot(ray.direction);
        let a = ray.direction.length_squared();
        let half_b = oc.dot(&ray.direction);
        //let b = oc.dot(ray.direction) * 2.0;
        //let c = oc.dot(oc) - radius * radius;
        let c = oc.length_squared() - self.radius * self.radius;

        //let discriminant = b*b - 4.0*a*c;
        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let temp = (-half_b - root) / a;
            if temp < t_max && temp > t_min {
                let p = ray.at(temp);
                let outward_normal = (p - self.center) / self.radius;
                Some(HitRecord::new(p, outward_normal, temp).set_face_normal(ray, outward_normal))
            } else {
                let temp = (-half_b + root) / a;
                if temp < t_max && temp > t_min {
                    let p = ray.at(temp);
                    let outward_normal = (p - self.center) / self.radius;
                    Some(
                        HitRecord::new(p, outward_normal, temp)
                            .set_face_normal(ray, outward_normal),
                    )
                } else {
                    None
                }
            }
        } else {
            None
        }
    }
}
