#![cfg_attr(target_arch = "spirv", no_std)]

mod rng;
use rng as rng_;

#[cfg(target_arch = "spirv")]
#[allow(unused_imports)]
use spirv_std::num_traits::{float::Float, int::PrimInt};

use glam::{UVec3, Vec3};
use shared_structs::{Material, Ray, RayIntersectable, SceneMetadata, Sphere, Triangle};
use spirv_std::{
    glam::{self, uvec2},
    spirv,
};

pub fn reflect(slf: Vec3, normal: Vec3) -> Vec3 {
    slf - 2.0 * slf.dot(normal) * normal
}

const MAX_RECURSION_DEPTH: u32 = 8;
fn render_worker_inner(
    worker_id: u32,
    origin: &Vec3,
    direction: &Vec3,
    scene: &SceneMetadata,
    sphere_objs: &[Sphere],
    sphere_material: &[Material],
    mesh_objs: &[Triangle],
    mesh_material: &[Material],
    importance: f32,
    recursion_depth: u32,
    prev_obj_id: i32,
) -> Vec3 {
    if importance < (1.0 / 256.0) || recursion_depth > MAX_RECURSION_DEPTH {
        return scene.background.clone();
    }

    let mut rng = rng_::RngState::new(uvec2(worker_id & 0xaa, worker_id & 0x55));

    let intersect = |ray, repel: i32| -> (f32, Vec3, Material, i32) {
        let sphere_collision = sphere_objs
            .iter()
            .enumerate()
            .filter(|(_, geo)| !(geo.id() as i32 == repel))
            .filter_map(|(i, obj)| {
                let collision = obj.ray_intersect(ray);
                assert!(collision.is_none() || !collision.map(|c| c.is_nan()).unwrap_or_default());
                collision.map(|c| (ray.solve(c), obj.normal(c), sphere_material[i], obj.id()))
            })
            .fold((0f32, Vec3::ZERO, Material::default(), -1), |acc, e| {
                Some(acc.map_or(
                    e,
                    |acc: (f32, Vec3, Material, u32)| {
                        if !e.0.is_nan() && acc.0 > e.0 {
                            e
                        } else {
                            acc
                        }
                    },
                ))
            });
        let mesh_collision = mesh_objs
            .iter()
            .enumerate()
            .filter(|(_, geo)| !repel.map(|r| geo.id() == r).unwrap_or_default())
            .filter_map(|(i, obj)| {
                let collision = obj.ray_intersect(ray);
                assert!(collision.is_none() || !collision.map(|c| c.is_nan()).unwrap_or_default());
                collision.map(|c| (ray.solve(c), obj.normal(c), sphere_material[i], obj.id()))
            })
            .fold(Option::None, |acc, e| {
                Some(acc.map_or(
                    e,
                    |acc: (f32, Vec3, Material, u32)| {
                        if !e.0.is_nan() && acc.0 > e.0 {
                            e
                        } else {
                            acc
                        }
                    },
                ))
            });
        match (sphere_collision, mesh_collision) {
            (Some(s), Some(m)) => {
                if s.0 < m.0 {
                    Some(s)
                } else {
                    Some(m)
                }
            }
            (opt_a, opt_b) => opt_a.or(opt_b),
        }
    };

    let ray = Ray::new(*origin, *direction);
    intersect(ray, prev_obj_id).map(|(lerp, normal, material, collision_id)| {
        let collision = ray.lerp(lerp);
        let normal_vec = {
            let tmp = normal.normalize();
            tmp * (tmp.dot(-*direction)).signum()
        };
        let orthos = normal_vec.any_orthonormal_pair();

        assert!(!normal_vec.is_nan());

        let light_direction = (scene.light_position - collision).normalize();
        let shadow = {
            let sphere_shadow = sphere_objs
                .iter()
                .map(|obj| {
                    if obj.id() != collision_id {
                        return None;
                    }
                    let ray = Ray::new(collision, light_direction);
                    let new_collision = obj.ray_intersect(ray);
                    new_collision
                        .filter(|c| c.distance(collision) > 1e-6)
                        .filter(|c| ray.solve(*c) <= ray.solve(scene.light_position))
                })
                .any(|e| e.is_some());
            let mesh_shadow = mesh_objs
                .iter()
                .map(|obj| {
                    if obj.id() != collision_id {
                        return None;
                    }
                    let ray = Ray::new(collision, light_direction);
                    let new_collision = obj.ray_intersect(ray);
                    new_collision
                        .filter(|c| c.distance(collision) > 1e-6)
                        .filter(|c| ray.solve(*c) <= ray.solve(scene.light_position))
                })
                .any(|e| e.is_some());
            sphere_shadow || mesh_shadow
        };

        let diffuse = if !shadow {
            normal_vec.dot(light_direction).max(0f32)
        } else {
            0f32
        };
        let specular = if !shadow {
            normal_vec
                .dot((light_direction.normalize() + -direction.normalize()).normalize())
                .max(0.0)
                .powf(material.specular_exp)
        } else {
            0f32
        };

        let reflect_vec = reflect(*direction, normal_vec);
        let epsilon_factor = 1.0 / (2 * scene.resolution.x.max(scene.resolution.y)) as f32;
        let reflect_color: Vec3 = {
            // estimate sample count
            let samples =
                ((importance * (MAX_RECURSION_DEPTH - recursion_depth) as f32) as u32).max(1);
            (0..samples)
                .map(|_| {
                    let d = rng.gen_r2();
                    (d[0] * orthos.0 + d[1] * orthos.1) * epsilon_factor + collision
                })
                .filter_map(|rv| {
                    render_worker_inner(
                        worker_id,
                        &rv,
                        &reflect_vec,
                        scene,
                        sphere_objs,
                        sphere_material,
                        mesh_objs,
                        mesh_material,
                        importance * material.reflect_rate,
                        recursion_depth + 1,
                        Some(collision_id),
                    )
                })
                .map(|p| p / samples as f32)
                .sum::<Vec3>()
        };

        ((material.color * (material.ka + material.kd * diffuse) + Vec3::ONE * specular)
            * (1.0 - material.reflect_rate)
            + reflect_color * material.reflect_rate)
            .clamp(Vec3::ZERO, Vec3::ONE)
    })
}

