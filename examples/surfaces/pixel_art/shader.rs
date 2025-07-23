use alkyd::PIXEL_ART_HANDLE;
use bevy::{
    color::palettes::css::{BLACK, WHITE},
    prelude::*,
    render::{
        render_asset::RenderAssets,
        render_resource::{AsBindGroup, AsBindGroupShaderType, ShaderRef, ShaderType},
        texture::GpuImage,
    },
};

#[derive(Asset, TypePath, AsBindGroup, Clone)]
#[uniform(0, PixelArtUniform)]
#[bind_group_data(PixelArtConfigKeys)]
pub struct PixelArtMaterial {
    pub diffuse_color: Color,
    pub outline_color: Color,
    pub quantize_steps: u32,
    pub bayer_count: u32,
    pub bayer_scale: f32,
    pub mettalic: f32,
    pub specular: f32,
    pub debug: bool,
    pub invert_color: bool,
}

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct PixelArtConfigKeys {
    pub debug: bool,
    pub invert_color: bool,
}

impl From<&PixelArtMaterial> for PixelArtConfigKeys {
    fn from(material: &PixelArtMaterial) -> Self {
        Self {
            debug: material.debug,
            invert_color: material.invert_color,
        }
    }
}

#[derive(Clone, ShaderType)]
pub struct PixelArtUniform {
    pub diffuse_color: Vec4,
    pub outline_color: Vec4,
    pub bayer_count: u32,
    pub bayer_scale: f32,
    pub quantize_steps: u32,
    pub mettalic: f32,
    pub specular: f32,
}

impl Default for PixelArtMaterial {
    fn default() -> Self {
        Self {
            diffuse_color: WHITE.into(),
            outline_color: BLACK.into(),
            bayer_count: 10,
            bayer_scale: 5.0,
            quantize_steps: 100,
            mettalic: 0.0,
            specular: 0.0,
            debug: false,
            invert_color: false,
        }
    }
}

impl Material for PixelArtMaterial {
    fn fragment_shader() -> ShaderRef {
        PIXEL_ART_HANDLE.into()
    }
    fn specialize(
        _pipeline: &bevy::pbr::MaterialPipeline<Self>,
        descriptor: &mut bevy::render::render_resource::RenderPipelineDescriptor,
        _layout: &bevy::render::mesh::MeshVertexBufferLayoutRef,
        key: bevy::pbr::MaterialPipelineKey<Self>,
    ) -> Result<(), bevy::render::render_resource::SpecializedMeshPipelineError> {
        if key.bind_group_data.debug {
            let fragment = descriptor.fragment.as_mut().unwrap();
            fragment.shader_defs.push("DEBUG".into());
        }

        if key.bind_group_data.invert_color {
            let fragment = descriptor.fragment.as_mut().unwrap();
            fragment.shader_defs.push("INVERT_COLOR".into());
        }

        Ok(())
    }
}

impl AsBindGroupShaderType<PixelArtUniform> for PixelArtMaterial {
    fn as_bind_group_shader_type(&self, _: &RenderAssets<GpuImage>) -> PixelArtUniform {
        PixelArtUniform {
            diffuse_color: self.diffuse_color.to_linear().to_vec4(),
            quantize_steps: self.quantize_steps,
            outline_color: self.outline_color.to_linear().to_vec4(),
            bayer_count: self.bayer_count,
            bayer_scale: self.bayer_scale,
            specular: self.specular,
            mettalic: self.mettalic,
        }
    }
}
