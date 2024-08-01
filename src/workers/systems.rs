use std::borrow::Cow;

use bevy::{
    prelude::*,
    render::{
        render_asset::{RenderAssetUsages, RenderAssets},
        render_graph::{self, RenderLabel},
        render_resource::*,
        renderer::{RenderContext, RenderDevice},
        texture::GpuImage,
    },
    window::PrimaryWindow,
};

use crate::workers::resources::{CommonUniform, NoiseImage};

use super::{
    resources::{NoiseGeneratorBindGroup, NoiseGeneratorPipeline, ShaderHandles},
    texture_slots::{
        texture_a::TextureA, texture_b::TextureB, texture_c::TextureC, texture_d::TextureD,
    },
    SHADER_ASSET_PATH, SIZE, WORKGROUP_SIZE,
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
pub struct NoiseGeneratorLabel;

pub fn setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    info!("loading main slot");

    let mut image = Image::new_fill(
        Extent3d {
            width: SIZE.0,
            height: SIZE.1,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[0; SIZE.0 as usize * SIZE.1 as usize * 16],
        TextureFormat::Rgba32Float,
        RenderAssetUsages::RENDER_WORLD,
    );
    image.texture_descriptor.usage =
        TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING;
    let image = images.add(image);

    #[cfg(feature = "editor")]
    commands.spawn(Camera2dBundle {
        camera: Camera {
            target: RenderTarget::Image(image.clone()),
            ..default()
        },
        ..default()
    });

    commands.insert_resource(NoiseImage(image.clone()));

    let window = windows.get_single().unwrap();
    let mut common_uniform = CommonUniform::new();

    common_uniform.i_resolution.x = window.width();
    common_uniform.i_resolution.y = window.height();
    commands.insert_resource(common_uniform);

    info!("loading a slot");
    let mut texture_a = Image::new_fill(
        Extent3d {
            width: SIZE.0,
            height: SIZE.1,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[0; SIZE.0 as usize * SIZE.1 as usize * 16],
        TextureFormat::Rgba32Float,
        RenderAssetUsages::RENDER_WORLD,
    );
    texture_a.texture_descriptor.usage =
        TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING;

    let texture_a = images.add(texture_a);

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(SIZE.0 as f32, SIZE.1 as f32)),
            ..default()
        },
        texture: texture_a.clone(),

        ..default()
    });

    commands.insert_resource(TextureA(texture_a));
    info!("loading b slot");

    let mut texture_b = Image::new_fill(
        Extent3d {
            width: SIZE.0,
            height: SIZE.1,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[0; SIZE.0 as usize * SIZE.1 as usize * 4],
        TextureFormat::Rgba32Float,
        RenderAssetUsages::RENDER_WORLD,
    );
    texture_b.texture_descriptor.usage =
        TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING;

    let texture_b = images.add(texture_b);

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(SIZE.0 as f32, SIZE.1 as f32)),
            ..default()
        },
        texture: texture_b.clone(),

        ..default()
    });

    commands.insert_resource(TextureB(texture_b));
    info!("loading c slot");

    let mut texture_c = Image::new_fill(
        Extent3d {
            width: SIZE.0,
            height: SIZE.1,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[0; SIZE.0 as usize * SIZE.1 as usize * 4],
        TextureFormat::Rgba32Float,
        RenderAssetUsages::RENDER_WORLD,
    );
    texture_c.texture_descriptor.usage =
        TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING;

    let texture_c = images.add(texture_c);
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(SIZE.0 as f32, SIZE.1 as f32)),
            ..default()
        },
        texture: texture_c.clone(),
        ..default()
    });
    commands.insert_resource(TextureC(texture_c));
    info!("loading d slot");

    let mut texture_d = Image::new_fill(
        Extent3d {
            width: SIZE.0,
            height: SIZE.1,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[0; SIZE.0 as usize * SIZE.1 as usize * 4],
        TextureFormat::Rgba32Float,
        RenderAssetUsages::RENDER_WORLD,
    );
    texture_d.texture_descriptor.usage =
        TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING;

    let texture_d = images.add(texture_d);

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(SIZE.0 as f32, SIZE.1 as f32)),
            ..default()
        },
        texture: image.clone(),

        ..default()
    });

    commands.insert_resource(TextureD(texture_d));
}

// pub fn make_and_load_shaders(asset_server: &Res<AssetServer>) -> ShaderHandles {
//     let image_shader_handle = asset_server.load("shaders/{}/image.wgsl");
//     let texture_a_shader = asset_server.load("shaders/{}/buffer_a.wgsl");
//     let texture_b_shader = asset_server.load("shaders/{}/buffer_b.wgsl");
//     let texture_c_shader = asset_server.load("shaders/{}/buffer_c.wgsl");
//     let texture_d_shader = asset_server.load("shaders/{}/buffer_d.wgsl");

//     ShaderHandles {
//         image_shader: image_shader_handle,
//         texture_a_shader,
//         texture_b_shader,
//         texture_c_shader,
//         texture_d_shader,
//     }
// }

