use alkyd::{IRRIDESCANT_SHADER_HANDLE, PIXEL_ART_HANDLE};
use bevy::{
    color::palettes::css::WHITE,
    pbr::MaterialExtension,
    prelude::*,
    render::{
        render_asset::RenderAssets,
        render_resource::{ShaderType, AsBindGroup, AsBindGroupShaderType, ShaderRef },
        texture::GpuImage,
    },
};

#[derive(Asset, TypePath, AsBindGroup, Clone)]
#[uniform(0, PixelArtUniform)]
pub struct PixelArtMaterial {
    pub diffuse_color: Color,
    pub mettalic: f32,
    pub specular: f32,
    #[texture(1)]
    #[sampler(2)]
    pub diffuse_texture: Option<Handle<Image>>,
}

#[derive(Clone, ShaderType)]
pub struct PixelArtUniform {
    pub diffuse_color: Vec4,
    pub mettalic: f32,
    pub specular: f32,
}

impl Default for PixelArtMaterial {
    fn default() -> Self {
        Self {
            diffuse_color: WHITE.into(),
            mettalic: 0.0,
            specular: 0.0,
            diffuse_texture: None,
        }
    }
}

impl Material for PixelArtMaterial {
    fn fragment_shader() -> ShaderRef {
        PIXEL_ART_HANDLE.into()
    }
}

impl AsBindGroupShaderType<PixelArtUniform> for PixelArtMaterial {
    fn as_bind_group_shader_type(&self, _: &RenderAssets<GpuImage>) -> PixelArtUniform {
        PixelArtUniform {
            diffuse_color: self.diffuse_color.to_linear().to_vec4(),
            specular: self.specular,
            mettalic: self.mettalic
        }
    }
}
