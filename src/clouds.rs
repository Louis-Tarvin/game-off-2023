use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::render_resource::{AsBindGroup, ShaderRef},
};

#[derive(AsBindGroup, Debug, Clone, TypeUuid, TypePath)]
#[uuid = "3b72ad2f-5f0d-4a2c-b53f-e82342706584"]
pub struct CloudMaterial {
    #[uniform(0)]
    pub color_a: Color,
    #[uniform(1)]
    pub color_b: Color,
}

impl Material for CloudMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/cloud_material.wgsl".into()
    }
    fn vertex_shader() -> ShaderRef {
        "shaders/cloud_material.wgsl".into()
    }
    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}
