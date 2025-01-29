use bevy::{
    ecs::query::QueryItem,
    math::bounding::Aabb2d,
    prelude::*,
    render::{
        extract_component::ExtractComponent,
        extract_resource::ExtractResource,
        render_resource::{
            binding_types::{storage_buffer, storage_buffer_read_only, storage_buffer_sized, uniform_buffer}, BindGroup, BindGroupEntries, BindGroupLayout, BindGroupLayoutEntries, Buffer, BufferDescriptor, BufferUsages, CachedComputePipelineId, ComputePassDescriptor, ComputePipeline, ComputePipelineDescriptor, PipelineCache, PushConstantRange, ShaderStages
        },
        renderer::{RenderContext, RenderDevice},
    },
};
use bytemuck::{Pod, Zeroable};
use rand::{rngs::StdRng, Rng, SeedableRng};

use crate::utilities::aabb::Aabb2dGpu;

use super::render::prepare::CompactBindGroups;

#[derive(Clone, Copy, Pod, Zeroable)]
#[cfg_attr(feature = "bevy-inspector-egui", derive(Reflect, InspectorOptions))]
#[cfg_attr(feature = "bevy-inspector-egui", reflect(InspectorOptions))]
#[repr(C)]
pub struct Wind {
    pub speed: f32,
    pub amplitude: f32,
    pub frequency: f32,
    pub direction: f32,
    pub oscillation: f32,
    pub scale: f32,
    pub _padding: [f32; 2],
}

impl Default for Wind {
    fn default() -> Self {
        Self {
            speed: 0.15,
            amplitude: 1.,
            frequency: 1.,
            direction: 0.0,
            oscillation: 1.5,
            scale: 100.,

            _padding: [0.0, 0.0],
        }
    }
}

#[derive(Component, Resource, Default, Clone)]
#[cfg_attr(feature = "bevy-inspector-egui", derive(Reflect, InspectorOptions))]
#[cfg_attr(feature = "bevy-inspector-egui", reflect(Resource, InspectorOptions))]
pub struct GrassWind {
    pub wind_data: Wind,
    pub wind_map: Handle<Image>,
}

impl ExtractComponent for GrassWind {
    type QueryData = &'static GrassWind;
    type QueryFilter = ();
    type Out = Self;

    fn extract_component(item: QueryItem<'_, Self::QueryData>) -> Option<Self::Out> {
        Some(item.clone())
    }
}

impl ExtractResource for GrassWind {
    type Source = Self;

    fn extract_resource(source: &Self::Source) -> Self {
        source.clone()
    }
}

impl GrassWind {
    // pub fn generate_wind_map(size: usize, scale: f64) -> Image {
    //     let mut data = Vec::with_capacity(size * size * 4);

    //     let (x1, y1, x2, y2) = (-1.0, -1.0, 1.0, 1.0);
    //     for y in 0..size {
    //         for x in 0..size {
    //             let s = x as f64 / size as f64;
    //             let t = y as f64 / size as f64;
    //             let dx = x2 - x1;
    //             let dy = y2 - y1;

    //             let nx = x1 + (s * 2.0 * PI).cos() * (dx / (2.0 * PI));
    //             let ny = y1 + (t * 2.0 * PI).cos() * (dy / (2.0 * PI));
    //             let nz = x1 + (s * 2.0 * PI).sin() * (dx / (2.0 * PI));
    //             let nw = y1 + (t * 2.0 * PI).sin() * (dy / (2.0 * PI));

    //             let noise = perlin.get([nx * scale, ny * scale, nz * scale, nw * scale]);
    //             let noise_scaled = ((noise + 1.0) / 2.0 * 16777215.0) as u32;

    //             let r = ((noise_scaled >> 16) & 255) as u8;
    //             let g = ((noise_scaled >> 8) & 255) as u8;
    //             let b = (noise_scaled & 255) as u8;

    //             data.push(r);
    //             data.push(g);
    //             data.push(b);
    //             data.push(255);
    //         }
    //     }

    //     Image::new(
    //         Extent3d {
    //             width: size as u32,
    //             height: size as u32,
    //             depth_or_array_layers: 1,
    //         },
    //         TextureDimension::D2,
    //         data,
    //         TextureFormat::Rgba8UnormSrgb,
    //         RenderAssetUsages::all(),
    //     )
    // }
}

#[derive(Resource)]
pub struct GrassClumpPipeline {
    clump_layout: BindGroupLayout,
    chunk_layout: BindGroupLayout,
}

