use bevy::{
    prelude::*,
    render::{
        extract_resource::ExtractResourcePlugin, graph::CameraDriverLabel,
        render_graph::RenderGraph, Render, RenderApp, RenderSet,
    },
};
use resources::{NoiseGeneratorPipeline, NoiseImages};
use systems::{prepare_bind_group, setup, NoiseGeneratorLabel, NoiseGeneratorNode};

pub mod resources;
pub mod systems;
pub struct WorkersPlugin;

pub const SHADER_ASSET_PATH: &str = "noise.wgsl";
pub const DISPLAY_FACTOR: u32 = 4;
pub const SIZE: (u32, u32) = (1920 / DISPLAY_FACTOR, 1080 / DISPLAY_FACTOR);
pub const WORKGROUP_SIZE: u32 = 8;

impl Plugin for WorkersPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ExtractResourcePlugin::<NoiseImages>::default())
            .add_systems(Startup, setup);
        let render_app = app.sub_app_mut(RenderApp).add_systems(
            Render,
            prepare_bind_group.in_set(RenderSet::PrepareBindGroups),
        );

        let mut render_graph = render_app
            .world_mut()
            .get_resource_mut::<RenderGraph>()
            .expect("Should be able to get render graph");
        render_graph.add_node(NoiseGeneratorLabel, NoiseGeneratorNode::default());
        render_graph.add_node_edge(NoiseGeneratorLabel, CameraDriverLabel);
    }
    fn finish(&self, app: &mut App) {
        let render_app = app.sub_app_mut(RenderApp);
        render_app.init_resource::<NoiseGeneratorPipeline>();
    }
}
