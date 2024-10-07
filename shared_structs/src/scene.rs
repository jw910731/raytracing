use spirv_std::glam::{UVec2, Vec3};

#[derive(Copy, Clone)]
pub struct SceneMetadata {
    pub eye: Vec3,
    pub canvas_wv: Vec3,
    pub canvas_hv: Vec3,
    pub canvas_corner: Vec3,
    pub background: Vec3,
    pub light_position: Vec3,
    pub resolution: UVec2,
    pub antialiasing: u32,
    pub padding: u32,
}

#[derive(Clone, Copy, Default)]
pub struct Material {
    pub color: Vec3,
    pub ka: f32,
    pub kd: f32,
    pub ks: f32,
    pub specular_exp: f32,
    pub reflect_rate: f32,
}
