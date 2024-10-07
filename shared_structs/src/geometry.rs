use spirv_std::glam::Vec3;

#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float;

fn vec3_sum(v: Vec3) -> f32 {
    v.x + v.y + v.z
}

#[derive(Clone, Copy)]
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
        (vec3_sum(pt) - vec3_sum(self.origin)) / vec3_sum(self.direction)
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }
}

pub trait RayIntersectable {
    // Return intersection point in respect to the given ray
    fn ray_intersect(&self, ray: Ray) -> Option<Vec3>;

    // Get normal vector at the intersection point
    fn normal(&self, intersection_point: Vec3) -> Vec3;
}

#[derive(Clone, Copy)]
pub struct Sphere {
    id: u32,
    pub center: Vec3,
    pub radius: f32,
}

impl Sphere {
    pub fn new(id: u32, center: Vec3, radius: f32) -> Sphere {
        Sphere { id, center, radius }
    }

    pub fn id(self) -> u32 {
        self.id
    }
}

impl PartialEq for Sphere {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl RayIntersectable for Sphere {
    fn ray_intersect(&self, ray: Ray) -> Option<Vec3> {
        let l = self.center - ray.origin;
        let tca = l.dot(ray.direction());
        if tca.is_nan() || tca < 0.0 {
            return None;
        }
        let d_2 = l.length_squared() - (tca.powi(2));
        if d_2 < 0.0 {
            return None;
        }
        let thc = ((self.radius.powi(2)) - d_2).sqrt();
        if thc.is_nan() {
            return None;
        }
        Some(ray.lerp(tca - thc))
    }

    fn normal(&self, intersection_point: Vec3) -> Vec3 {
        (self.center - intersection_point).normalize()
    }
}

#[derive(Clone, Copy)]
pub struct Triangle {
    id: u32,
    verticies: [Vec3; 3],
    normal: Vec3,
}

impl Triangle {
    pub fn new(id: u32, pts: [Vec3; 3]) -> Triangle {
        Triangle {
            id,
            verticies: pts,
            normal: (pts[1] - pts[0]).cross(pts[2] - pts[0]),
        }
    }

    pub fn id(self) -> u32 {
        self.id
    }
}

impl PartialEq for Triangle {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl RayIntersectable for Triangle {
    // Assisted by perplexity generative AI, I am not pretty sure what the hell this is doing
    fn ray_intersect(&self, ray: Ray) -> Option<Vec3> {
        let dot: f32 = self.normal.dot(ray.direction());
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

        if y.0.dot(y.1) < 0.0 || y.0.dot(y.2) < 0.0 {
            return None;
        }
        Some(projection)
    }

    fn normal(&self, _: Vec3) -> Vec3 {
        self.normal
    }
}
