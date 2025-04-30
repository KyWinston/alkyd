use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::{
        render_asset::RenderAssets,
        render_resource::{
            BindGroupEntries, BufferInitDescriptor, BufferUsages, Extent3d, TextureDimension,
            TextureFormat, TextureUsages,
        },
        renderer::RenderDevice,
        texture::GpuImage,
    },
};

use super::{
    resources::{NoiseProperties, TexGenImage, TexGenImageBindGroup, TexGenPipeline},
    SIZE,
};

pub fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let mut image = Image::new_fill(
        Extent3d {
            width: SIZE,
            height: SIZE,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[0, 0, 0, 255],
        TextureFormat::R32Float,
        RenderAssetUsages::RENDER_WORLD,
    );
    image.texture_descriptor.usage =
        TextureUsages::COPY_DST | TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING;
    let image = images.add(image.clone());

    commands.insert_resource(TexGenImage {
        texture_0: image.clone(),
        texture_1: image,
    });
}

pub fn prepare_bind_group(
    mut commands: Commands,
    pipeline: Res<TexGenPipeline>,
    gpu_images: Res<RenderAssets<GpuImage>>,
    texture_gen_images: Res<TexGenImage>,
    render_device: Res<RenderDevice>,
) {
    let texture_a = gpu_images.get(&texture_gen_images.texture_0).unwrap();
    let texture_b = gpu_images.get(&texture_gen_images.texture_1).unwrap();

    let noise_props = render_device.create_buffer_with_data(&BufferInitDescriptor {
        label: None,
        contents: bytemuck::cast_slice(&[NoiseProperties {
            octaves: 2,
            lacunarity: 2.0,
            gain: 0.3,
            amplitude: 1.0,
            frequency: 1.0,
        }]),
        usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
    });
    let bind_group = render_device.create_bind_group(
        None,
        &pipeline.texture_bind_group_layout,
        &BindGroupEntries::sequential((
            &texture_a.texture_view,
            &texture_b.texture_view,
            noise_props.as_entire_binding(),
        )),
    );

    commands.insert_resource(TexGenImageBindGroup([bind_group]));
}
