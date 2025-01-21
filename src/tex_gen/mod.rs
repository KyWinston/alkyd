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

use crate::compute::{systems::init_compute, ComputeState, SIZE};

pub mod resources;
pub mod systems;

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

struct TexGenNode {
    state: ComputeState,
}

impl Default for TexGenNode {
    fn default() -> Self {
        Self {
            state: ComputeState::Loading,
        }
    }
}

impl Node for TexGenNode {
    fn update(&mut self, world: &mut World) {
        let pipeline = world.resource::<TexGenPipeline>();
        let pipeline_cache = world.resource::<PipelineCache>();

        match self.state {
            ComputeState::Loading => {
                match pipeline_cache.get_compute_pipeline_state(pipeline.init_pipeline) {
                    CachedPipelineState::Ok(_) => {
                        self.state = ComputeState::Init;
                    }
                    CachedPipelineState::Err(err) => {
                        panic!("Initializing assets:\n{err}")
                    }
                    _ => {}
                }
            }
            ComputeState::Init => {
                self.state = ComputeState::Finished;
            }
            ComputeState::Finished => {}
            ComputeState::Update => {}
        }
    }

    fn run(
        &self,
        _graph: &mut render_graph::RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World,
    ) -> Result<(), render_graph::NodeRunError> {
        let bind_group = &world.resource::<TexGenImageBindGroup>().0;
        let pipeline = world.resource::<TexGenPipeline>();

        init_compute(
            render_context,
            world,
            bind_group,
            pipeline.init_pipeline,
            0,
            &[],
            self.state.clone(),
            ComputePassDescriptor::default(),
        );
        Ok(())
    }
}
