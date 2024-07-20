use bevy::{asset::Handle, prelude::Resource, render::texture::Image};

#[derive(Resource)]
pub struct NoiseImage(pub Handle<Image>);
