use std::ops::Range;
use glam::{ DVec3, Vec3, Mat3 };
use crate::{ hittable::{ HitRecord, Hittable }, material::Material, ray::Ray };

pub struct Cylinder {
    pub center: DVec3,
    pub height: f64,
    pub radius: f64,
    pub material: Material,
    pub translation: DVec3, // Added translation
    pub rotation: f64, // Added rotation (in radians)
}

fn to_vec3(v: DVec3) -> Vec3 {
    Vec3::new(v.x as f32, v.y as f32, v.z as f32)
}

impl Cylinder {
    fn rotation_matrix_y(angle: f64) -> Mat3 {
        // Convert the angle to f32
        Mat3::from_rotation_y(angle as f32)
    }

    fn transform_ray(&self, ray: &Ray) -> Ray {
        let rotation_matrix = Self::rotation_matrix_y(-self.rotation);

        // Use the helper function for conversion
        let origin = to_vec3(ray.origin - self.translation - self.center);
        let direction = to_vec3(ray.direction);

        Ray {
            origin: DVec3::from(rotation_matrix * origin),
            direction: DVec3::from(rotation_matrix * direction),
        }
    }
}

impl Hittable for Cylinder {
    fn hit(&self, ray: &Ray, interval: Range<f64>) -> Option<HitRecord> {
        let transformed_ray = self.transform_ray(ray);
        let oc = transformed_ray.origin;

        // These coefficients are part of the quadratic formula solution
        let a =
            transformed_ray.direction.x * transformed_ray.direction.x +
            transformed_ray.direction.z * transformed_ray.direction.z;
        let b = 2.0 * (oc.x * transformed_ray.direction.x + oc.z * transformed_ray.direction.z);
        let c = oc.x * oc.x + oc.z * oc.z - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-b - sqrtd) / (2.0 * a);
        if !interval.contains(&root) {
            root = (-b + sqrtd) / (2.0 * a);
            if !interval.contains(&root) {
                return None;
            }
        }

        let point = transformed_ray.at(root);

        // Check if the intersection point is within the height of the cylinder
        if point.y < 0.0 || point.y > self.height {
            return None;
        }

        let outward_normal = DVec3::new(point.x, 0.0, point.z).normalize();

        // Reverse the transformations for the normal and the hit point
        let rotation_matrix = Self::rotation_matrix_y(self.rotation);
        let transformed_normal = DVec3::from(rotation_matrix * to_vec3(outward_normal));
        let transformed_point =
            DVec3::from(rotation_matrix * to_vec3(point)) + self.translation + self.center;

        let rec = HitRecord::with_face_normal(
            self.material.clone(),
            transformed_point,
            transformed_normal,
            root,
            ray
        );

        Some(rec)
    }
}