pub fn make_and_load_shaders(asset_server: &Res<AssetServer>) -> ShaderHandles {
    let image_shader = asset_server.load("./shaders/noise/noise.wgsl");
    let texture_a_shader = asset_server.load("./shaders/noise/slot_a.wgsl");
    let texture_b_shader = asset_server.load("./shaders/noise/slot_b.wgsl");
    let texture_c_shader = asset_server.load("./shaders/noise/slot_c.wgsl");
    let texture_d_shader = asset_server.load("./shaders/noise/slot_d.wgsl");

    ShaderHandles {
        image_shader,
        texture_a_shader,
        texture_b_shader,
        texture_c_shader,
        texture_d_shader,
    }
}

pub fn make_new_texture(
    canvas_size: &Vec2,
    image_handle: &Handle<Image>,
    images: &mut ResMut<Assets<Image>>,
) {
    if let Some(image) = images.get_mut(image_handle) {
        image.resize(Extent3d {
            width: canvas_size.x as u32,
            height: canvas_size.y as u32,
            depth_or_array_layers: 1,
        });
    }
}

pub fn extract_stuff_here(mut commands: Commands, all_shader_handles: Res<ShaderHandles>) {
    commands.insert_resource(all_shader_handles.clone());
}

pub fn queue_bind_group(
    mut commands: Commands,
    pipeline: Res<NoiseGeneratorPipeline>,
    gpu_images: Res<RenderAssets<GpuImage>>,
    main_image: Res<NoiseImage>,
    pipeline_cache: ResMut<PipelineCache>,
    texture_a_image: Res<TextureA>,
    texture_b_image: Res<TextureB>,
    texture_c_image: Res<TextureC>,
    texture_d_image: Res<TextureD>,
    asset_server: Res<AssetServer>,
    render_device: Res<RenderDevice>,
) {
    let shader = asset_server.load("shaders/noise/noise.wgsl");
    let init_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
        label: None,
        layout: vec![pipeline.texture_bind_group_layout.clone()],
        push_constant_ranges: vec![],
        shader: shader.clone(),
        shader_defs: vec!["INIT".to_string().into()],
        entry_point: Cow::from("init"),
    });

    let update_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
        label: None,
        layout: vec![pipeline.texture_bind_group_layout.clone()],
        push_constant_ranges: vec![],
        shader,
        shader_defs: vec![],
        entry_point: Cow::from("update"),
    });

    let main_view = gpu_images.get(&main_image.0).unwrap();

    let texture_a_view = gpu_images.get(&texture_a_image.0).unwrap();
    let texture_b_view = gpu_images.get(&texture_b_image.0).unwrap();
    let texture_c_view = gpu_images.get(&texture_c_image.0).unwrap();
    let texture_d_view = gpu_images.get(&texture_d_image.0).unwrap();

    let noise_gen_bind_group = render_device.create_bind_group(
        Some("main_bind_group"),
        &pipeline.texture_bind_group_layout,
        &[
            BindGroupEntry {
                binding: 0,
                resource: BindingResource::TextureView(&texture_a_view.texture_view),
            },
            BindGroupEntry {
                binding: 1,
                resource: BindingResource::TextureView(&texture_b_view.texture_view),
            },
            BindGroupEntry {
                binding: 2,
                resource: BindingResource::TextureView(&texture_c_view.texture_view),
            },
            BindGroupEntry {
                binding: 3,
                resource: BindingResource::TextureView(&texture_d_view.texture_view),
            },
            BindGroupEntry {
                binding: 4,
                resource: BindingResource::TextureView(&main_view.texture_view),
            },
        ],
    );

    commands.insert_resource(NoiseGeneratorBindGroup {
        noise_gen_bind_group,
        init_pipeline: init_pipeline.clone(),
        update_pipeline: update_pipeline.clone(),
    });
}

pub enum NoiseGeneratorState {
    Loading,
    Init,
    Update,
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
        let pipeline = world.resource::<NoiseGeneratorBindGroup>();
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
                    self.state = NoiseGeneratorState::Update;
                }
            }
            NoiseGeneratorState::Update => unreachable!(),
        }
    }

    fn run(
        &self,
        _graph: &mut render_graph::RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World,
    ) -> Result<(), render_graph::NodeRunError> {
        info!("running main");

        let bind_group = world.resource::<NoiseGeneratorBindGroup>();

        let init_pipeline_cache = bind_group.init_pipeline;
        let update_pipeline_cache = bind_group.update_pipeline;

        let pipeline_cache = world.resource::<PipelineCache>();

        let mut pass =
            render_context
                .command_encoder()
                .begin_compute_pass(&ComputePassDescriptor {
                    label: Some("main_compute_pass"),
                    ..default()
                });
        pass.set_bind_group(0, &bind_group.noise_gen_bind_group, &[]);
        match self.state {
            NoiseGeneratorState::Loading => {}
            NoiseGeneratorState::Init => {
                let init_pipeline = pipeline_cache
                    .get_compute_pipeline(init_pipeline_cache)
                    .unwrap();
                pass.set_pipeline(init_pipeline);
                pass.dispatch_workgroups(SIZE.0 / WORKGROUP_SIZE, SIZE.1 / WORKGROUP_SIZE, 1);
            }
            NoiseGeneratorState::Update => {
                let update_pipeline = pipeline_cache
                    .get_compute_pipeline(update_pipeline_cache)
                    .unwrap();
                pass.set_pipeline(update_pipeline);
                pass.dispatch_workgroups(SIZE.0 / WORKGROUP_SIZE, SIZE.1 / WORKGROUP_SIZE, 1);
            }
        }
        Ok(())
    }
}