#[spirv(compute(threads(8, 8, 1)))]
pub fn main_cs(
    #[spirv(global_invocation_id)] id: UVec3,
    #[spirv(uniform, descriptor_set = 1, binding = 0)] scene_metadata: &SceneMetadata,
    #[spirv(uniform, descriptor_set = 1, binding = 1)] sphere_objs: &[Sphere],
    #[spirv(uniform, descriptor_set = 1, binding = 2)] sphere_material: &[Material],
    #[spirv(uniform, descriptor_set = 1, binding = 3)] mesh_objs: &[Triangle],
    #[spirv(uniform, descriptor_set = 1, binding = 4)] mesh_material: &[Material],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] buf: &mut [Vec3],
) {
    if id.x > scene_metadata.resolution.x || id.y > scene_metadata.resolution.y {
        return;
    }
    let index = (id.y * scene_metadata.resolution.x + id.x) as usize;
    let world_pixel_width = scene_metadata.canvas_wv.length() / scene_metadata.resolution.x as f32;
    let world_pixel_height = scene_metadata.canvas_hv.length() / scene_metadata.resolution.y as f32;
    let pixel_coord = scene_metadata.canvas_corner
        + scene_metadata.canvas_wv.normalize() * world_pixel_width * (id.x as f32)
        + scene_metadata.canvas_hv.normalize() * world_pixel_height * (id.y as f32);
    let pixel: Option<Vec3> = render_worker_inner(
        index as u32,
        &scene_metadata.eye,
        &(pixel_coord - scene_metadata.eye),
        scene_metadata,
        sphere_objs,
        sphere_material,
        mesh_objs,
        mesh_material,
        1f32,
        0,
        None,
    );
    buf[index] = pixel.unwrap_or(scene_metadata.background.clone());
}
