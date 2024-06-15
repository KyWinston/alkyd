use bevy::{
    color::palettes::css::BLUE,
    prelude::*,
    render::{
        render_asset::RenderAssets,
        render_resource::{AsBindGroup, AsBindGroupShaderType, ShaderRef, ShaderType},
        texture::GpuImage,
    },
};

use crate::GALAXYFOG_SHADER_HANDLE;

#[derive(Asset, TypePath, AsBindGroup, Clone)]
#[uniform(0, GalaxyUniform)]
pub struct GalaxyFogMaterial {
    pub diffuse_color: Color,
    pub center: Vec3,
    pub radius: f32,
}

impl Default for GalaxyFogMaterial {
    fn default() -> Self {
        Self {
            diffuse_color: Color::Srgba(BLUE),
            center: Vec3::ZERO,
            radius: 50.0,
        }
    }
}

#[derive(Clone, ShaderType)]
pub struct GalaxyUniform {
    pub diffuse_color: Vec4,
    pub center: Vec3,
    pub radius: f32,
}

impl Material for GalaxyFogMaterial {
    fn fragment_shader() -> ShaderRef {
        GALAXYFOG_SHADER_HANDLE.into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }

    fn specialize(
        _pipeline: &bevy::pbr::MaterialPipeline<Self>,
        descriptor: &mut bevy::render::render_resource::RenderPipelineDescriptor,
        _layout: &bevy::render::mesh::MeshVertexBufferLayoutRef,
        _key: bevy::pbr::MaterialPipelineKey<Self>,
    ) -> Result<(), bevy::render::render_resource::SpecializedMeshPipelineError> {
        descriptor.primitive.cull_mode = Some(wgpu::Face::Front);

        Ok(())
    }
}

impl AsBindGroupShaderType<GalaxyUniform> for GalaxyFogMaterial {
    fn as_bind_group_shader_type(&self, _: &RenderAssets<GpuImage>) -> GalaxyUniform {
        GalaxyUniform {
            diffuse_color: self.diffuse_color.linear().to_vec4(),
            center: self.center,
            radius: self.radius,
        }
    }
}
