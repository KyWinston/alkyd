use bevy::prelude::*;
use utilities::UtilitiesPlugin;
use workers::WorkersPlugin;

pub mod components;
pub mod compute;
pub mod raymarch_quad;
pub mod showcase;
pub mod utilities;
pub mod workers;
// pub mod grass;

// pub(crate) const IRRIDESCANT_SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(1208033355542926744);
// pub(crate) const PAINTERLY_SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(1908033355537029744);
pub(crate) const NOISE_FUNCTIONS_HANDLE: Handle<Shader> = Handle::weak_from_u128(94071345065644201137);
pub(crate) const NOISE_GEN_UTILS_HANDLE: Handle<Shader> = Handle::weak_from_u128(94071345065837501137);
pub(crate) const NOISE_COMPUTE_HANDLE: Handle<Shader> = Handle::weak_from_u128(24071345358763528837);
pub(crate) const SIMPLEX_HANDLE: Handle<Shader> = Handle::weak_from_u128(34071823065847501137);
pub(crate) const SIMPLEX_4D_HANDLE: Handle<Shader> = Handle::weak_from_u128(34071823465847412137);
pub(crate) const GLOBAL_VALUES_HANDLE: Handle<Shader> = Handle::weak_from_u128(34071828566847501137);
pub(crate) const TEX_GEN_HANDLE: Handle<Shader> = Handle::weak_from_u128(1508033515847412137);
pub(crate) const SPRITELY_HANDLE: Handle<Shader> = Handle::weak_from_u128(1708343655899895001229744);
pub(crate) const SOBEL_HANDLE: Handle<Shader> = Handle::weak_from_u128(1748343643674965476001229744);
pub(crate) const BLEND_MODES_HANDLE: Handle<Shader> = Handle::weak_from_u128(184229632462351882081599150);
pub(crate) const CONVERTERS_HANDLE: Handle<Shader> = Handle::weak_from_u128(522521912971636216150179);
// pub(crate) const GRASS_SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(195_094_223_228_228_028_086_047_086_167_255_040_126);

pub struct AlkydPlugin;

impl Plugin for AlkydPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((UtilitiesPlugin, WorkersPlugin));
    }
}
