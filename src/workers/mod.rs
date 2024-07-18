use bevy::prelude::*;
use systems::setup;

pub mod systems;

pub struct WorkersPlugin;

impl Plugin for WorkersPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).run();
    }
}
