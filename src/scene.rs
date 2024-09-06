use core::f32;
use std::{fs::File, io::Write};

use anyhow::Result;
use glam::Vec3;

use crate::geometry::{Ray, RayIntersectable};

pub struct Scene {
    pub eye: Vec3,
    pub img_coord: [Vec3; 4], // UL, UR, LL, LR
    pub resolution: (i32, i32),
    pub scene_obj: Vec<Box<dyn RayIntersectable>>,
    pub background: Vec3,
}

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

impl Scene {
    pub fn render(&self, file: &mut File) -> Result<()> {
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
        for r in 0..self.resolution.1 {
            for c in 0..self.resolution.0 {
                let world_canvas_coord = origin
                    + wv.normalize() * world_pixel_width * (c as f32)
                    + hv.normalize() * world_pixel_height * (r as f32);
                let ray = Ray::new(world_canvas_coord, world_canvas_coord - self.eye);
                let lerp = self.scene_obj.iter().fold(f32::INFINITY, |acc, obj| {
                    let collision = obj.ray_intersect(ray);
                    let t = collision.map(|e| ray.solve(e));
                    t.map(|t| acc.min(t)).unwrap_or(acc)
                });
                if lerp.is_finite() {
                    output_util(ray.lerp(lerp), file)?;
                } else {
                    output_util(self.background, file)?;
                };
            }
            file.write("\n".as_bytes())?;
        }
        Ok(())
    }
}
