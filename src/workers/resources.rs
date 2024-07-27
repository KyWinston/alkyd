use bevy::{
    prelude::*,
    render::{
        extract_resource::ExtractResource,
        render_resource::{
            BindGroup, BindGroupLayout, BindGroupLayoutEntry, BindingType, Buffer, CachedComputePipelineId, ShaderStages, StorageTextureAccess, TextureFormat, TextureViewDimension
        },
        renderer::RenderDevice,
    },
};
use crevice::std140::AsStd140;

#[derive(Clone, ExtractResource, Resource, Debug)]
pub struct Canvas {
    pub width: u32,
    pub height: u32,
    pub borders: f32,
    pub position: Vec3,
}

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
                NoiseGeneratorPipeline::make_texture_layout(4),
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

#[derive(Resource, Clone, Copy)]
pub struct CommonUniform {
    pub i_resolution: Vec2,
    pub changed_window_size: f32,
    pub padding0: f32,

    pub i_time: f32,
    pub i_time_delta: f32,
    pub i_frame: f32,
    pub i_sample_rate: f32, // sound sample rate

    pub i_mouse: Vec4,

    pub i_channel_time: Vec4,
    pub i_channel_resolution: Vec4,
    pub i_date: Vec4,
}

impl CommonUniform {
    pub fn new() -> Self {
        Self {
            i_resolution: Vec2::ZERO,
            changed_window_size: 0.0,
            padding0: 0.0,

            i_time: 0.,
            i_time_delta: 0.,
            i_frame: 0.,
            i_sample_rate: 0.,

            i_mouse: Vec4::ZERO,

            i_channel_time: Vec4::ZERO,
            i_channel_resolution: Vec4::ZERO,
            i_date: Vec4::ZERO,
        }
    }

    pub fn into_crevice(&self) -> CommonUniformCrevice {
        CommonUniformCrevice {
            i_resolution: crevice::std140::Vec2 {
                x: self.i_resolution.x,
                y: self.i_resolution.y,
            },

            changed_window_size: self.changed_window_size,
            padding0: self.padding0,

            i_time: self.i_time,
            i_time_delta: self.i_time_delta,
            i_frame: self.i_frame,
            i_sample_rate: self.i_sample_rate,

            i_mouse: crevice::std140::Vec4 {
                x: self.i_mouse.x,
                y: self.i_mouse.y,
                z: self.i_mouse.z,
                w: self.i_mouse.w,
            },

            i_channel_time: crevice::std140::Vec4 {
                x: self.i_channel_time.x,
                y: self.i_channel_time.y,
                z: self.i_channel_time.z,
                w: self.i_channel_time.w,
            },
            i_channel_resolution: crevice::std140::Vec4 {
                x: self.i_channel_resolution.x,
                y: self.i_channel_resolution.y,
                z: self.i_channel_resolution.z,
                w: self.i_channel_resolution.w,
            },
            i_date: crevice::std140::Vec4 {
                x: self.i_date.x,
                y: self.i_date.y,
                z: self.i_date.z,
                w: self.i_date.w,
            },
        }
    }
}

#[derive(Clone, Copy, AsStd140)]
pub struct CommonUniformCrevice {
    pub i_resolution: crevice::std140::Vec2,
    pub changed_window_size: f32,
    pub padding0: f32,

    pub i_time: f32,
    pub i_time_delta: f32,
    pub i_frame: f32,
    pub i_sample_rate: f32, // sound sample rate

    pub i_mouse: crevice::std140::Vec4,

    pub i_channel_time: crevice::std140::Vec4,
    pub i_channel_resolution: crevice::std140::Vec4,
    pub i_date: crevice::std140::Vec4,
}

#[derive(Deref, Clone, Resource)]
pub struct ExtractedUniform(pub CommonUniformCrevice);

impl ExtractResource for ExtractedUniform {
    type Source = CommonUniform;

    fn extract_resource(common_uniform: &Self::Source) -> Self {
        ExtractedUniform(common_uniform.into_crevice().clone())
    }
}

#[derive(Deref, Clone, Resource)]
pub struct CommonUniformMeta {
    pub buffer: Buffer,
}