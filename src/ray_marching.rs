use glam::Vec3;

use crate::geometry::{Ray, RayMarchable};

pub fn ray_marching<T: RayMarchable + ?Sized>(ray: Ray, obj: &T) -> Option<Vec3> {
    let mut distance = obj.distance(ray.lerp(0.0));
    let mut t = distance;
    while distance > 1e-6 {
        let next_pt = ray.lerp(t);
        let new_dist = obj.distance(next_pt);
        if new_dist > distance {
            return None;
        }
        distance = new_dist;
        t += new_dist;
    }

    // Prevent ray from marching INTO the sphere, fix the answer to the surface of the sphere
    Some(obj.fix_vec(ray.lerp(t)))
}
