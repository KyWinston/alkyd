use bevy::prelude::*;
use bevy_inspector_egui::quick::ResourceInspectorPlugin;
use compute::plugin::{AppComputePlugin, AppComputeWorkerPlugin};

use materials::painterly::resources::{MaterialsInspector, VoronoiImage};
use utilities::{UtilitiesPlugin, VoronoiWorker};
use wgpu::hal::empty::Resource;

use crate::{
    materials::painterly::{painterly::PainterlyMaterial, MaterialSwatchPlugin},
    utilities::systems::LoadNoise,
};

pub struct AlkydPlugin {
    pub debug: bool,
}

#[derive(Resource)]
pub struct Debug(pub bool);

pub mod compute;
pub mod materials;
pub mod utilities;

pub const PAINTERLY_SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(1708033355537029744);
pub const VORONOI_SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(1708033355537473489);

impl Plugin for AlkydPlugin {
    fn build(&self, app: &mut App) {

        app.add_plugins((
            MaterialSwatchPlugin { debug: self.debug },
            MaterialPlugin::<PainterlyMaterial>::default(),
            UtilitiesPlugin,
            AppComputePlugin,
            AppComputeWorkerPlugin::<VoronoiWorker>::default(),
        ));
        app.add_event::<LoadNoise>()
            .insert_resource::<VoronoiImage>(VoronoiImage([Vec4::ZERO; 100]))
            .insert_resource::<Debug>(Debug(self.debug));
        if self.debug {
            app.add_plugins(ResourceInspectorPlugin::<MaterialsInspector>::default());
            app.init_resource::<MaterialsInspector>()
                .register_type::<MaterialsInspector>();
        }
    }
}
