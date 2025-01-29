use super::{
    components::{Grass, GrassChunk, GrassChunkBuffers, GrassCullChunks, GrassGpuInfo},
    config::GrassConfig,
    lod::GrassLODMesh,
    material::GrassMaterial,
};
use bevy::{
    ecs::{entity::Entity, system::Commands},
    hierarchy::BuildChildren,
    math::{bounding::Aabb2d, Affine3A},
    prelude::*,
    render::{
        batching::NoAutomaticBatching, primitives::{Aabb, Frustum}, renderer::RenderDevice,
        view::NoFrustumCulling,
    },
};

pub(crate) fn unload_chunks(
    commands: &mut Commands,
    entity: Entity,
    cull_chunks: &mut GrassCullChunks,
) {
    for (_, chunk_entity) in cull_chunks.0.iter() {
        commands.entity(entity).remove_children(&[*chunk_entity]);
        commands.entity(*chunk_entity).despawn();
    }
    cull_chunks.0.clear();
}

pub(crate) fn cull_chunks(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
    mut q_grass: Query<(
        Entity,
        &Grass,
        &GrassGpuInfo,
        &mut GrassCullChunks,
        &Mesh3d,
        &GrassLODMesh,
        &MeshMaterial3d<GrassMaterial>,
        &Visibility,
    )>,
    meshes: Res<Assets<Mesh>>,
    camera_query: Query<(&Transform, &Frustum)>,
    grass_config: Res<GrassConfig>,
) {
    for (entity, grass, gpu_info, mut cull_chunks, mesh, lod_mesh, material, _visibility) in
        &mut q_grass
    {
        let index_count = meshes.get(mesh).unwrap().indices().unwrap().len() as u32;
        let lod_index_count = if let Some(lod_mesh) = &lod_mesh.0 {
            Some(meshes.get(lod_mesh).unwrap().indices().unwrap().len() as u32)
        } else {
            None
        };

        let chunk_min = gpu_info.aabb.min;
        let chunk_max = chunk_min + gpu_info.chunk_size;

        let aabb = Aabb::from_min_max(
            Vec3::new(
                chunk_min.x,
                -grass.height_map.as_ref().unwrap().scale,
                chunk_min.y,
            ),
            Vec3::new(
                chunk_max.x,
                grass.height_map.as_ref().unwrap().scale,
                chunk_max.y,
            ),
        );

        let mut new_chunks = Vec::new();

        for x in 0..grass.chunk_count.x {
            'chunk: for z in 0..grass.chunk_count.y {
                let chunk_pos = UVec2::new(x, z);
                let world_pos = Vec3::new(
                    x as f32 * gpu_info.chunk_size.x,
                    0.0,
                    z as f32 * gpu_info.chunk_size.y,
                );

                for (transform, frustum) in camera_query.iter() {
                    // TODO: Seperate distance/frustum culling and keep chunks in distance loaded but tell prepare to not create bind groups
                    if ((Vec3::from(aabb.center) + world_pos).xz() - transform.translation.xz())
                        .length()
                        < grass_config.cull_distance
                        && frustum.intersects_obb(
                            &aabb,
                            &Affine3A::from_translation(world_pos),
                            false,
                            false,
                        )
                    {
                        if !cull_chunks.0.contains_key(&chunk_pos) {
                            let chunk_aabb = Aabb2d {
                                min: aabb.min().xz() + world_pos.xz(),
                                max: aabb.max().xz() + world_pos.xz(),
                            };

                            let chunk_entity = commands
                                .spawn((
                                    GrassChunk {
                                        grass_entity: entity,
                                        aabb: chunk_aabb,
                                        instance_count: gpu_info.instance_count,
                                        scan_workgroup_count: gpu_info.scan_workgroup_count,
                                    },
                                    GrassChunkBuffers::create_buffers(
                                        &render_device,
                                        chunk_aabb,
                                        gpu_info.instance_count,
                                        index_count,
                                        gpu_info.scan_workgroup_count,
                                        lod_index_count,
                                        grass_config.grass_shadows.enabled(),
                                    ),
                                    mesh.clone(),
                                    lod_mesh.clone(),
                                    material.clone(),
                                    NoFrustumCulling,
                                    NoAutomaticBatching,
                                ))
                                .id();

                            cull_chunks.0.insert(chunk_pos, chunk_entity);
                            new_chunks.push(chunk_entity);
                        }

                        continue 'chunk;
                    } else {
                        if let Some(chunk_entity) = cull_chunks.0.remove(&chunk_pos) {
                            commands.entity(entity).remove_children(&[chunk_entity]);
                            commands.entity(chunk_entity).despawn();
                        }
                    }
                }
            }
        }

        commands
            .entity(entity)
            .insert_children(0, new_chunks.as_slice());
    }
}


// pub fn create_wind_map(mut wind: ResMut<GrassWind>, asset_server: Res<AssetServer>) {
//     wind.wind_map = asset_server.add(GrassWind::generate_wind_map(2048, 4.));
// }

pub(crate) fn prepare_clump(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
    clumps: Res<GrassClumps>,
    clump_config: Res<GrassClumpConfig>,
    pipeline: Res<GrassGeneratePipeline>,
    clump_bind_group: Option<Res<GrassClumpsBindGroup>>,
) {
    if clump_bind_group.is_some() {
        return;
    }

    let aabb_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
        label: Some("clump_aabb_buffer"),
        contents: &bytemuck::cast_slice(&[Aabb2dGpu::from(clump_config.aabb)]),
        usage: BufferUsages::UNIFORM,
    });
    let clump_size_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
        label: Some("clump_size_buffer"),
        contents: &bytemuck::cast_slice(&[clumps.cell_size]),
        usage: BufferUsages::UNIFORM,
    });
    let positions_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
        label: Some("clump_positions_buffer"),
        contents: &bytemuck::cast_slice(clumps.positions.as_slice()),
        usage: BufferUsages::STORAGE,
    });
    let params_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
        label: Some("clump_params_buffer"),
        contents: &bytemuck::cast_slice(clumps.params.as_slice()),
        usage: BufferUsages::STORAGE,
    });
    let bind_group = render_device.create_bind_group(
        Some("clump_bind_group"),
        &pipeline.clump_layout,
        &BindGroupEntries::sequential((
            aabb_buffer.as_entire_binding(),
            clump_size_buffer.as_entire_binding(),
            positions_buffer.as_entire_binding(),
            params_buffer.as_entire_binding(),
        )),
    );

    commands.insert_resource(GrassClumpsBindGroup {
        _positions_buffer: positions_buffer,
        _params_buffer: params_buffer,
        bind_group,
    });
}
