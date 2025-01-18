//! A compute shader that simulates Conway's Game of Life.
//!
//! Compute shaders use the GPU for computing arbitrary information, that may be independent of what
//! is rendered to the screen.

use bevy::{
    prelude::*,
    render::{
        extract_resource::ExtractResourcePlugin,
        graph::CameraDriverLabel,
        render_graph::{self, Node, RenderGraph, RenderLabel},
        render_resource::*,
        renderer::RenderContext,
        Render, RenderApp, RenderSet,
    },
};
use resources::{TexGenImage, TexGenImageBindGroup, TexGenPipeline};
use systems::{prepare_bind_group, setup};

pub mod resources;
pub mod systems;

pub const SIZE: u32 = 1920;
pub const WORKGROUP_SIZE: u32 = 8;

pub struct TextureGenPlugin;

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
struct TexGenLabel;

impl Plugin for TextureGenPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ExtractResourcePlugin::<TexGenImage>::default())
            .add_systems(Startup, setup);

        let render_app = app.sub_app_mut(RenderApp);
        render_app.add_systems(
            Render,
            prepare_bind_group.in_set(RenderSet::PrepareBindGroups),
        );

        let mut render_graph = render_app.world_mut().resource_mut::<RenderGraph>();
        render_graph.add_node(TexGenLabel, TexGenNode::default());
        render_graph.add_node_edge(TexGenLabel, CameraDriverLabel);
    }

    fn finish(&self, app: &mut App) {
        let render_app = app.sub_app_mut(RenderApp);
        render_app.init_resource::<TexGenPipeline>();
    }
}

enum TexGenState {
    Loading,
    Init,
    Finished,
}

struct TexGenNode {
    state: TexGenState,
}

impl Default for TexGenNode {
    fn default() -> Self {
        Self {
            state: TexGenState::Loading,
        }
    }
}

impl Node for TexGenNode {
    fn update(&mut self, world: &mut World) {
        let pipeline = world.resource::<TexGenPipeline>();
        let pipeline_cache = world.resource::<PipelineCache>();

        // if the corresponding pipeline has loaded, transition to the next stage
        match self.state {
            TexGenState::Loading => {
                match pipeline_cache.get_compute_pipeline_state(pipeline.init_pipeline) {
                    CachedPipelineState::Ok(_) => {
                        self.state = TexGenState::Init;
                    }
                    CachedPipelineState::Err(err) => {
                        panic!("Initializing assets:\n{err}")
                    }
                    _ => {}
                }
            }
            TexGenState::Init => {
                self.state = TexGenState::Finished;
            }
            TexGenState::Finished => {}
        }
    }

    fn run(
        &self,
        _graph: &mut render_graph::RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World,
    ) -> Result<(), render_graph::NodeRunError> {
        let bind_group = &world.resource::<TexGenImageBindGroup>().0;
        let pipeline_cache = world.resource::<PipelineCache>();
        let pipeline = world.resource::<TexGenPipeline>();

        let mut pass = render_context
            .command_encoder()
            .begin_compute_pass(&ComputePassDescriptor::default());

        // select the pipeline based on the current state
        match self.state {
            TexGenState::Loading => {}
            TexGenState::Init => {
                let init_pipeline = pipeline_cache
                    .get_compute_pipeline(pipeline.init_pipeline)
                    .unwrap();
                pass.set_bind_group(0, &bind_group[0], &[]);
                pass.set_pipeline(init_pipeline);
                pass.dispatch_workgroups(
                    SIZE / WORKGROUP_SIZE,
                    SIZE / WORKGROUP_SIZE,
                    SIZE / WORKGROUP_SIZE,
                );
            }
            TexGenState::Finished => {}
        }
        Ok(())
    }
}
