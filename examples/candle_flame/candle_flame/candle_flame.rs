use bevy::{
    color::palettes::css::BLUE,
    prelude::*,
    render::{
        render_asset::RenderAssets,
        render_resource::{AsBindGroup, AsBindGroupShaderType, Face, ShaderRef, ShaderType},
        texture::GpuImage,
    },
};

use super::GALAXYFOG_SHADER_HANDLE;

#[derive(Asset, TypePath, AsBindGroup, Clone)]
#[uniform(0, CandleUniform)]
pub struct CandleFlameMaterial {
    pub diffuse_color: Color,
    pub center: Vec3,
    pub radius: f32,
    pub steps: u32,
    pub precision: f32,
    #[uniform(1)]
    pub props: NoiseProperties,
}

impl Default for CandleFlameMaterial {
    fn default() -> Self {
        Self {
            diffuse_color: Color::Srgba(BLUE),
            center: Vec3::ZERO,
            radius: 2.0,
            steps: 30,
            precision: 25.0,
            props: NoiseProperties::default(),
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

#[derive(Clone, ShaderType)]
pub struct NoiseProperties {
    pub octaves: i32,
    pub lacunarity: f32,
    pub gain: f32,
    pub amplitude: f32,
    pub frequency: f32,
}

impl Default for NoiseProperties {
    fn default() -> Self {
        Self {
            octaves: 4,
            lacunarity: 2.0,
            gain: 0.03,
            amplitude: 1.0,
            frequency: 1.0,
        }
    }
}

impl Material for CandleFlameMaterial {
    fn fragment_shader() -> ShaderRef {
        "example_assets/candle_flame.wgsl".into()
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
        descriptor.primitive.cull_mode = Some(Face::Front);

        Ok(())
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

impl AsBindGroupShaderType<NoiseProperties> for CandleFlameMaterial {
    fn as_bind_group_shader_type(&self, _: &RenderAssets<GpuImage>) -> NoiseProperties {
        NoiseProperties {
            octaves: self.props.octaves,
            lacunarity: self.props.lacunarity,
            gain: self.props.gain,
            amplitude: self.props.amplitude,
            frequency: self.props.frequency,
        }
    }
}
