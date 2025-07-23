use bevy::prelude::*;

#[derive(Component, Clone)]
#[require(Transform)]
pub struct FluidVolume {
    pub bounds_size: Vec3,
}

impl FluidVolume {
    pub fn new(bounds_size: Vec3) -> Self {
        Self { bounds_size }
    }
}

#[derive(Component)]
pub struct VolumeFilling;

#[derive(Component)]
pub struct VolumeDebug(pub Timer);

#[derive(Component)]
#[require(Transform)]
pub struct FluidParticle {
    pub parent_volume: Entity,
    pub velocity: Vec3,
    pub mass: f32,
    pub density: f32,
    pub pressure: f32,
    pub force: Vec3,
}

impl FluidParticle {
    pub fn new(parent_volume: Entity, mass: f32) -> Self {
        Self {
            parent_volume,
            velocity: Vec3::ZERO,
            mass,
            density: 0.0,
            pressure: 0.0,
            force: Vec3::ZERO,
        }
    }
}
