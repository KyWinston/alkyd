use bevy::{
    prelude::*,
    render::{
        render_asset::RenderAssets,
        render_resource::{AsBindGroup, AsBindGroupShaderType, ShaderRef, ShaderType},
        texture::GpuImage,
    },
};

use crate::PAINTERLY_SHADER_HANDLE;

#[derive(Clone, ShaderType)]
pub struct Kernel {
    pub kernel: [Vec4; 1000],
}

#[derive(Asset, TypePath, AsBindGroup, Clone)]
#[uniform(0, PatternWfcUniform)]
pub struct PatternGeneratorWfc {
    pub kernel: Kernel,
    pub kernel_width: i32,
    pub mirror: bool,
    pub swap: bool,
    pub wrap: (bool, bool),
}

impl Default for PatternGeneratorWfc {
    fn default() -> Self {
        Self {
            kernel: Kernel {
                kernel: [Vec4::ZERO; 1000],
            },
            kernel_width: 10,
            mirror: true,
            swap: true,
            wrap: (true, true),
        }
    }
}

#[derive(Clone, ShaderType)]
pub struct PatternWfcUniform {
    pub kernel_width: i32,
    pub mirror: i32,
    pub swap: i32,
    pub wrap_x: i32,
    pub wrap_y: i32,
}

impl Material for PatternGeneratorWfc {
    fn fragment_shader() -> ShaderRef {
        PAINTERLY_SHADER_HANDLE.into()
    }
}

impl AsBindGroupShaderType<PatternWfcUniform> for PatternGeneratorWfc {
    fn as_bind_group_shader_type(&self, _: &RenderAssets<GpuImage>) -> PatternWfcUniform {
        PatternWfcUniform {
            kernel_width: self.kernel_width,
            mirror: self.mirror.into(),
            swap: self.swap.into(),
            wrap_x: self.wrap.0.into(),
            wrap_y: self.wrap.1.into(),
        }
    }
}
