use bevy::prelude::*;
use bevy_inspector_egui::quick::ResourceInspectorPlugin;
use compute::plugin::{AppComputePlugin, AppComputeWorkerPlugin};

use materials::{
    galaxyfog::{galaxy::GalaxyFogMaterial, GalaxyFogPlugin},
    painterly::resources::{MaterialsInspector, VoronoiImage},
};
use utilities::{UtilitiesPlugin, VoronoiWorker};

use crate::{
    materials::painterly::{painterly::PainterlyMaterial, MaterialSwatchPlugin},
    utilities::systems::LoadNoise,
};

pub struct AlkydPlugin {
    pub debug: bool,
}

#[derive(Resource)]
pub struct Debug(pub bool);

#[derive(Component)]
pub struct Showcase;

pub mod compute;
pub mod materials;
pub mod utilities;

pub const PAINTERLY_SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(1708033355537029744);
pub const GALAXYFOG_SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(1508032910437029714);
pub const VORONOI_SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(1708033355537473489);
pub const NOISE_FUNCTIONS_HANDLE: Handle<Shader> = Handle::weak_from_u128(94071345065644201137);
pub const NOISE_GEN_UTILS_HANDLE: Handle<Shader> = Handle::weak_from_u128(94071345065837501137);
pub const SIMPLEX_HANDLE: Handle<Shader> = Handle::weak_from_u128(34071823065847501137);
pub const SIMPLEX_4D_HANDLE: Handle<Shader> = Handle::weak_from_u128(34071823465847412137);

impl Plugin for AlkydPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            MaterialSwatchPlugin { debug: self.debug },
            GalaxyFogPlugin { debug: self.debug },
            MaterialPlugin::<PainterlyMaterial>::default(),
            MaterialPlugin::<GalaxyFogMaterial>::default(),
            UtilitiesPlugin,
            AppComputePlugin,
            AppComputeWorkerPlugin::<VoronoiWorker>::default(),
        ));

        app.add_event::<LoadNoise>()
            .insert_resource::<VoronoiImage>(VoronoiImage([Vec4::ZERO; 100]))
            .insert_resource::<Debug>(Debug(self.debug));
    }
}