impl FromWorld for GrassClumpPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();

        let clump_layout = render_device.create_bind_group_layout(
            Some("clump_bind_group_layout"),
            &BindGroupLayoutEntries::sequential(
                ShaderStages::COMPUTE,
                (
                    uniform_buffer::<Vec2>(false), // clump_size
                    storage_buffer_sized(false, None),
                ),
            ),
        );

        let chunk_layout = render_device.create_bind_group_layout(
            Some("chunk_layout"),
            &BindGroupLayoutEntries::sequential(
                ShaderStages::COMPUTE,
                (uniform_buffer::<Aabb2dGpu>(false),),
            ),
        );

        Self {
            clump_layout,
            chunk_layout,
        }
    }
}

#[derive(Resource, ExtractResource, Clone, Reflect)]
#[reflect(Resource)]
pub struct GrassClumpConfig {
    pub seed: u64,
    pub aabb: Aabb2d,
    pub count: UVec2,
    pub colors: ClumpColors,
}
impl Default for GrassClumpConfig {
    fn default() -> Self {
        Self {
            seed: 1,
            aabb: Aabb2d {
                min: Vec2::new(-50.0, -50.0),
                max: Vec2::new(50.0, 50.0),
            },
            count: UVec2::new(40, 40),
            colors: ClumpColors::default(),
        }
    }
}

impl GrassClumpConfig {
    fn generate_clumps(&self) -> GrassClumps {
        let cell_size = Vec2::new(
            (self.aabb.max.x - self.aabb.min.x) / self.count.x as f32,
            (self.aabb.max.y - self.aabb.min.y) / self.count.y as f32,
        );

        let mut rng = StdRng::seed_from_u64(self.seed);

        let mut random_points = Vec::new();
        let mut clumps = Vec::new();
        for x in 0..self.count.x {
            for y in 0..self.count.y {
                let x_range = self.aabb.min.x + x as f32 * cell_size.x
                    ..self.aabb.min.x + (x + 1) as f32 * cell_size.x;
                let y_range = self.aabb.min.y + y as f32 * cell_size.y
                    ..self.aabb.min.y + (y + 1) as f32 * cell_size.y;
                random_points.push(Vec2::new(
                    rng.random_range(x_range),
                    rng.random_range(y_range),
                ));

                let facing = match rng.random_range(1..=3) {
                    0 => GrassClumpDirection::In,
                    1 => GrassClumpDirection::Out,
                    2 => GrassClumpDirection::Random,
                    _ => {
                        let random_angle = rng.random_range(0.0..std::f32::consts::TAU);
                        let random_direction = Vec2::new(random_angle.cos(), random_angle.sin());
                        GrassClumpDirection::Facing(random_direction)
                    }
                }
                .to_vec2();

                let color = self.colors.get_random_color(&mut rng);

                clumps.push(GrassClump {
                    tip_color: color.tip.to_linear().to_vec4(),
                    base_color: color.base.to_linear().to_vec4(),
                    facing,
                    length: rng.random_range(0.8..1.2),
                    tilt: 0.8,
                })
            }
        }

        GrassClumps {
            cell_size,
            positions: random_points,
            params: clumps,
        }
    }
}

#[derive(Reflect, Clone)]
pub struct ClumpColor {
    tip: Color,
    base: Color,
    weight: f32,
}

#[derive(Reflect, Clone)]
pub struct ClumpColors {
    colors: Vec<ClumpColor>,
}
impl Default for ClumpColors {
    fn default() -> Self {
        Self {
            colors: vec![
                ClumpColor {
                    tip: Srgba::rgb(0.15, 0.17, 0.01).into(),
                    base: Srgba::rgb(0.117, 0.20, 0.0).into(),
                    weight: 1.0,
                },
                ClumpColor {
                    tip: Srgba::rgb(0.18, 0.18, 0.1).into(),
                    base: Srgba::rgb(0.17, 0.184, 0.085).into(),
                    weight: 0.25,
                },
            ],
        }
    }
}

impl ClumpColors {
    pub fn get_random_color(&self, rng: &mut StdRng) -> ClumpColor {
        if self.colors.is_empty() {
            return ClumpColor {
                tip: Color::srgb(1.0, 1.0, 1.0),
                base: Color::srgb(1.0, 1.0, 1.0),
                weight: 1.0,
            };
        }

        let total_weight: f32 = self.colors.iter().map(|c| c.weight).sum();
        let mut random = rng.random_range(0.0..total_weight);

        for color in &self.colors {
            random -= color.weight;
            if random <= 0.0 {
                return color.clone();
            }
        }

        self.colors[0].clone()
    }
}

