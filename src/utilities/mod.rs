use crate::{
    GLOBAL_VALUES_HANDLE, NOISE_FUNCTIONS_HANDLE, NOISE_GEN_UTILS_HANDLE, SIMPLEX_4D_HANDLE,
    SIMPLEX_HANDLE, SPRITELY_HANDLE, TEX_GEN_HANDLE,
};
use bevy::{asset::load_internal_asset, prelude::*};
// use stepper::{DebugSchedule, SteppingPlugin};

#[derive(TypePath)]
pub struct VoronoiShader;

#[derive(TypePath)]
pub struct TexGenerator;

#[derive(Resource)]
pub struct VoronoiWorker;

#[derive(Resource)]
pub struct TexGenWorker;

pub struct UtilitiesPlugin;
// pub mod stepper;

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
            TEX_GEN_HANDLE,
            "../../assets/shader_utils/tex_gen.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            SPRITELY_HANDLE,
            "../../assets/spritely.wgsl",
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
    }
}
