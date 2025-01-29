use bevy::prelude::*;
use fluid::FluidPlugin;
use utilities::UtilitiesPlugin;
use workers::WorkersPlugin;

pub mod components;
pub mod compute;
pub mod fluid;
pub mod showcase;
pub mod tex_gen;
pub mod utilities;
pub mod workers;
// pub mod grass;

pub const IRRIDESCANT_SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(1208033355542926744);
pub const PAINTERLY_SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(1908033355537029744);
pub const NOISE_FUNCTIONS_HANDLE: Handle<Shader> = Handle::weak_from_u128(94071345065644201137);
pub const NOISE_GEN_UTILS_HANDLE: Handle<Shader> = Handle::weak_from_u128(94071345065837501137);
pub const NOISE_COMPUTE_HANDLE: Handle<Shader> = Handle::weak_from_u128(24071345358763528837);
pub const SIMPLEX_HANDLE: Handle<Shader> = Handle::weak_from_u128(34071823065847501137);
pub const SIMPLEX_4D_HANDLE: Handle<Shader> = Handle::weak_from_u128(34071823465847412137);
pub const GLOBAL_VALUES_HANDLE: Handle<Shader> = Handle::weak_from_u128(34071828566847501137);
pub const TEX_GEN_HANDLE: Handle<Shader> = Handle::weak_from_u128(1508033515847412137);
pub const SPRITELY_HANDLE: Handle<Shader> = Handle::weak_from_u128(1708343655899895001229744);
pub const SOBEL_HANDLE: Handle<Shader> = Handle::weak_from_u128(1748343643674965476001229744);
pub const BLEND_MODES_HANDLE: Handle<Shader> = Handle::weak_from_u128(184229632462351882081599150);
pub const CONVERTERS_HANDLE: Handle<Shader> = Handle::weak_from_u128(522521912971636216150179);
pub const FLUID_SIM_HANDLE: Handle<Shader> = Handle::weak_from_u128(38257092369836390268233459);
pub const FLUID_SIM_SECOND_PASS_HANDLE: Handle<Shader> = Handle::weak_from_u128(382579689756468233459);
pub const FLUID_CONSTS: Handle<Shader> = Handle::weak_from_u128(6243576094856749806743908);

pub struct AlkydPlugin;

impl Plugin for AlkydPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((UtilitiesPlugin, WorkersPlugin, FluidPlugin));
    }
}
