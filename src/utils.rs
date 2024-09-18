use glam::Vec3A as Vec3;

use crate::geometry::{Geometry, Ray, RayMarchable, Sphere, Triangle};
use glam::vec3a as vec3;

use anyhow::{Error, Result};

pub fn ray_marching<T: RayMarchable + ?Sized>(ray: Ray, obj: &T) -> Option<Vec3> {
    let mut t = obj.distance(ray.lerp(0.0));
    const LIMIT: i32 = 100;
    let mut i = 0;
    loop {
        let next_pt = ray.lerp(t);
        let new_dist = obj.distance(next_pt);
        if new_dist < 1e-6 {
            break;
        }
        if i >= LIMIT {
            return None;
        }
        t += new_dist;
        i += 1;
    }

    // Prevent ray from marching INTO the sphere, fix the answer to the surface of the sphere
    Some(obj.fix_vec(ray.lerp(t)))
}

pub struct InputData {
    pub eye: Vec3,
    pub view_direction: Vec3,
    pub up_direction: Vec3,
    pub fov: f32, // in degree
    pub resolution: (i32, i32),
    pub objects: Vec<Geometry>,
}

impl InputData {
    pub fn parse(input_text: &str) -> Result<InputData> {
        let mut ret = InputData {
            eye: vec3(0f32, 0f32, 0f32),
            view_direction: vec3(0f32, 0f32, 0f32),
            up_direction:vec3(0f32, 0f32, 0f32),
            fov: 0f32,
            resolution: (0, 0),
            objects: vec![],
        };
        let lines = input_text.lines();
        let parse_error = || Error::msg("Parse error");
        for line in lines {
            let mut sep = line.split_ascii_whitespace();
            let id = sep.next().unwrap();
            match id {
                "E" => {
                    ret.eye = vec3(
                        sep.next().ok_or(parse_error())?.parse()?,
                        sep.next().ok_or(parse_error())?.parse()?,
                        sep.next().ok_or(parse_error())?.parse()?,
                    );
                }
                "V" => {
                    ret.view_direction = vec3(
                        sep.next().ok_or(parse_error())?.parse()?,
                        sep.next().ok_or(parse_error())?.parse()?,
                        sep.next().ok_or(parse_error())?.parse()?,
                    );
                    ret.up_direction = vec3(
                        sep.next().ok_or(parse_error())?.parse()?,
                        sep.next().ok_or(parse_error())?.parse()?,
                        sep.next().ok_or(parse_error())?.parse()?,
                    );
                }
                "F" => {
                    ret.fov = sep.next().ok_or(parse_error())?.parse()?;
                }
                "R" => {
                    ret.resolution = (
                        sep.next().ok_or(parse_error())?.parse()?,
                        sep.next().ok_or(parse_error())?.parse()?,
                    );
                }
                "S" => {
                    ret.objects.push(Geometry::Sphere(Sphere::new(
                        vec3(
                            sep.next().ok_or(parse_error())?.parse()?,
                            sep.next().ok_or(parse_error())?.parse()?,
                            sep.next().ok_or(parse_error())?.parse()?,
                        ),
                        sep.next().ok_or(parse_error())?.parse()?,
                    )));
                }
                "T" => {
                    ret.objects.push(Geometry::Triangle(Triangle::new([
                        vec3(
                            sep.next().ok_or(parse_error())?.parse()?,
                            sep.next().ok_or(parse_error())?.parse()?,
                            sep.next().ok_or(parse_error())?.parse()?,
                        ),
                        vec3(
                            sep.next().ok_or(parse_error())?.parse()?,
                            sep.next().ok_or(parse_error())?.parse()?,
                            sep.next().ok_or(parse_error())?.parse()?,
                        ),
                        vec3(
                            sep.next().ok_or(parse_error())?.parse()?,
                            sep.next().ok_or(parse_error())?.parse()?,
                            sep.next().ok_or(parse_error())?.parse()?,
                        ),
                    ])));
                }
                _ => {}
            }
        }
        Ok(ret)
    }
}
