use bevy::{asset::load_internal_asset, prelude::*, render::render_resource::Shader};

use crate::{FOLIAGE_GEN_HANDLE, foliage::node::FoliageProperties};

pub mod node;

pub struct FoliagePlugin;

impl Plugin for FoliagePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<FoliageProperties>();

        load_internal_asset!(
            app,
            FOLIAGE_GEN_HANDLE,
            "../../assets/shader_utils/foliage.wgsl",
            Shader::from_wgsl
        );
    }
}
