use bevy::{asset::load_internal_asset, prelude::*, render::render_resource::Shader};

use crate::{
    SHELL_GEN_HANDLE,
    shells::node::{ShellHandle, ShellProperties},
    workers::distribute_points::distribute_points,
};

pub mod node;

pub struct ShellsPlugin;

impl Plugin for ShellsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ShellProperties>().add_systems(
            Update,
            distribute_points.run_if(resource_added::<ShellHandle>),
        );

        load_internal_asset!(
            app,
            SHELL_GEN_HANDLE,
            "../../assets/shader_utils/shells.wgsl",
            Shader::from_wgsl
        );
    }
}
