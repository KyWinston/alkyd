use bevy::ecs::component::Component;
use bevy::prelude::*;

#[derive(Component, Reflect, Clone, Default)]
#[reflect(Component)]
pub struct ShellProperties {
    pub length: f32,
    pub density: f32,
    pub thickness: f32,
    pub count: u32,
    pub variance: Vec2,
    pub color: Color,
}

#[derive(Reflect, Clone)]
pub enum ShellMode {
    PointCloud,
    SingleSheet,
}
