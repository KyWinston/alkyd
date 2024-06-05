use super::VoronoiWorker;
use crate::{compute::worker::AppComputeWorker, materials::painterly::resources::VoronoiImage};
use bevy::prelude::*;

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
    mut voro_img: ResMut<VoronoiImage>,
    mut load_ev: EventWriter<LoadNoise>,
    compute_worker: ResMut<AppComputeWorker<VoronoiWorker>>,
) {
    if !compute_worker.ready() {
        return;
    };

    let result: [Vec4; 20 * 20 * 20] = compute_worker
        .read_vec("centroids")
        .as_slice()
        .try_into()
        .unwrap();

    // if poisson {
    voro_img.0 = result;
    // } else {
    load_ev.send(LoadNoise);
    // }
}
