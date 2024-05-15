use bevy::{
    asset::embedded_asset,
    prelude::*,
    render::{
        extract_resource::ExtractResourcePlugin,
        render_graph::{RenderGraph, RenderLabel},
        Render, RenderApp, RenderSet,
    },
};

use self::{
    resources::{NoiseImages, NoiseNode, NoisePipeline},
    systems::{queue_bind_group, setup},
};

const DISPLAY_FACTOR: u32 = 4;
const SIZE: (u32, u32) = (1280 / DISPLAY_FACTOR, 720 / DISPLAY_FACTOR);
const WORKGROUP_SIZE: u32 = 8;

pub struct NoiseGenPlugin;

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
struct NoiseLabel;

pub mod resources;
pub mod systems;

impl Plugin for NoiseGenPlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "src", "noise_gen.wgsl");

        app.add_plugins(ExtractResourcePlugin::<NoiseImages>::default())
            .add_systems(Startup, setup);

        let render_app = app.sub_app_mut(RenderApp);
        render_app.add_systems(
            Render,
            queue_bind_group.in_set(RenderSet::PrepareBindGroups),
        );

        let mut render_graph = render_app.world.resource_mut::<RenderGraph>();
        render_graph.add_node(NoiseLabel, NoiseNode::default());
        render_graph.add_node_edge(NoiseLabel, bevy::render::graph::CameraDriverLabel);
    }

    fn finish(&self, app: &mut App) {
        let render_app = app.sub_app_mut(RenderApp);
        render_app.init_resource::<NoisePipeline>();
    }
}