pub enum GrassClumpDirection {
    In,
    Out,
    Random,
    Facing(Vec2),
}
impl GrassClumpDirection {
    pub fn to_vec2(&self) -> Vec2 {
        match self {
            Self::In => Vec2::new(2.0, 0.0),
            Self::Out => Vec2::new(3.0, 0.0),
            Self::Random => Vec2::new(4.0, 0.0),
            Self::Facing(direction) => direction.normalize(),
        }
    }
}

#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct GrassClump {
    tip_color: Vec4,
    base_color: Vec4,
    facing: Vec2,
    length: f32,
    tilt: f32,
}

#[derive(Resource, ExtractResource, Clone)]
pub struct GrassClumps {
    pub cell_size: Vec2,
    pub positions: Vec<Vec2>,
    pub params: Vec<GrassClump>,
}

pub(crate) fn clump_startup(mut commands: Commands, clump_config: Res<GrassClumpConfig>) {
    commands.insert_resource(clump_config.generate_clumps());
}

#[derive(Resource)]
pub struct GrassClumpsBindGroup {
    pub(crate) positions_buffer: Buffer,
    pub(crate) params_buffer: Buffer,
    pub bind_group: BindGroup,
}

#[derive(Resource)]
pub struct PrefixSumPipeline {
    pub scan_layout: BindGroupLayout,
    pub scan_blocks_layout: BindGroupLayout,
    pub scan_pipeline: CachedComputePipelineId,
    pub scan_blocks_pipeline: CachedComputePipelineId,
}

impl FromWorld for PrefixSumPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();

        let scan_layout = render_device.create_bind_group_layout(
            "scan_layout",
            &BindGroupLayoutEntries::sequential(
                ShaderStages::COMPUTE,
                (
                    storage_buffer_read_only::<Vec<u32>>(false),
                    storage_buffer::<Vec<u32>>(false),
                    storage_buffer::<Vec<u32>>(false),
                ),
            ),
        );

        let scan_blocks_layout = render_device.create_bind_group_layout(
            "scan_blocks_layout",
            &BindGroupLayoutEntries::sequential(
                ShaderStages::COMPUTE,
                (
                    storage_buffer_read_only::<Vec<u32>>(false),
                    storage_buffer::<Vec<u32>>(false),
                ),
            ),
        );

        let scan_shader = world
            .resource::<AssetServer>()
            .load("embedded://bevy_procedural_grass/shaders/scan.wgsl");
        let scan_blocks_shader = world
            .resource::<AssetServer>()
            .load("embedded://bevy_procedural_grass/shaders/scan_blocks.wgsl");

        let pipeline_cache = world.resource_mut::<PipelineCache>();

        let scan_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: Some("compute_scan_grass_pipeline".into()),
            layout: vec![scan_layout.clone()],
            zero_initialize_workgroup_memory: false,
            push_constant_ranges: Vec::new(),
            shader: scan_shader.clone(),
            shader_defs: vec![],
            entry_point: "scan".into(),
        });

        let scan_blocks_pipeline =
            pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
                label: Some("compute_scan_blocks_pipeline".into()),
                layout: vec![scan_blocks_layout.clone()],
                zero_initialize_workgroup_memory: false,
                push_constant_ranges: vec![PushConstantRange {
                    stages: ShaderStages::COMPUTE,
                    range: 0..4,
                }],
                shader: scan_blocks_shader.clone(),
                shader_defs: vec![],
                entry_point: "scan_blocks".into(),
            });

        Self {
            scan_layout,
            scan_blocks_layout,
            scan_pipeline,
            scan_blocks_pipeline,
        }
    }
}

impl PrefixSumPipeline {
    pub fn get_pipelines(world: &World) -> Option<(&ComputePipeline, &ComputePipeline)> {
        let pipeline_cache = world.get_resource::<PipelineCache>()?;
        let pipeline = world.get_resource::<Self>()?;

        Some((
            pipeline_cache.get_compute_pipeline(pipeline.scan_pipeline)?,
            pipeline_cache.get_compute_pipeline(pipeline.scan_blocks_pipeline)?,
        ))
    }
}

