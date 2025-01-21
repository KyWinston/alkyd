use bevy::{prelude::*, render::render_resource::ShaderType};
use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, ShaderType)]
pub struct FluidVolumeBuffer {
    pub(crate) id: u32,
    pub(crate) position: Vec3,
    pub(crate) particle_amount: u32,
    pub(crate) bounds_size: Vec3,
}

impl FluidVolumeBuffer {
    pub fn blank() -> Self {
        Self {
            id: 0,
            position: Vec3::ZERO,
            particle_amount: 0,
            bounds_size: Vec3::ZERO,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, ShaderType, Default)]
pub struct FluidParticleBuffer {
    pub local_position: Vec3,
    pub velocity: Vec3,
    pub mass: f32,
    pub pressure: f32,
    pub density: f32,
    pub force: Vec3,
    _padding: Vec4,
}

impl FluidParticleBuffer {
    pub fn new(local_position: Vec3) -> Self {
        Self {
            local_position,
            density: 0.0,
            mass: 1.0,
            _padding: Vec4::ZERO,
            velocity: Vec3::ZERO,
            ..default()
        }
    }
}
