use std::path::Path;

use bevy::{
    asset::{embedded_asset, io::AssetSourceId, AssetPath},
    prelude::*,
};

pub struct EmbeddedAssetPlugin;

impl Plugin for EmbeddedAssetPlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "src", "../utilities/noise.wgsl");
        app.add_systems(Startup, setup);
    }
}

fn setup() {
    // Each example is its own crate (with name from [[example]] in Cargo.toml).
    let crate_name = "alkyd";

    let path = Path::new(crate_name).join("noise.png");
    let source = AssetSourceId::from("embedded");
    let asset_path = AssetPath::from_path(&path).with_source(source);

    assert_eq!(asset_path, "embedded://alkyd/noise.png".into());
}
