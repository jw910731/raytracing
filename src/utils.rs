use std::sync::Arc;

use glam::Vec3A as Vec3;

use crate::geometry::{Geometry, Sphere, Triangle};
use glam::vec3a as vec3;

use anyhow::{Error, Result};

pub struct Material {
    pub color: Vec3,
    pub phong: (f32, f32, f32, f32), // Ka, Kd, Ks, specular_exp
    pub reflect_rate: f32,
}

pub struct InputData {
    pub eye: Vec3,
    pub view_direction: Vec3,
    pub up_direction: Vec3,
    pub fov: f32, // in degree
    pub resolution: (i32, i32),
    pub objects: Vec<(Geometry, Arc<Material>)>,
    pub light: Vec3,
}

impl InputData {
    pub fn parse(input_text: &str) -> Result<InputData> {
        let mut ret = InputData {
            eye: vec3(0f32, 0f32, 0f32),
            view_direction: vec3(0f32, 0f32, 0f32),
            up_direction: vec3(0f32, 0f32, 0f32),
            fov: 0f32,
            resolution: (0, 0),
            objects: vec![],
            light: vec3(0f32, 0f32, 0f32),
        };
        let lines = input_text.lines();
        let parse_error = || Error::msg("Parse error");

        let mut current_material = Arc::new(Material {
            color: vec3(0f32, 0f32, 0f32),
            phong: (0f32, 0f32, 0f32, 0f32),
            reflect_rate: 0f32,
        });
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
                "M" => {
                    current_material = Arc::new(Material {
                        color: vec3(
                            sep.next().ok_or(parse_error())?.parse()?,
                            sep.next().ok_or(parse_error())?.parse()?,
                            sep.next().ok_or(parse_error())?.parse()?,
                        ),
                        phong: (
                            sep.next().ok_or(parse_error())?.parse()?,
                            sep.next().ok_or(parse_error())?.parse()?,
                            sep.next().ok_or(parse_error())?.parse()?,
                            sep.next().ok_or(parse_error())?.parse()?,
                        ),
                        reflect_rate: sep.next().ok_or(parse_error())?.parse()?,
                    })
                }
                "L" => {
                    ret.light = vec3(
                        sep.next().ok_or(parse_error())?.parse()?,
                        sep.next().ok_or(parse_error())?.parse()?,
                        sep.next().ok_or(parse_error())?.parse()?,
                    );
                }
                "S" => {
                    ret.objects.push((
                        Geometry::Sphere(Sphere::new(
                            vec3(
                                sep.next().ok_or(parse_error())?.parse()?,
                                sep.next().ok_or(parse_error())?.parse()?,
                                sep.next().ok_or(parse_error())?.parse()?,
                            ),
                            sep.next().ok_or(parse_error())?.parse()?,
                        )),
                        current_material.clone(),
                    ));
                }
                "T" => {
                    ret.objects.push((
                        Geometry::Triangle(Triangle::new([
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
                        ])),
                        current_material.clone(),
                    ));
                }
                _ => {}
            }
        }
        Ok(ret)
    }
}
