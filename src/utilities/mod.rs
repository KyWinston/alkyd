use crate::{
    // compute::{traits::{ComputeShader, ComputeWorker}, worker::AppComputeWorker, worker_builder::AppComputeWorkerBuilder},
    GLOBAL_VALUES_HANDLE,
    NOISE_FUNCTIONS_HANDLE,
    NOISE_GEN_UTILS_HANDLE,
    SIMPLEX_4D_HANDLE,
    SIMPLEX_HANDLE,
    TEX_GEN_HANDLE,
    VORONOI_SHADER_HANDLE,
};
use bevy::{asset::load_internal_asset, prelude::*, render::render_resource::ShaderType};

// use systems::generate_texture;

// use self::systems::{read_data, run_worker};

pub mod systems;

#[derive(TypePath)]
pub struct VoronoiShader;

#[derive(TypePath)]
pub struct TexGenerator;

// impl ComputeShader for VoronoiShader {
//     fn shader() -> ShaderRef {
//         VORONOI_SHADER_HANDLE.into()
//     }
// }

// impl ComputeShader for TexGenerator {
//     fn shader() -> ShaderRef {
//         TEX_GEN_HANDLE.into()
//     }
// }

#[derive(ShaderType)]
struct Properties {
    cell_number: f32,
}

#[derive(Resource)]
pub struct VoronoiWorker;

#[derive(Resource)]
pub struct TexGenWorker;

// impl ComputeWorker for VoronoiWorker {
//     fn build(world: &mut World) -> AppComputeWorker<Self> {
//         AppComputeWorkerBuilder::new(world)
//             .add_staging("centroids", &[Vec4::ZERO; 100])
//             .add_pass::<VoronoiShader>([10, 10, 1], &["centroids"])
//             .one_shot()
//             .build()
//     }
// }

// impl ComputeWorker for TexGenWorker {
//     fn build(world: &mut World) -> AppComputeWorker<Self> {
//         AppComputeWorkerBuilder::new(world)
//             .add_storage("slot_a_input", &[0.0; 100])
//             .add_staging("slot_a_output", &[0f32; 100])
//             .add_pass::<TexGenerator>([10, 10, 10], &["slot_a_input", "slot_a_output"])
//             .one_shot()
//             .build()
//     }
// }

pub struct UtilitiesPlugin;

impl Plugin for UtilitiesPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            GLOBAL_VALUES_HANDLE,
            "../../assets/shader_utils/consts/globs.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            VORONOI_SHADER_HANDLE,
            "../../assets/noise.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            TEX_GEN_HANDLE,
            "../../assets/shader_utils/tex_gen.wgsl",
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
        // app.add_systems(Update, (read_data, generate_texture, run_worker));
    }
}
