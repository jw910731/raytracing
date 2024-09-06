use glam::f32::Vec3;

use crate::utils::ray_marching;

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray {
            origin,
            direction: direction.normalize(),
        }
    }

    /// return the vector point of the ray advances for a given length
    ///
    /// ## Params:
    /// * `length` - how far the ray advance
    pub fn lerp(&self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }

    pub fn solve(&self, pt: Vec3) -> f32 {
        (pt.element_sum() - self.origin.element_sum()) / self.direction.element_sum()
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }
}

pub trait Distance {
    fn distance(&self, v: Vec3) -> f32;
}

pub trait RayMarchable: Distance {
    fn fix_vec(&self, v: Vec3) -> Vec3;
}

pub trait RayIntersectable {
    fn ray_intersect(&self, ray: Ray) -> Option<Vec3>;
}

#[derive(Clone, Copy, Debug)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Sphere {
        Sphere { center, radius }
    }
}

impl Distance for Sphere {
    fn distance(&self, vec: Vec3) -> f32 {
        (vec - self.center).length() - self.radius
    }
}

impl RayMarchable for Sphere {
    fn fix_vec(&self, v: Vec3) -> Vec3 {
        let fix_vec = v - self.center;
        self.center + (fix_vec * (self.radius / fix_vec.length()))
    }
}

impl RayIntersectable for Sphere {
    fn ray_intersect(&self, ray: Ray) -> Option<Vec3> {
        ray_marching(ray, self)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Triangle {
    verticies: [Vec3; 3],
    normal: Vec3,
}

impl Triangle {
    pub fn new(pts: [Vec3; 3]) -> Triangle {
        Triangle {
            verticies: pts,
            normal: (pts[1] - pts[0]).cross(pts[2] - pts[0]),
        }
    }
}

impl RayIntersectable for Triangle {
    // Assisted by perplexity generative AI, I am not pretty sure what the hell this is doing
    fn ray_intersect(&self, ray: Ray) -> Option<Vec3> {
        let dot = self.normal.dot(ray.direction());
        if dot.abs() < 1e-6 {
            return None;
        }
        let t = self.normal.dot(self.verticies[0] - ray.origin) / dot;
        if t < 0.0 {
            return None;
        }

        // project on to the plane
        let projection = ray.lerp(t);

        // Inside-outside test
        let x = (
            self.verticies[0] - projection,
            self.verticies[1] - projection,
            self.verticies[2] - projection,
        );
        let y = (x.1.cross(x.2), x.2.cross(x.0), x.0.cross(x.1));

        if y.0.dot(y.1) < 1e-6 || y.0.dot(y.2) < 1e-6 {
            return None;
        }
        Some(projection)
    }
}

pub enum Geometry {
    Sphere(Sphere),
    Triangle(Triangle),
}

impl RayIntersectable for Geometry {
    fn ray_intersect(&self, ray: Ray) -> Option<Vec3> {
        match self {
            Geometry::Sphere(s) => s.ray_intersect(ray),
            Geometry::Triangle(t) => t.ray_intersect(ray),
        }
    }
}
