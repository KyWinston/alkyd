use bevy::prelude::*;

use bevy_app_compute::prelude::{AppComputePlugin, AppComputeWorkerPlugin};
use materials::{
    galaxyfog::{galaxy::GalaxyFogMaterial, GalaxyFogPlugin},
    painterly::resources::VoronoiImage,
};
use utilities::{
    systems::{LoadNoise, LoadPattern},
    TexGenWorker, UtilitiesPlugin, VoronoiWorker,
};

use crate::materials::painterly::{painterly::PainterlyMaterial, MaterialSwatchPlugin};

#[derive(Resource)]
pub struct Debug(pub bool);

#[derive(Component)]
pub struct Showcase;

pub mod materials;
pub mod pattern_wfc;
pub mod utilities;

pub const PAINTERLY_SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(1708033355537029744);
pub const GALAXYFOG_SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(1508032910437029714);
pub const VORONOI_SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(1708033355537473489);
pub const NOISE_FUNCTIONS_HANDLE: Handle<Shader> = Handle::weak_from_u128(94071345065644201137);
pub const NOISE_GEN_UTILS_HANDLE: Handle<Shader> = Handle::weak_from_u128(94071345065837501137);
pub const SIMPLEX_HANDLE: Handle<Shader> = Handle::weak_from_u128(34071823065847501137);
pub const SIMPLEX_4D_HANDLE: Handle<Shader> = Handle::weak_from_u128(34071823465847412137);
pub const GLOBAL_VALUES_HANDLE: Handle<Shader> = Handle::weak_from_u128(34071828566847501137);
pub const TEX_GEN_HANDLE: Handle<Shader> = Handle::weak_from_u128(1508033515847412137);
pub const PATTERN_WFC_HANDLE: Handle<Shader> = Handle::weak_from_u128(1708343657678895007029744);
pub const PATTERN_FUNC_HANDLE: Handle<Shader> = Handle::weak_from_u128(17083435765920889029744);

pub struct AlkydPlugin {
    pub debug: bool,
}

impl Plugin for AlkydPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            MaterialSwatchPlugin { debug: self.debug },
            GalaxyFogPlugin { debug: self.debug },
            MaterialPlugin::<PainterlyMaterial>::default(),
            MaterialPlugin::<GalaxyFogMaterial>::default(),
            // MaterialPlugin::<PatternGenFunc>::default(),

            #[cfg(feature = "editor")]
            EditorPlugin,
            UtilitiesPlugin,
            AppComputePlugin,
            AppComputeWorkerPlugin::<VoronoiWorker>::default(),
            AppComputeWorkerPlugin::<TexGenWorker>::default(),
        ));

        app.add_event::<LoadNoise>()
            .add_event::<LoadPattern>()
            .insert_resource::<VoronoiImage>(VoronoiImage([Vec4::ZERO; 100]))
            .insert_resource::<Debug>(Debug(self.debug));
    }
}
