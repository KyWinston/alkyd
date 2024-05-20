use super::VoronoiWorker;
use crate::{compute::worker::AppComputeWorker, materials::resources::VoronoiImage};
use bevy::{
    prelude::*,
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
    },
};

#[derive(Event)]
pub struct LoadNoise;

pub fn run_worker(
    mut load_ev: EventReader<LoadNoise>,
    mut compute_worker: ResMut<AppComputeWorker<VoronoiWorker>>,
) {
    for _ in load_ev.read() {
        compute_worker.execute();
    }
}

pub fn read_data(
    mut images: ResMut<Assets<Image>>,
    mut voro_img: ResMut<VoronoiImage>,
    compute_worker: ResMut<AppComputeWorker<VoronoiWorker>>,
) {
    if !compute_worker.ready() {
        return;
    };
    if voro_img.0.is_none() {
        let result: Vec<u8> = compute_worker.read_vec("texture");
        let voro = Image::new(
            Extent3d {
                width: 2048,
                height: 2048,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            result,
            TextureFormat::R32Float,
            RenderAssetUsages::RENDER_WORLD,
        );
        voro_img.0 = Some(images.add(voro));
    }
}
