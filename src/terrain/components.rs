use bevy::prelude::*;

#[derive(Component, Clone)]
#[require(Transform)]
pub struct TerrainRoot;

#[derive(Component, Clone)]
pub struct TerrainChunk {
    pub id: usize,
    pub instances: u32,
}
