use glam::f32::Vec3A as Vec3;

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

pub trait RayIntersectable {
    // Return intersection point in respect to the given ray
    fn ray_intersect(&self, ray: Ray) -> Option<Vec3>;

    // Get normal vector at the intersection point
    fn normal(&self, intersection_point: Vec3) -> Vec3;
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

impl RayIntersectable for Sphere {
    fn ray_intersect(&self, ray: Ray) -> Option<Vec3> {
        let l = self.center - ray.origin;
        let tca = l.dot(ray.direction());
        if tca.is_nan() || tca < 0.0 {
            return None;
        }
        let d_2 = l.length_squared() - tca.powi(2);
        if d_2 < 0.0 {
            return None;
        }
        let thc = (self.radius.powi(2) - d_2).sqrt();
        if thc.is_nan() {
            return None;
        }
        Some(ray.lerp(tca - thc))
    }

    fn normal(&self, intersection_point: Vec3) -> Vec3 {
        (self.center - intersection_point).normalize()
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

        if y.0.dot(y.1) < 0.0 || y.0.dot(y.2) < 0.0 {
            return None;
        }
        Some(projection)
    }

    fn normal(&self, _: Vec3) -> Vec3 {
        self.normal
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Point(Vec3);

impl RayIntersectable for Point {
    fn ray_intersect(&self, ray: Ray) -> Option<Vec3> {
        if ray.direction().dot((self.0 - ray.origin).normalize()) - 1.0 < 1e-6 {
            Some(ray.lerp(ray.solve(self.0)))
        } else {
            None
        }
    }

    fn normal(&self, _intersection_point: Vec3) -> Vec3 {
        Vec3::ZERO
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Geometry {
    Sphere(Sphere),
    Triangle(Triangle),
    Point(Point),
}

impl RayIntersectable for Geometry {
    fn ray_intersect(&self, ray: Ray) -> Option<Vec3> {
        match self {
            Geometry::Sphere(s) => s.ray_intersect(ray),
            Geometry::Triangle(t) => t.ray_intersect(ray),
            Geometry::Point(p) => p.ray_intersect(ray),
        }
    }

    fn normal(&self, intersection_point: Vec3) -> Vec3 {
        match self {
            Geometry::Sphere(s) => s.normal(intersection_point),
            Geometry::Triangle(t) => t.normal(intersection_point),
            Geometry::Point(p) => p.normal(intersection_point),
        }
    }
}
