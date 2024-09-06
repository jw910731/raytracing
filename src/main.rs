mod geometry;
mod ray_marching;
mod scene;

use std::{env::args, fs::File, io::Read};

use geometry::{RayMarchable, Sphere, Triangle};
use glam::{vec3, Vec3};

use anyhow::{Error, Result};
use scene::Scene;

struct InputData {
    eye: Vec3,
    img_coord: [Vec3; 4], // UL, UR, LL, LR
    resolution: (i32, i32),
    objects: Vec<Box<dyn RayMarchable>>,
}

impl InputData {
    fn parse(input_text: &str) -> Result<InputData> {
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
                    ret.objects.push(Box::new(Sphere::new(
                        vec3(
                            sep.next().ok_or(parse_error())?.parse()?,
                            sep.next().ok_or(parse_error())?.parse()?,
                            sep.next().ok_or(parse_error())?.parse()?,
                        ),
                        sep.next().ok_or(parse_error())?.parse()?,
                    )));
                }
                "T" => {
                    ret.objects.push(Box::new(Triangle::new([
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

fn main() {
    let args = args().collect::<Vec<_>>();
    if args.len() < 3 {
        eprintln!("You should provide 1 argument to specify the path of input file");
        eprintln!("You should provide 2 argument to specify the path of output file");
        return;
    }
    let mut in_file = File::open(args[1].as_str()).unwrap();
    let mut out_file = File::create(args[2].as_str()).unwrap();
    let mut input_buf = String::new();
    in_file.read_to_string(&mut input_buf).unwrap();
    let data = InputData::parse(&input_buf).unwrap();
    let scene = Scene {
        eye: data.eye,
        img_coord: data.img_coord,
        resolution: data.resolution,
        scene_obj: data.objects,
        background: vec3(0f32, 0f32, 0f32),
    };
    scene.render(&mut out_file).unwrap();
}
