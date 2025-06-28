use bevy::{asset::load_internal_asset, prelude::*, render::render_resource::Shader};

use crate::{shells::node::ShellProperties, SHELL_GEN_HANDLE};


pub mod node;

pub struct ShellsPlugin;

impl Plugin for ShellsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ShellProperties>();
        load_internal_asset!(
            app,
            SHELL_GEN_HANDLE,
            "../../assets/shader_utils/shells.wgsl",
            Shader::from_wgsl
        );
    }
}
