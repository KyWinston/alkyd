use bevy::{
    ecs::system::{lifetimeless::SRes, SystemParamItem},
    prelude::*,
    render::{
        render_asset::{PrepareAssetError, RenderAsset},
        render_resource::{Buffer, BufferInitDescriptor, BufferUsages},
        renderer::RenderDevice,
    },
};
use bytemuck::{Pod, Zeroable};

#[derive(Clone, Copy, Pod, Zeroable, Reflect, Debug)]
#[repr(C)]
pub struct GrassData {
    pub position: Vec3,
    pub normal: Vec3,
    pub chunk_uvw: Vec3,
}
#[derive(Component, Clone, Asset, TypePath)]
pub struct GrassChunkBuffer {
    pub buffer: Buffer,
    pub length: usize,
}

#[derive(Component, Default, Deref, Clone, Asset, TypePath)]
pub struct GrassChunkData(pub Vec<GrassData>);

impl RenderAsset for GrassChunkBuffer {
    type SourceAsset = GrassChunkData;
    type Param = SRes<RenderDevice>;

    fn prepare_asset(
        source_asset: Self::SourceAsset,
        param: &mut SystemParamItem<Self::Param>,
    ) -> Result<Self, PrepareAssetError<Self::SourceAsset>> {
        let render_device = param;

        let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(source_asset.as_slice()),
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST | BufferUsages::STORAGE,
        });

        Ok(Self {
            buffer,
            length: source_asset.len(),
        })
    }
}
