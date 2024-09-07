use core::f32;
use std::{fs::File, io::Write};

use anyhow::Result;
use glam::{vec3, Vec3};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{
    geometry::{Geometry, Ray, RayIntersectable},
    utils::InputData,
};

fn write_all<W: Write, I: Iterator<Item = u8>>(writer: &mut W, iter: I) -> Result<()> {
    const SIZE: usize = 1024;

    let mut buffer = [0u8; SIZE];
    let mut index = 0;

    for i in iter {
        buffer[index] = i;

        index += 1;
        if index == SIZE {
            writer.write_all(&buffer)?;
            index = 0;
        }
    }
    writer.write_all(&buffer[..index])?;
    Ok(())
}

pub struct Scene {
    pub eye: Vec3,
    pub img_coord: [Vec3; 4], // UL, UR, LL, LR
    pub resolution: (i32, i32),
    pub scene_obj: Vec<Geometry>,
    pub background: Vec3,
}

impl Scene {
    pub fn new(data: InputData) -> Scene {
        Scene {
            eye: data.eye,
            img_coord: data.img_coord,
            resolution: data.resolution,
            scene_obj: data.objects,
            background: vec3(0f32, 0f32, 0f32),
        }
    }
}

fn render_worker(coord: Vec3, eye: &Vec3, background: &Vec3, objs: &Vec<Geometry>) -> Vec3 {
    let ray = Ray::new(coord, coord - eye);
    let lerp = objs.iter().fold(f32::INFINITY, |acc, obj| {
        let collision = obj.ray_intersect(ray);
        let t = collision.map(|e| ray.solve(e));
        t.map(|t| acc.min(t)).unwrap_or(acc)
    });
    let output = if lerp.is_finite() {
        ray.lerp(lerp)
    } else {
        background.clone()
    };
    output
}

impl Scene {
    pub fn render(&mut self, file: &mut File) -> Result<()> {
        file.write("P6\n".as_bytes())?;
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
        let eval_pixel = |i| {
            let r = i as i32 / self.resolution.0;
            let c = i as i32 % self.resolution.0;
            let pixel_coord = origin
                + wv.normalize() * world_pixel_width * (c as f32)
                + hv.normalize() * world_pixel_height * (r as f32);
            render_worker(pixel_coord, &self.eye, &self.background, &self.scene_obj)
        };

        let size = self.resolution.0 as usize * self.resolution.1 as usize;
        let pixels = (0..size)
            .into_par_iter()
            .map(|i| eval_pixel(i))
            .flat_map(|pixel| {
                let result = (255.0 * (pixel + 1.0) / 2.0).round();
                [result.x as u8, result.y as u8, result.z as u8]
            })
            .collect_vec_list();
        write_all(file, pixels.into_iter().flatten())?;
        Ok(())
    }
}