pub fn prefix_sum_pass(
    render_context: &mut RenderContext,
    chunks: Vec<(&CompactBindGroups, &PrefixSumBindGroups)>,
    scan_pipeline: &ComputePipeline,
    scan_blocks_pipeline: &ComputePipeline,
) {
    let mut pass = render_context
        .command_encoder()
        .begin_compute_pass(&ComputePassDescriptor::default());

    pass.set_pipeline(scan_pipeline);
    for (_, bind_groups) in &chunks {
        pass.set_bind_group(0, &bind_groups.scan_bind_group, &[]);
        pass.dispatch_workgroups(bind_groups.scan_workgroups, 1, 1);
    }
    pass.set_pipeline(scan_blocks_pipeline);
    for (_, bind_groups) in &chunks {
        pass.set_push_constants(0, &(bind_groups.scan_workgroups as u32).to_le_bytes());
        pass.set_bind_group(0, &bind_groups.scan_blocks_bind_group, &[]);
        pass.dispatch_workgroups(bind_groups.scan_blocks_workgroups, 1, 1);
    }
}

#[derive(Clone)]
pub(crate) struct PrefixSumBuffers {
    pub scan_buffer: Buffer,
    pub scan_blocks_buffer: Buffer,
    pub scan_blocks_out_buffer: Buffer,
}

impl PrefixSumBuffers {
    pub fn create_buffers(
        render_device: &RenderDevice,
        input_length: u32,
        scan_workgroups: u32,
    ) -> Self {
        let scan_buffer = render_device.create_buffer(&BufferDescriptor {
            label: Some("scan_buffer"),
            size: (std::mem::size_of::<u32>() * input_length as usize) as u64,
            usage: BufferUsages::STORAGE,
            mapped_at_creation: false,
        });
        let scan_blocks_buffer = render_device.create_buffer(&BufferDescriptor {
            label: Some("scan_blocks_buffer"),
            size: (std::mem::size_of::<u32>() * scan_workgroups as usize) as u64,
            usage: BufferUsages::STORAGE,
            mapped_at_creation: false,
        });

        let scan_blocks_out_buffer = render_device.create_buffer(&BufferDescriptor {
            label: Some("scan_blocks_out_buffer"),
            size: (std::mem::size_of::<u32>() * scan_workgroups as usize) as u64,
            usage: BufferUsages::STORAGE,
            mapped_at_creation: false,
        });

        Self {
            scan_buffer,
            scan_blocks_buffer,
            scan_blocks_out_buffer,
        }
    }
}

#[derive(Component, Clone)]
pub struct PrefixSumBindGroups {
    pub scan_bind_group: BindGroup,
    pub scan_blocks_bind_group: BindGroup,

    pub scan_workgroups: u32,
    pub scan_blocks_workgroups: u32,
}

impl PrefixSumBindGroups {
    pub fn create_bind_groups(
        render_device: &RenderDevice,
        pipeline: &PrefixSumPipeline,
        input_buffer: &Buffer,
        buffers: &PrefixSumBuffers,
        scan_workgroups: u32,
        scan_blocks_workgroups: u32,
    ) -> Self {
        let scan_bind_group = render_device.create_bind_group(
            Some("scan_bind_group"),
            &pipeline.scan_layout,
            &BindGroupEntries::sequential((
                input_buffer.as_entire_binding(),
                buffers.scan_buffer.as_entire_binding(),
                buffers.scan_blocks_buffer.as_entire_binding(),
            )),
        );

        let scan_blocks_bind_group = render_device.create_bind_group(
            Some("scan_blocks_bind_group"),
            &pipeline.scan_blocks_layout,
            &BindGroupEntries::sequential((
                buffers.scan_blocks_buffer.as_entire_binding(),
                buffers.scan_blocks_out_buffer.as_entire_binding(),
            )),
        );

        Self {
            scan_bind_group,
            scan_blocks_bind_group,

            scan_workgroups,
            scan_blocks_workgroups,
        }
    }
}

pub fn calculate_workgroup_counts(count: u32) -> (u32, u32) {
    let mut scan_workgroup_count = (count as f32 / 128.).ceil() as u32;
    if scan_workgroup_count > 128 {
        let mut p2 = 128;
        while p2 < scan_workgroup_count {
            p2 *= 2;
        }

        scan_workgroup_count = p2;
    } else {
        while 128 % scan_workgroup_count != 0 {
            scan_workgroup_count += 1;
        }
    }

    let scan_groups_workgroup_count = (count as f32 / 1024.).ceil() as u32;

    (scan_workgroup_count, scan_groups_workgroup_count)
}
