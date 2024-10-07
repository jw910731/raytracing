use anyhow::{Error, Result};
use shared_structs::{Material, SceneMetadata, Sphere, Triangle};

use glam::{uvec2, vec3, Vec3};

#[derive(Clone, Copy)]
pub enum Geometry {
    Sphere(Sphere),
    Triangle(Triangle),
}

pub struct InputData {
    pub eye: Vec3,
    pub view_direction: Vec3,
    pub up_direction: Vec3,
    pub fov: f32, // in degree
    pub resolution: (i32, i32),
    pub objects: Vec<(Geometry, Material)>,
    pub light: Vec3,
}

pub fn new_scene_metadata(data: InputData, antialiasing: u32) -> SceneMetadata {
    let center = data.eye + data.view_direction.normalize();
    let width = (data.fov / 2.0).to_radians().tan() * data.view_direction.length() * 2.0;
    let height = width * (data.resolution.1 as f32 / data.resolution.0 as f32);
    let unit_up = data.up_direction.normalize();
    let unit_left = unit_up.cross(data.view_direction).normalize();
    let corner = center + unit_up * (height / 2.0) + unit_left * (width / 2.0);
    SceneMetadata {
        eye: data.eye,
        canvas_corner: corner,
        canvas_hv: (-unit_up) * height,
        canvas_wv: (-unit_left) * width,
        resolution: uvec2(data.resolution.0 as u32, data.resolution.1 as u32),
        background: vec3(0f32, 0f32, 0f32),
        light_position: data.light,
        antialiasing,
        padding: 0,
    }
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

        let mut current_material = Material {
            color: vec3(0f32, 0f32, 0f32),
            ka: 0f32,
            kd: 0f32,
            ks: 0f32,
            specular_exp: 0f32,
            reflect_rate: 0f32,
        };
        let mut auto_inc_id: u32 = 0;
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
                    current_material = Material {
                        color: vec3(
                            sep.next().ok_or(parse_error())?.parse()?,
                            sep.next().ok_or(parse_error())?.parse()?,
                            sep.next().ok_or(parse_error())?.parse()?,
                        ),
                        ka: sep.next().ok_or(parse_error())?.parse()?,
                        kd: sep.next().ok_or(parse_error())?.parse()?,
                        ks: sep.next().ok_or(parse_error())?.parse()?,
                        specular_exp: sep.next().ok_or(parse_error())?.parse()?,
                        reflect_rate: sep.next().ok_or(parse_error())?.parse()?,
                    }
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
                            auto_inc_id,
                            vec3(
                                sep.next().ok_or(parse_error())?.parse()?,
                                sep.next().ok_or(parse_error())?.parse()?,
                                sep.next().ok_or(parse_error())?.parse()?,
                            ),
                            sep.next().ok_or(parse_error())?.parse()?,
                        )),
                        current_material.clone(),
                    ));
                    auto_inc_id += 1;
                }
                "T" => {
                    ret.objects.push((
                        Geometry::Triangle(Triangle::new(
                            auto_inc_id,
                            [
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
                            ],
                        )),
                        current_material.clone(),
                    ));
                    auto_inc_id += 1;
                }
                _ => {}
            }
        }
        Ok(ret)
    }
}
