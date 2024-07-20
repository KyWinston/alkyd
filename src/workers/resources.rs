use bevy::{
    prelude::*,
    render::{
        extract_resource::ExtractResource,
        render_resource::{
            BindGroup, BindGroupLayout, BindGroupLayoutEntry, BindingType, CachedComputePipelineId,
            SamplerBindingType, ShaderStages, StorageTextureAccess, TextureFormat,
            TextureViewDimension,
        },
        renderer::RenderDevice,
    },
};

#[derive(Resource, Clone, ExtractResource)]
pub struct NoiseImage(pub Handle<Image>);

#[derive(Clone, Resource, ExtractResource)]
pub struct ShaderHandles {
    pub image_shader: Handle<Shader>,
    pub texture_a_shader: Handle<Shader>,
    pub texture_b_shader: Handle<Shader>,
    pub texture_c_shader: Handle<Shader>,
    pub texture_d_shader: Handle<Shader>,
}

#[derive(Resource)]
pub struct NoiseGeneratorBindGroup {
    pub noise_gen_bind_group: BindGroup,
    pub init_pipeline: CachedComputePipelineId,
    pub update_pipeline: CachedComputePipelineId,
}

#[derive(Resource)]
pub struct NoiseGeneratorPipeline {
    pub texture_bind_group_layout: BindGroupLayout,
    pub texture_group_layout: BindGroupLayout,
}

impl NoiseGeneratorPipeline {
    pub fn make_texture_layout(binding: u32) -> BindGroupLayoutEntry {
        BindGroupLayoutEntry {
            binding,
            visibility: ShaderStages::COMPUTE,
            ty: BindingType::StorageTexture {
                access: StorageTextureAccess::ReadWrite,
                format: TextureFormat::Rgba32Float,
                view_dimension: TextureViewDimension::D2,
            },
            count: None,
        }
    }
    pub fn new(render_device: &RenderDevice) -> Self {
        let abcd_group_layout = render_device.create_bind_group_layout(
            Some("abcd_layout"),
            &[
                NoiseGeneratorPipeline::make_texture_layout(0),
                NoiseGeneratorPipeline::make_texture_layout(1),
                NoiseGeneratorPipeline::make_texture_layout(2),
                NoiseGeneratorPipeline::make_texture_layout(3),
            ],
        );

        let main_image_group_layout = render_device.create_bind_group_layout(
            Some("main_layout"),
            &[
                NoiseGeneratorPipeline::make_texture_layout(0),
                NoiseGeneratorPipeline::make_texture_layout(1),
                NoiseGeneratorPipeline::make_texture_layout(2),
                NoiseGeneratorPipeline::make_texture_layout(3),
                BindGroupLayoutEntry {
                    binding: 4,
                    visibility: ShaderStages::COMPUTE,
                    ty: BindingType::StorageTexture {
                        access: StorageTextureAccess::ReadWrite,
                        format: TextureFormat::Rgba32Float,
                        view_dimension: TextureViewDimension::D2,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 5,
                    visibility: ShaderStages::COMPUTE,
                    ty: BindingType::Sampler(SamplerBindingType::Filtering),
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 6,
                    visibility: ShaderStages::COMPUTE,
                    ty: BindingType::Sampler(SamplerBindingType::Filtering),
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 10,
                    visibility: ShaderStages::COMPUTE,
                    ty: BindingType::Sampler(SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        );

        NoiseGeneratorPipeline {
            texture_bind_group_layout: main_image_group_layout,
            texture_group_layout: abcd_group_layout,
        }
    }
}

impl FromWorld for NoiseGeneratorPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();
        NoiseGeneratorPipeline::new(render_device)
    }
}
