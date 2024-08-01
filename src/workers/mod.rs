use std::time::Duration;

use bevy::{
    app::ScheduleRunnerPlugin,
    prelude::*,
    render::{
        extract_resource::ExtractResourcePlugin,
        graph::CameraDriverLabel,
        render_graph::RenderGraph,
        render_resource::{BufferDescriptor, BufferUsages},
        renderer::RenderDevice,
        Render, RenderApp, RenderSet,
    },
};
#[cfg(feature = "compute")]
use resources::{Canvas, CommonUniformMeta, NoiseGeneratorPipeline, NoiseImage, ShaderHandles};
use systems::{queue_bind_group, setup, NoiseGeneratorLabel, NoiseGeneratorNode};
use texture_slots::{
    texture_a::{queue_bind_group_a, TextureA, TextureALabel, TextureANode},
    texture_b::{queue_bind_group_b, TextureB, TextureBLabel, TextureBNode},
    texture_c::{queue_bind_group_c, TextureC, TextureCLabel, TextureCNode},
    texture_d::{queue_bind_group_d, TextureD, TextureDLabel, TextureDNode},
};

pub mod resources;
pub mod systems;
pub mod texture_slots;
pub struct WorkersPlugin;

pub const SHADER_ASSET_PATH: &str = "shaders/noise/noise.wgsl";
pub const DISPLAY_FACTOR: u32 = 4;
pub const SIZE: (u32, u32) = (1920 / DISPLAY_FACTOR, 1080 / DISPLAY_FACTOR);
pub const WORKGROUP_SIZE: u32 = 8;

impl Plugin for WorkersPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "compute")]
        app.add_plugins((
            ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(1.0 / 120.0)),
            ExtractResourcePlugin::<TextureA>::default(),
            ExtractResourcePlugin::<TextureB>::default(),
            ExtractResourcePlugin::<TextureC>::default(),
            ExtractResourcePlugin::<TextureD>::default(),
            ExtractResourcePlugin::<NoiseImage>::default(),
            ExtractResourcePlugin::<ShaderHandles>::default(),
            ExtractResourcePlugin::<Canvas>::default(),
        ))
        .add_systems(Startup, setup);
        let render_app = app.sub_app_mut(RenderApp);
        render_app.add_systems(
            Render,
            (queue_bind_group
                .pipe(queue_bind_group_a)
                .pipe(queue_bind_group_b)
                .pipe(queue_bind_group_c)
                .pipe(queue_bind_group_d))
            .in_set(RenderSet::Queue),
        );

        let mut render_graph = render_app
            .world_mut()
            .get_resource_mut::<RenderGraph>()
            .expect("Should be able to get render graph");
        render_graph.add_node(NoiseGeneratorLabel, NoiseGeneratorNode::default());
        render_graph.add_node(TextureALabel, TextureANode::default());
        render_graph.add_node(TextureBLabel, TextureBNode::default());
        render_graph.add_node(TextureCLabel, TextureCNode::default());
        render_graph.add_node(TextureDLabel, TextureDNode::default());
        render_graph.add_node_edge(TextureALabel, TextureBLabel);
        render_graph.add_node_edge(TextureBLabel, TextureCLabel);
        render_graph.add_node_edge(TextureCLabel, TextureDLabel);
        render_graph.add_node_edge(TextureDLabel, NoiseGeneratorLabel);
        render_graph.add_node_edge(NoiseGeneratorLabel, CameraDriverLabel);
    }
    fn finish(&self, app: &mut App) {
        let render_app = app.sub_app_mut(RenderApp);
        let render_device = render_app.world().resource::<RenderDevice>();

        let buffer = render_device.create_buffer(&BufferDescriptor {
            label: Some("common uniform buffer"),
            size: std::mem::size_of::<f32>() as u64 * 25,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        #[cfg(feature = "compute")]
        render_app
            .init_resource::<NoiseGeneratorPipeline>()
            .insert_resource(CommonUniformMeta {
                buffer: buffer.clone(),
            });
    }
}
