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

    let result: [Vec4; 100] = compute_worker
        .read_vec("centroids")
        .as_slice()
        .try_into()
        .unwrap();

    voro_img.0 = result;
    for v_ix in 0..10 {
        for v_iy in 0..10 {
            smallest_dist(voro_img.0.to_vec(), v_ix, v_iy);
        }
    }
    load_ev.send(LoadNoise);
}

fn smallest_dist(points: Vec<Vec4>, idx: usize, idy: usize) {
    let mut min_dist = 1.0;
    for x in 0..2 {
        for y in 0..2 {
            let neighbor_dist = points[idx + (x - 1) + (idy + y - 1) * 10]
                .xy()
                .distance(points[idx + idy * 10].xy());
            if neighbor_dist < min_dist {
                min_dist = neighbor_dist;
            }
        }
    }
}
