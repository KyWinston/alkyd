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
    // let mut reshape = vec![vec![Vec4::ZERO; 10]; 10];
    // for (idx, res) in result.iter().enumerate() {
    //     let y = (idx as f32 / 10.0).floor() as usize;
    //     let x = (idx as f32 / 10.0).fract() as usize;
    //     reshape[x][y] = *res;
    // }
    // let mut poisson = true;
    // for (idy, y) in reshape.iter().enumerate() {
    //     for (idx, x) in y.iter().enumerate() {
    //         for i in 0..2 {
    //             for j in 0..2 {
    //                 let mut offset = Vec2::new(j as f32 - 1.0, i as f32 - 1.0);
    //                 if idx == 0 {
    //                     offset.x = i as f32;
    //                 }
    //                 if idy == 0 {
    //                     offset.y = j as f32;
    //                 }
    //                 if offset.x != 0.0 || offset.y != 0.0 {
    //                     let dist = (Vec2::new(idx as f32, idy as f32) + x.xy()).distance(
    //                         Vec2::new(idx as f32 + offset.x, idy as f32 + offset.y)
    //                             + reshape[idx + offset.x as usize][idy + offset.y as usize].xy(),
    //                     );

    //                     if dist < 0.4 {
    //                         poisson = false;
    //                         voro_img.0[idx + idy * 10] = Vec4::NEG_ONE;
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }

    // if poisson {
    voro_img.0 = result;
    // } else {
    load_ev.send(LoadNoise);
    // }
}
