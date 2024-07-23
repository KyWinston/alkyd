use bevy::{
    pbr::MaterialExtension, prelude::*, render::{
        render_asset::RenderAssets,
        render_resource::{AsBindGroup, AsBindGroupShaderType, ShaderRef, ShaderType},
        texture::GpuImage,
    }
};

use crate::IRRIDESCANT_SHADER_HANDLE;

#[derive(Asset, TypePath, AsBindGroup, Clone)]
#[uniform(100, IrridescantUniform)]
pub struct IrridescantMaterial {
    pub ior: f32,
}

impl Default for IrridescantMaterial {
    fn default() -> Self {
        Self { ior: 1.33 }
    }
}

#[derive(Clone, ShaderType)]
pub struct IrridescantUniform {
    pub ior: f32,
}

impl MaterialExtension for IrridescantMaterial {
    fn fragment_shader() -> ShaderRef {
        IRRIDESCANT_SHADER_HANDLE.into()
    }
}

impl AsBindGroupShaderType<IrridescantUniform> for IrridescantMaterial {
    fn as_bind_group_shader_type(&self, _: &RenderAssets<GpuImage>) -> IrridescantUniform {
        IrridescantUniform { ior: self.ior }
    }
}
