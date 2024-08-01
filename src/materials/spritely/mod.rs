use bevy::prelude::*;

use self::systems::animate;

pub mod components;
pub mod resources;
pub mod shader;
pub mod systems;

pub struct SpritelyPlugin;

impl Plugin for SpritelyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate);
    }
}
