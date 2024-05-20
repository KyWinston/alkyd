
use bevy::prelude::{AssetEvent, Assets, EventReader, Res, ResMut, Shader};

use pipeline_cache::AppPipelineCache;

pub mod error;
pub mod pipeline_cache;
pub mod plugin;
pub mod traits;
pub mod worker;
pub mod worker_builder;

// Since these are always used when using this crate
pub use bevy::render::render_resource::{ShaderRef, ShaderType};

pub(crate) fn process_pipeline_queue_system(mut pipeline_cache: ResMut<AppPipelineCache>) {
    pipeline_cache.process_queue();
}

pub(crate) fn extract_shaders(
    mut pipeline_cache: ResMut<AppPipelineCache>,
    shaders: Res<Assets<Shader>>,
    mut events: EventReader<AssetEvent<Shader>>,
) {
    for event in events.read() {
        match event {
            AssetEvent::Added { id: shader_id } | AssetEvent::Modified { id: shader_id } => {
                if let Some(shader) = shaders.get(shader_id.clone()) {
                    pipeline_cache.set_shader(shader_id, shader);
                }
            }
            AssetEvent::Removed { id: shader_id } => pipeline_cache.remove_shader(shader_id),
            AssetEvent::LoadedWithDependencies { id: shader_id } => {
                if let Some(shader) = shaders.get(shader_id.clone()) {
                    pipeline_cache.set_shader(shader_id, shader);
                }
            }
            _ => {}
        }
    }
}
