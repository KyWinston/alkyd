use bevy::{asset::Handle, prelude::Resource, render::texture::Image, utils::HashMap};

#[derive(Resource)]
pub struct NoiseImages(pub HashMap<String, [Handle<Image>; 2]>);
