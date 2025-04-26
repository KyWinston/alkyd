use crate::{
    BLEND_MODES_HANDLE, CONVERTERS_HANDLE, GLOBAL_VALUES_HANDLE, NOISE_COMPUTE_HANDLE,
    NOISE_FUNCTIONS_HANDLE, NOISE_GEN_UTILS_HANDLE, SIMPLEX_4D_HANDLE, SIMPLEX_HANDLE,
    SOBEL_HANDLE, SPRITELY_HANDLE, TEX_GEN_HANDLE,
};

use bevy::{asset::load_internal_asset, prelude::*};

pub struct UtilitiesPlugin;

impl Plugin for UtilitiesPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            GLOBAL_VALUES_HANDLE,
            "../../assets/shader_utils/consts/globs.wesl",
            Shader::from_wesl
        );

        load_internal_asset!(
            app,
            NOISE_COMPUTE_HANDLE,
            "../../assets/shader_utils/noise/noise_compute.wesl",
            Shader::from_wesl
        );
        load_internal_asset!(
            app,
            TEX_GEN_HANDLE,
            "../../assets/shader_utils/noise/tex_gen.wesl",
            Shader::from_wesl
        );
        load_internal_asset!(
            app,
            SPRITELY_HANDLE,
            "../../assets/shader_utils/sprite_rotation.wesl",
            Shader::from_wesl
        );
        load_internal_asset!(
            app,
            NOISE_FUNCTIONS_HANDLE,
            "../../assets/shader_utils/general.wesl",
            Shader::from_wesl
        );
        load_internal_asset!(
            app,
            SIMPLEX_HANDLE,
            "../../assets/shader_utils/noise/simplex_3d.wesl",
            Shader::from_wesl
        );
        load_internal_asset!(
            app,
            SIMPLEX_4D_HANDLE,
            "../../assets/shader_utils/noise/simplex_4d.wesl",
            Shader::from_wesl
        );
        load_internal_asset!(
            app,
            NOISE_GEN_UTILS_HANDLE,
            "../../assets/shader_utils/noise/noise_gen.wesl",
            Shader::from_wesl
        );
        load_internal_asset!(
            app,
            SOBEL_HANDLE,
            "../../assets/shader_utils/filters/sobel.wesl",
            Shader::from_wesl
        );
        load_internal_asset!(
            app,
            BLEND_MODES_HANDLE,
            "../../assets/shader_utils/color_functions/blend_modes.wesl",
            Shader::from_wesl
        );
        load_internal_asset!(
            app,
            CONVERTERS_HANDLE,
            "../../assets/shader_utils/color_functions/convert.wesl",
            Shader::from_wesl
        );
    }
}
