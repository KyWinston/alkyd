use std::borrow::Cow;

use bevy::{
    prelude::*,
    render::{
        RenderSet,
        extract_resource::ExtractResourcePlugin,
        render_resource::{
            AsBindGroup, BindGroup, BindGroupLayout, BindGroupLayoutEntries,
            CachedComputePipelineId, PipelineCache, ShaderStages, StorageTextureAccess,
            TextureFormat,
            binding_types::{texture_storage_2d, uniform_buffer},
        },
        renderer::RenderDevice,
    },
};
use bevy_app_compute::prelude::{
    AppComputePlugin, AppComputeWorker, AppComputeWorkerBuilder, AppComputeWorkerPlugin,
    ComputeShader, ComputeWorker, ShaderRef,
};

use crate::TEX_GEN_HANDLE;

pub struct WorkersPlugin;

pub const WORKGROUP_SIZE: u32 = 64;

impl Plugin for WorkersPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((AppComputePlugin));
    }
}
