use core::f32;
use std::{fs::File, io::Write};

use anyhow::Result;
use glam::{vec3, Vec3};
use rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

use crate::{
    geometry::{Geometry, Ray, RayIntersectable},
    utils::InputData,
};

fn output_util(vec: Vec3, file: &mut File) -> Result<()> {
    let collision = vec;
    let results = (255.0 * (collision + 1.0) / 2.0).round();
    file.write(
        format!(
            "{} {} {} ",
            results.x as i32, results.y as i32, results.z as i32
        )
        .as_bytes(),
    )?;
    Ok(())
}

pub struct Scene {
    pub eye: Vec3,
    pub img_coord: [Vec3; 4], // UL, UR, LL, LR
    pub resolution: (i32, i32),
    pub scene_obj: Vec<Geometry>,
    pub background: Vec3,
    canvas: Vec<Vec3>,
}

impl Scene {
    pub fn new(data: InputData) -> Scene {
        let size = (data.resolution.0 * data.resolution.1) as usize;
        let canvas = vec![vec3(0f32, 0f32, 0f32); size];
        Scene {
            eye: data.eye,
            img_coord: data.img_coord,
            resolution: data.resolution,
            scene_obj: data.objects,
            background: vec3(0f32, 0f32, 0f32),
            canvas,
        }
    }
}

fn render_worker(
    output: &mut Vec3,
    coord: Vec3,
    eye: &Vec3,
    background: &Vec3,
    objs: &Vec<Geometry>,
) -> Vec3 {
    let ray = Ray::new(coord, coord - eye);
    let lerp = objs.iter().fold(f32::INFINITY, |acc, obj| {
        let collision = obj.ray_intersect(ray);
        let t = collision.map(|e| ray.solve(e));
        t.map(|t| acc.min(t)).unwrap_or(acc)
    });
    *output = if lerp.is_finite() {
        ray.lerp(lerp)
    } else {
        background.clone()
    };
    output.clone()
}

impl Scene {
    pub fn render(&mut self, file: &mut File) -> Result<()> {
        file.write("P3\n".as_bytes())?;
        file.write(format!("{} {}\n255\n", self.resolution.0, self.resolution.1).as_bytes())?;
        let (wv, hv) = (
            self.img_coord[1] - self.img_coord[0],
            self.img_coord[2] - self.img_coord[0],
        );
        let world_pixel_width = wv.length() / self.resolution.0 as f32;
        let world_pixel_height = hv.length() / self.resolution.1 as f32;
        let origin = self.img_coord[0]
            + (wv.normalize() * world_pixel_width / 2.0)
            + (hv.normalize() * world_pixel_height / 2.0);
        self.canvas
            .par_iter_mut()
            .enumerate()
            .map(|(i, pixel_ref)| {
                let r = i as i32 / self.resolution.0;
                let c = i as i32 % self.resolution.0;
                let pixel_coord = origin
                    + wv.normalize() * world_pixel_width * (c as f32)
                    + hv.normalize() * world_pixel_height * (r as f32);
                render_worker(
                    pixel_ref,
                    pixel_coord,
                    &self.eye,
                    &self.background,
                    &self.scene_obj,
                );
            })
            .count();
        for r in 0..self.resolution.1 {
            for c in 0..self.resolution.0 {
                output_util(self.canvas[(r * self.resolution.0 + c) as usize], file)?;
            }
            file.write("\n".as_bytes())?;
        }
        Ok(())
    }
}
