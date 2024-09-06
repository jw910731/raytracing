use glam::f32::Vec3;

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
        todo!()
    }
}

pub struct Triangle {
    verticies: [Vec3; 3],
    ortho_vec: Vec3,
}

impl Triangle {
    pub fn new(pts: [Vec3; 3]) -> Triangle {
        Triangle {
            verticies: pts,
            ortho_vec: (pts[1] - pts[0]).cross(pts[2] - pts[0]),
        }
    }
}

impl Distance for Triangle {
    // Generated by pethetic generative AI which includes mathemagic that I definetely don't understand
    fn distance(&self, pt: Vec3) -> f32 {
        let point_segment_distance = |pt: Vec3, seg_start: Vec3, seg_end: Vec3| -> f32 {
            let segment = seg_end - seg_start;
            let t = (pt - seg_start).dot(segment) / segment.length_squared();
            let t_clamped = t.clamp(0.0, 1.0);
            let closest_point = seg_start + t_clamped * segment;
            (pt - closest_point).length()
        };
        // Project the point onto the plane of the triangle
        let v = pt - self.verticies[0];
        let dist_to_plane = v.dot(self.ortho_vec.normalize());
        let projected_pt = pt - dist_to_plane * self.ortho_vec.normalize();

        // Check if the projected point is inside the triangle
        let area = |a: Vec3, b: Vec3, c: Vec3| (b - a).cross(c - a).length();

        if area(self.verticies[0], self.verticies[1], self.verticies[2])
            == (area(projected_pt, self.verticies[0], self.verticies[1])
                + area(projected_pt, self.verticies[1], self.verticies[2])
                + area(projected_pt, self.verticies[0], self.verticies[2]))
        {
            // Point is inside the triangle, return the distance to the plane
            dist_to_plane.abs()
        } else {
            // Point is outside the triangle, find the closest point on the edges or vertices
            let dist_to_edge0 = point_segment_distance(pt, self.verticies[0], self.verticies[1]);
            let dist_to_edge1 = point_segment_distance(pt, self.verticies[1], self.verticies[2]);
            let dist_to_edge2 = point_segment_distance(pt, self.verticies[2], self.verticies[0]);
            let dist_to_vertex0 = (pt - self.verticies[0]).length();
            let dist_to_vertex1 = (pt - self.verticies[1]).length();
            let dist_to_vertex2 = (pt - self.verticies[2]).length();

            dist_to_edge0
                .min(dist_to_edge1)
                .min(dist_to_edge2)
                .min(dist_to_vertex0)
                .min(dist_to_vertex1)
                .min(dist_to_vertex2)
        }
    }
}

impl RayMarchable for Triangle {
    fn fix_vec(&self, pt: Vec3) -> Vec3 {
        let v = pt - self.verticies[0];
        let dist_to_plane = v.dot(self.ortho_vec.normalize());
        pt - dist_to_plane * self.ortho_vec.normalize()
    }
}

impl RayIntersectable for Triangle {
    fn ray_intersect(&self, ray: Ray) -> Option<Vec3> {
        todo!()
    }
}
