use crate::{
    compute::{
        traits::{ComputeShader, ComputeWorker},
        worker::AppComputeWorker,
        worker_builder::AppComputeWorkerBuilder,
    },
    NOISE_FUNCTIONS_HANDLE, NOISE_GEN_UTILS_HANDLE, SIMPLEX_4D_HANDLE, SIMPLEX_HANDLE,
    VORONOI_SHADER_HANDLE,
};
use bevy::{
    asset::load_internal_asset,
    prelude::*,
    render::render_resource::{ShaderRef, ShaderType},
};
use systems::{read_data, run_worker};

pub mod systems;

#[derive(TypePath)]
pub struct VoronoiShader;

impl ComputeShader for VoronoiShader {
    fn shader() -> ShaderRef {
        VORONOI_SHADER_HANDLE.into()
    }
}

#[derive(ShaderType)]
struct Properties {
    cell_number: f32,
}

#[derive(Resource)]
pub struct VoronoiWorker;

impl ComputeWorker for VoronoiWorker {
    fn build(world: &mut World) -> AppComputeWorker<Self> {
        AppComputeWorkerBuilder::new(world)
            .add_staging("centroids", &[Vec4::ZERO; 100])
            .add_pass::<VoronoiShader>([10, 10, 1], &["centroids"])
            .one_shot()
            .build()
    }
}

pub struct UtilitiesPlugin;

impl Plugin for UtilitiesPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            VORONOI_SHADER_HANDLE,
            "../../assets/noise.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            NOISE_FUNCTIONS_HANDLE,
            "../../assets/shader_utils/general.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            SIMPLEX_HANDLE,
            "../../assets/shader_utils/simplex_3d.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            SIMPLEX_4D_HANDLE,
            "../../assets/shader_utils/simplex_4d.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            NOISE_GEN_UTILS_HANDLE,
            "../../assets/shader_utils/noise_gen.wgsl",
            Shader::from_wgsl
        );
        app.add_systems(Update, (read_data, run_worker));
    }
}
