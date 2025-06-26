use bevy::{
    color::palettes::css::BLUE,
    prelude::*,
    render::{
        render_asset::RenderAssets,
        render_resource::{AsBindGroup, AsBindGroupShaderType, ShaderRef, ShaderType},
        texture::GpuImage,
    },
};

#[derive(Asset, TypePath, AsBindGroup, Clone)]
#[uniform(0, CandleUniform)]
pub struct CandleFlameMaterial {
    pub diffuse_color: Color,
    pub center: Vec3,
    pub radius: f32,
    pub steps: u32,
    pub precision: f32,
    #[texture(1)]
    #[sampler(2)]
    pub fbm: Option<Handle<Image>>,
    #[texture(3)]
    #[sampler(4)]
    pub fbm_2: Option<Handle<Image>>,
}

impl Default for CandleFlameMaterial {
    fn default() -> Self {
        Self {
            diffuse_color: Color::Srgba(BLUE),
            center: Vec3::ZERO,
            radius: 2.0,
            steps: 30,
            precision: 25.0,
            fbm: None,
            fbm_2: None,
        }
    }
}

#[derive(Clone, ShaderType)]
pub struct CandleUniform {
    pub diffuse_color: Vec4,
    pub center: Vec3,
    pub radius: f32,
    pub steps: u32,
    pub precision: f32,
}

impl Material for CandleFlameMaterial {
    fn fragment_shader() -> ShaderRef {
        "example_assets/candle_flame.wgsl".into()
    }
    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

impl AsBindGroupShaderType<CandleUniform> for CandleFlameMaterial {
    fn as_bind_group_shader_type(&self, _: &RenderAssets<GpuImage>) -> CandleUniform {
        CandleUniform {
            diffuse_color: self.diffuse_color.to_linear().to_vec4(),
            center: self.center,
            radius: self.radius,
            steps: self.steps,
            precision: self.precision,
        }
    }
}
