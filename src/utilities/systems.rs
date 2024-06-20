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
    compute_worker: ResMut<AppComputeWorker<VoronoiWorker>>,
    mut load_ev: EventWriter<LoadNoise>,
) {
    if !compute_worker.ready() {
        return;
    };

    if let Ok(result) = compute_worker.read_vec("centroids").as_slice().try_into() {
        let mut new_vec: [Vec4; 100] = result;
        for v_ix in 0..9 {
            for v_iy in 0..9 {
                new_vec[v_ix as usize + v_iy as usize * 10] =
                    smallest_dist(&mut result.to_vec(), v_ix, v_iy);
            }
        }
        voro_img.0 = new_vec;
    }else{
        load_ev.send(LoadNoise);
    }
}

fn smallest_dist(points: &mut Vec<Vec4>, idx: i32, idy: i32) -> Vec4 {
    let mut min_dist = 1.0;
    for x in -1..1 {
        for y in -1..1 {
            let neighbor_dist = points[(idx + x).abs() as usize + (idy + y).abs() as usize * 10]
                .xy()
                .distance(points[idx as usize + idy as usize * 10].xy());
            if neighbor_dist < min_dist {
                min_dist = neighbor_dist;
            }
        }
    }
    points[idx as usize + idy as usize * 10].w = min_dist;
    points[idx as usize + idy as usize * 10]
}
