use core::f32;
use std::{fs::File, io::Write, ptr, sync::Arc};

use anyhow::Result;
use glam::{vec3a as vec3, Vec3A as Vec3};
use indicatif::ParallelProgressIterator;
use rand::prelude::*;
use rand_distr::UnitSphere;
use rayon::prelude::*;

use crate::{
    geometry::{Geometry, Ray, RayIntersectable},
    utils::{InputData, Material},
};

pub struct Scene {
    pub eye: Vec3,
    canvas_wv: Vec3,
    canvas_hv: Vec3,
    canvas_corner: Vec3,
    pub resolution: (i32, i32),
    pub scene_obj: Box<[(Geometry, Arc<Material>)]>,
    pub background: Vec3,
    pub antialiasing: u8,
    pub light_position: Vec3,
}

impl Scene {
    pub fn new(data: InputData) -> Scene {
        Scene::new_with_antialiasing(data, 1)
    }
    pub fn new_with_antialiasing(data: InputData, antialiasing: u8) -> Scene {
        let center = data.eye + data.view_direction.normalize();
        let width = (data.fov / 2.0).to_radians().tan() * data.view_direction.length() * 2.0;
        let height = width * (data.resolution.1 as f32 / data.resolution.0 as f32);
        let unit_up = data.up_direction.normalize();
        let unit_left = unit_up.cross(data.view_direction).normalize();
        let corner = center + unit_up * (height / 2.0) + unit_left * (width / 2.0);
        Scene {
            eye: data.eye,
            canvas_corner: corner,
            canvas_hv: (-unit_up) * height,
            canvas_wv: (-unit_left) * width,
            resolution: data.resolution,
            scene_obj: Box::from(data.objects),
            background: vec3(0f32, 0f32, 0f32),
            light_position: data.light,
            antialiasing,
        }
    }
}

const MAX_RECURSION_DEPTH: u32 = 4;
fn render_worker(coordinate: &Vec3, scene: &Scene) -> Vec3 {
    render_worker_inner(coordinate, &(coordinate - scene.eye), scene, 1.0, 0)
        .unwrap_or(scene.background.clone())
}
fn render_worker_inner(
    origin: &Vec3,
    direction: &Vec3,
    scene: &Scene,
    importance: f32,
    recursion_depth: u32,
) -> Option<Vec3> {
    if importance < (1.0 / 256.0) || recursion_depth > MAX_RECURSION_DEPTH {
        return None;
    }

    let ray = Ray::new(*origin, *direction);
    scene
        .scene_obj
        .par_iter()
        .filter_map(|(obj, material)| {
            let collision = obj.ray_intersect(ray);
            assert!(collision.is_none() || !collision.unwrap_or(Vec3::NEG_INFINITY).is_nan());
            let t = collision.map(|e| ray.solve(e));
            t.map(|t| (t, obj, material))
        })
        .collect_vec_list()
        .into_iter()
        .flatten()
        .fold(Option::None, |acc, e| {
            Some(acc.map_or(
                e,
                |acc: (f32, &Geometry, &Arc<Material>)| {
                    if !e.0.is_nan() && acc.0 > e.0 {
                        e
                    } else {
                        acc
                    }
                },
            ))
        })
        .map(|(lerp, geo, material)| {
            let collision = ray.lerp(lerp);

            let normal_vec = {
                let tmp = geo.normal(collision).normalize();
                tmp * (tmp.dot(-direction)).signum()
            };

            assert!(!normal_vec.is_nan());

            let light_direction = (scene.light_position - collision).normalize();
            let shadow = scene
                .scene_obj
                .par_iter()
                .map(|(obj, _)| {
                    if ptr::eq(obj, geo) {
                        return None;
                    }
                    let ray = Ray::new(collision, light_direction);
                    let new_collision = obj.ray_intersect(ray);
                    new_collision
                        .filter(|c| c.distance(collision) > 1e-6)
                        .filter(|c| ray.solve(*c) <= ray.solve(scene.light_position))
                })
                .any(|e| e.is_some());

            let diffuse = if !shadow {
                normal_vec.dot(light_direction).max(0f32)
            } else {
                0f32
            };
            let specular = if !shadow {
                normal_vec
                    .dot((light_direction.normalize() + -direction.normalize()).normalize())
                    .max(0.0)
                    .powf(material.phong.3)
            } else {
                0f32
            };

            let reflect_vec = direction.reflect(normal_vec);
            let epsilon_factor = 1.0 / (scene.resolution.0.max(scene.resolution.1)) as f32;
            let mut rng = thread_rng();
            let reflect_color: Vec3 = {
                let samples = (2.0 * (importance + 0.4).tan().floor()) as u32;
                (1..samples)
                    .map(|_| Vec3::from_array(rng.sample(UnitSphere)))
                    .filter_map(|epsilon| {
                        render_worker_inner(
                            &(collision + epsilon * epsilon_factor),
                            &reflect_vec,
                            scene,
                            importance * material.reflect_rate,
                            recursion_depth + 1,
                        )
                    })
                    .map(|p| p / samples as f32)
                    .sum::<Vec3>()
            };

            (material.color * (material.phong.0 + material.phong.1 * diffuse)
                + Vec3::ONE * specular
                + reflect_color * material.reflect_rate)
                .clamp(Vec3::ZERO, Vec3::ONE)
        })
}

impl Scene {
    pub fn render(&mut self, file: &mut File) -> Result<()> {
        file.write("P6\n".as_bytes())?;
        file.write(format!("{} {}\n255\n", self.resolution.0, self.resolution.1).as_bytes())?;
        let world_pixel_width = self.canvas_wv.length() / self.resolution.0 as f32;
        let world_pixel_height = self.canvas_hv.length() / self.resolution.1 as f32;
        let size = self.resolution.0 as usize * self.resolution.1 as usize;
        let canvas = (0..size)
            .into_par_iter()
            .progress_count(size as u64)
            .map_init(
                || thread_rng(),
                |rng, i| {
                    let r = i as i32 / self.resolution.0;
                    let c = i as i32 % self.resolution.0;
                    (0..self.antialiasing)
                        .map(|i| {
                            let (subc, subr) = {
                                let sq = 1 << (self.antialiasing.trailing_zeros() / 2);
                                let threshold = sq << 2;
                                if i < threshold {
                                    (
                                        ((1 + i / sq) as f32 / (2.0 * self.antialiasing as f32)),
                                        ((1 + i % sq) as f32 / (2.0 * self.antialiasing as f32)),
                                    )
                                } else {
                                    (rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0))
                                }
                            };
                            let pixel_coord = self.canvas_corner
                                + self.canvas_wv.normalize()
                                    * world_pixel_width
                                    * (c as f32 + subc)
                                + self.canvas_hv.normalize()
                                    * world_pixel_height
                                    * (r as f32 + subr);
                            render_worker(&pixel_coord, self)
                        })
                        .map(|v| v / (self.antialiasing as f32))
                        .sum::<Vec3>()
                },
            )
            .flat_map(|pixel| {
                let result = (255.0 * pixel).round();
                [result.x as u8, result.y as u8, result.z as u8]
            })
            .collect::<Vec<_>>();
        file.write_all(&canvas)?;
        Ok(())
    }
}
