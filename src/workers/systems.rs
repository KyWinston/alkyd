use bevy::{
    prelude::*,
    render::{
        render_asset::{RenderAssetUsages, RenderAssets},
        render_graph::{self, RenderLabel},
        render_resource::*,
        renderer::{RenderContext, RenderDevice},
        texture::GpuImage,
    },
    utils::hashbrown::HashMap,
};

use super::{
    resources::{NoiseGeneratorBindGroups, NoiseGeneratorPipeline, NoiseImages},
    SHADER_ASSET_PATH, SIZE, WORKGROUP_SIZE,
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
pub struct NoiseGeneratorLabel;

pub fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let mut image = Image::new_fill(
        Extent3d {
            width: SIZE.0,
            height: SIZE.1,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[0, 0, 0, 255],
        TextureFormat::R32Float,
        RenderAssetUsages::RENDER_WORLD,
    );
    image.texture_descriptor.usage =
        TextureUsages::COPY_DST | TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING;
    let mut img = HashMap::new();
    img.insert(
        "test".to_string(),
        [images.add(image.clone()), images.add(image)],
    );
    commands.insert_resource(NoiseImages(img));
}

pub fn prepare_bind_group(
    mut commands: Commands,
    pipeline: Res<NoiseGeneratorPipeline>,
    gpu_images: Res<RenderAssets<GpuImage>>,
    noise_images: ResMut<NoiseImages>,
    render_device: Res<RenderDevice>,
) {
    let view_a = gpu_images.get(noise_images.0["test"][0].id()).unwrap();
    let view_b = gpu_images.get(noise_images.0["test"][1].id()).unwrap();

    let bind_group = render_device.create_bind_group(
        None,
        &pipeline.texture_bind_group_layout,
        &BindGroupEntries::sequential((&view_a.texture_view, &view_b.texture_view)),
    );

    let bind_group2 = render_device.create_bind_group(
        None,
        &pipeline.texture_bind_group_layout,
        &BindGroupEntries::sequential((&view_a.texture_view, &view_b.texture_view)),
    );

    commands.insert_resource(NoiseGeneratorBindGroups([bind_group, bind_group2]));
}

enum NoiseGeneratorState {
    Loading,
    Init,
    Update(usize),
}

pub struct NoiseGeneratorNode {
    state: NoiseGeneratorState,
}

impl Default for NoiseGeneratorNode {
    fn default() -> Self {
        Self {
            state: NoiseGeneratorState::Loading,
        }
    }
}

impl render_graph::Node for NoiseGeneratorNode {
    fn update(&mut self, world: &mut World) {
        let pipeline = world.resource::<NoiseGeneratorPipeline>();
        let pipeline_cache = world.resource::<PipelineCache>();
        let path = SHADER_ASSET_PATH;

        match self.state {
            NoiseGeneratorState::Loading => {
                match pipeline_cache.get_compute_pipeline_state(pipeline.init_pipeline) {
                    CachedPipelineState::Ok(_) => {
                        self.state = NoiseGeneratorState::Init;
                    }
                    CachedPipelineState::Err(err) => {
                        panic!("Initializing assets/{path}:\n{err}")
                    }
                    _ => {}
                }
            }
            NoiseGeneratorState::Init => {
                if let CachedPipelineState::Ok(state) =
                    pipeline_cache.get_compute_pipeline_state(pipeline.update_pipeline)
                {
                    println!("{:?}", state);
                    self.state = NoiseGeneratorState::Update(1);
                }
            }
            NoiseGeneratorState::Update(0) => {
                self.state = NoiseGeneratorState::Update(1);
            }
            NoiseGeneratorState::Update(1) => {
                self.state = NoiseGeneratorState::Update(0);
            }
            NoiseGeneratorState::Update(_) => unreachable!(),
        }
    }

    fn run(
        &self,
        _graph: &mut render_graph::RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World,
    ) -> Result<(), render_graph::NodeRunError> {
        let bind_groups = world.resource::<NoiseGeneratorBindGroups>().0.clone();
        let pipeline_cache = world.resource::<PipelineCache>();
        let pipeline = world.resource::<NoiseGeneratorPipeline>();

        let mut pass = render_context
            .command_encoder()
            .begin_compute_pass(&ComputePassDescriptor::default());
        match self.state {
            NoiseGeneratorState::Loading => {}
            NoiseGeneratorState::Init => {
                let init_pipeline = pipeline_cache
                    .get_compute_pipeline(pipeline.init_pipeline)
                    .unwrap();
                pass.set_bind_group(0, &bind_groups[0], &[]);
                pass.set_pipeline(init_pipeline);
                pass.dispatch_workgroups(SIZE.0 / WORKGROUP_SIZE, SIZE.1 / WORKGROUP_SIZE, 1);
            }
            NoiseGeneratorState::Update(index) => {
                let init_pipeline = pipeline_cache
                    .get_compute_pipeline(pipeline.init_pipeline)
                    .unwrap();
                pass.set_bind_group(0, &bind_groups[index], &[]);
                pass.set_pipeline(init_pipeline);
                pass.dispatch_workgroups(SIZE.0 / WORKGROUP_SIZE, SIZE.1 / WORKGROUP_SIZE, 1);
            }
        }
        Ok(())
    }
}
