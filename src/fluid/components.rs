use bevy::prelude::*;

#[derive(Component)]
#[require(Transform)]
pub struct FluidVolume {
    pub(crate) particle_amount: usize,
    pub(crate) bounds_size: Vec3,
}

impl FluidVolume {
    pub fn new(particle_amount: usize, bounds_size: Vec3) -> Self {
        Self {
            particle_amount,
            bounds_size,
        }
    }
}

#[derive(Component)]
pub struct VolumeFilling;

#[derive(Component)]
pub struct VolumeDebug;

#[derive(Component)]
#[require(Transform)]
pub struct FluidParticle {
    pub(crate) parent_volume: Entity,
    pub(crate) velocity: Vec3,
    pub(crate) mass: f32,
    pub(crate) density: f32,
    pub(crate) pressure: Vec3,
    pub(crate) smoothing_radius: f32,
}

impl FluidParticle {
    pub fn new(parent_volume: Entity, mass: f32, smoothing_radius: f32) -> Self {
        Self {
            parent_volume,
            velocity: Vec3::ZERO,
            mass,
            density: 0.0,
            pressure: Vec3::ZERO,
            smoothing_radius,
        }
    }
}
