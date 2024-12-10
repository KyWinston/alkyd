use bevy::{pbr::ExtendedMaterial, prelude::*};
use bevy_easy_compute::prelude::AppComputePlugin;
#[cfg(feature = "editor")]
use editor::EditorPlugin;
use materials::{
    galaxyfog::{galaxy::GalaxyFogMaterial, GalaxyFogPlugin},
    irridescant::{shader::IrridescantMaterial, IrridescantMaterialPlugin},
    painterly::{resources::PainterlyMaterial, MaterialSwatchPlugin},
    spritely::{shader::SpritelyMaterial, SpritelyPlugin},
};
use utilities::UtilitiesPlugin;
use workers::WorkersPlugin;

#[derive(Resource)]
pub struct Debug(pub bool);

pub mod components;
#[cfg(feature = "editor")]
pub mod editor;
pub mod materials;
pub mod pattern_wfc;
pub mod utilities;
pub mod workers;
pub mod showcase;

pub const IRRIDESCANT_SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(1208033355542926744);
pub const PAINTERLY_SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(1908033355537029744);
pub const GALAXYFOG_SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(1508032910437029714);
pub const PROC_TEXTURE_HANDLE: Handle<Shader> = Handle::weak_from_u128(1708033355537473489);
pub const PROC_TEXTURE_HANDLE_A: Handle<Shader> = Handle::weak_from_u128(1708033356723473489);
pub const PROC_TEXTURE_HANDLE_B: Handle<Shader> = Handle::weak_from_u128(1708994455537473489);
pub const PROC_TEXTURE_HANDLE_C: Handle<Shader> = Handle::weak_from_u128(1708033353718453747);
pub const PROC_TEXTURE_HANDLE_D: Handle<Shader> = Handle::weak_from_u128(4333555337168973489);
pub const NOISE_FUNCTIONS_HANDLE: Handle<Shader> = Handle::weak_from_u128(94071345065644201137);
pub const NOISE_GEN_UTILS_HANDLE: Handle<Shader> = Handle::weak_from_u128(94071345065837501137);
pub const NOISE_COMPUTE_HANDLE: Handle<Shader> = Handle::weak_from_u128(24071345358763528837);
pub const SIMPLEX_HANDLE: Handle<Shader> = Handle::weak_from_u128(34071823065847501137);
pub const SIMPLEX_4D_HANDLE: Handle<Shader> = Handle::weak_from_u128(34071823465847412137);
pub const GLOBAL_VALUES_HANDLE: Handle<Shader> = Handle::weak_from_u128(34071828566847501137);
pub const TEX_GEN_HANDLE: Handle<Shader> = Handle::weak_from_u128(1508033515847412137);
pub const PATTERN_WFC_HANDLE: Handle<Shader> = Handle::weak_from_u128(1708343657678895007029744);
pub const PATTERN_FUNC_HANDLE: Handle<Shader> = Handle::weak_from_u128(17083435765920889029744);
pub const SPRITELY_HANDLE: Handle<Shader> = Handle::weak_from_u128(1708343655899895001229744);
pub const SOBEL_HANDLE: Handle<Shader> = Handle::weak_from_u128(1748343643674965476001229744);
pub const BLEND_MODES_HANDLE: Handle<Shader> = Handle::weak_from_u128(184229632462351882081599150);
pub const CONVERTERS_HANDLE: Handle<Shader> = Handle::weak_from_u128(522521912971636216150179);

pub struct AlkydPlugin {
    pub debug: bool,
}

impl Plugin for AlkydPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource::<Debug>(Debug(self.debug))
            .add_plugins((
                UtilitiesPlugin,
                MaterialSwatchPlugin,
                IrridescantMaterialPlugin,
                GalaxyFogPlugin,
                SpritelyPlugin,
                MaterialPlugin::<PainterlyMaterial>::default(),
                MaterialPlugin::<SpritelyMaterial>::default(),
                MaterialPlugin::<ExtendedMaterial<StandardMaterial, IrridescantMaterial>>::default(),
                AppComputePlugin,
                MaterialPlugin::<GalaxyFogMaterial>::default(),
                WorkersPlugin,
                #[cfg(feature = "editor")]
                EditorPlugin,
            ));
    }
}
