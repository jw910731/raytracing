use glam::Vec3A as Vec3;

use crate::geometry::{Geometry, Ray, RayMarchable, Sphere, Triangle};
use glam::vec3a as vec3;

use anyhow::{Error, Result};

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

pub struct InputData {
    pub eye: Vec3,
    pub img_coord: [Vec3; 4], // UL, UR, LL, LR
    pub resolution: (i32, i32),
    pub objects: Vec<Geometry>,
}

impl InputData {
    pub fn parse(input_text: &str) -> Result<InputData> {
        let mut ret = InputData {
            eye: vec3(0f32, 0f32, 0f32),
            img_coord: [
                vec3(0f32, 0f32, 0f32),
                vec3(0f32, 0f32, 0f32),
                vec3(0f32, 0f32, 0f32),
                vec3(0f32, 0f32, 0f32),
            ],
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
                "O" => {
                    ret.img_coord = [
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
                        vec3(
                            sep.next().ok_or(parse_error())?.parse()?,
                            sep.next().ok_or(parse_error())?.parse()?,
                            sep.next().ok_or(parse_error())?.parse()?,
                        ),
                    ];
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
