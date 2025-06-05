use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

#[derive(Asset, Reflect, AsBindGroup, Debug, Clone, Default)]
pub struct OrbMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub texture: Handle<Image>,
    #[uniform(2)]
    pub color_a: LinearRgba,
    #[uniform(3)]
    pub color_b: LinearRgba,
}

impl Material2d for OrbMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/orb.wgsl".into()
    }
    fn alpha_mode(&self) -> bevy::sprite::AlphaMode2d {
        bevy::sprite::AlphaMode2d::Blend
    }
}
