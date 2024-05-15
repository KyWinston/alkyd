use bevy::{
    prelude::*,
    render::{
        render_asset::{RenderAssetUsages, RenderAssets},
        render_resource::{
            BindGroupEntries, Extent3d, TextureDimension, TextureFormat, TextureUsages,
        },
        renderer::RenderDevice,
    },
};

use super::{
    components::OffCam,
    resources::{NoiseImageBindGroups, NoiseImages, NoisePipeline},
    SIZE,
};

pub fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let mut image = Image::new_fill(
        Extent3d {
            width: SIZE.0,
            height: SIZE.1,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[0, 0, 0, 255],
        TextureFormat::Rgba8Unorm,
        RenderAssetUsages::RENDER_WORLD,
    );
    image.texture_descriptor.usage =
        TextureUsages::COPY_DST | TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING;
    let image0 = images.add(image.clone());
    let image1 = images.add(image);

    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                order: -1,
                ..default()
            },
            ..default()
        },
        OffCam,
    ));
    commands.insert_resource(NoiseImages(vec![image0, image1]))
}

pub fn queue_bind_group(
    mut commands: Commands,
    pipeline: Res<NoisePipeline>,
    gpu_images: Res<RenderAssets<Image>>,
    noise_images: Res<NoiseImages>,
    render_device: Res<RenderDevice>,
) {
    let view_a = gpu_images.get(&noise_images.0[0]).unwrap();
    let view_b = gpu_images.get(&noise_images.0[1]).unwrap();
    let bind_group_0 = render_device.create_bind_group(
        None,
        &pipeline.texture_bind_group_layout,
        &BindGroupEntries::sequential((&view_a.texture_view, &view_b.texture_view)),
    );
    let bind_group_1 = render_device.create_bind_group(
        None,
        &pipeline.texture_bind_group_layout,
        &BindGroupEntries::sequential((&view_b.texture_view, &view_a.texture_view)),
    );
    commands.insert_resource(NoiseImageBindGroups([bind_group_0, bind_group_1]));
}
