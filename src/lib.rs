use bevy::{asset::weak_handle, prelude::*};
// use terrain::TerrainPlugin;
use utilities::UtilitiesPlugin;
use workers::WorkersPlugin;

pub mod components;
pub mod compute;
// pub mod fluid;
pub mod showcase;
// pub mod terrain;
pub mod custom_materials;
pub mod tex_gen;
pub mod utilities;
pub mod uv_paint;
pub mod workers;

pub const UV_PAINT_HANDLE: Handle<Shader> = weak_handle!("38279689-e0e5-42b4-af48-601ceb8da1b6");
pub const IRRIDESCANT_SHADER_HANDLE: Handle<Shader> =
    weak_handle!("16ccd8db-afe4-4afe-afe6-3b77bf067d02");
pub const PAINTERLY_SHADER_HANDLE: Handle<Shader> =
    weak_handle!("3411c9e9-4525-4810-8e93-7c5459439268");
pub const NOISE_FUNCTIONS_HANDLE: Handle<Shader> =
    weak_handle!("0412d402-9088-4f9c-883f-09988461dd89");
pub const NOISE_GEN_UTILS_HANDLE: Handle<Shader> =
    weak_handle!("b2f08cae-bf74-4d3d-9ee8-79cfb0a9876f");
pub const NOISE_COMPUTE_HANDLE: Handle<Shader> =
    weak_handle!("ef6fcd75-2564-4de1-be3b-d1961493eaf5");
pub const SIMPLEX_HANDLE: Handle<Shader> = weak_handle!("469c82b5-189e-4169-a0ec-10d86df9d218");
pub const SIMPLEX_4D_HANDLE: Handle<Shader> = weak_handle!("d27e4e75-8595-4def-a328-b92691248c05");
pub const GLOBAL_VALUES_HANDLE: Handle<Shader> =
    weak_handle!("066b8755-fd2a-41fc-b15a-c407c26a18a7");
pub const TEX_GEN_HANDLE: Handle<Shader> = weak_handle!("eaca6b3a-aec5-4f39-a665-ef2ac4cbf13c");
pub const SPRITELY_HANDLE: Handle<Shader> = weak_handle!("612937a1-e121-44be-803f-542ea29bd82e");
pub const SOBEL_HANDLE: Handle<Shader> = weak_handle!("4f96cdce-169d-499a-a68c-8292af0015c1");
pub const BLEND_MODES_HANDLE: Handle<Shader> = weak_handle!("62a9a1f1-35e8-464d-a0ec-27ee957ce478");
pub const CONVERTERS_HANDLE: Handle<Shader> = weak_handle!("547cd0a7-a4ed-4aa8-aff8-857ef4eea9fc");
pub const TERRAIN_SHADER_HANDLE: Handle<Shader> =
    weak_handle!("5f74e6d4-b59b-465b-8e88-8e12d8924715");
pub const FLUID_SIM_HANDLE: Handle<Shader> = weak_handle!("d14aa666-f4ff-430b-9fac-9845e54dbf45");
pub const FLUID_SIM_SECOND_PASS_HANDLE: Handle<Shader> =
    weak_handle!("8d3a02f2-be73-4d6b-a7a2-5d0cd71aa38e");
pub const FLUID_CONSTS: Handle<Shader> = weak_handle!("a06406b6-60d1-4533-ae71-c123cae79f01");

pub struct AlkydPlugin;

impl Plugin for AlkydPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((UtilitiesPlugin, WorkersPlugin));
    }
}
