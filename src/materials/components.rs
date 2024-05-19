use bevy::{prelude::*, render::render_graph::RenderLabel};

#[derive(Component)]
pub struct Showcase;

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
pub struct VoronoiLabel;
