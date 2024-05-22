fn main() {
    cargo_emit::rustc_env!("BEVY_ASSET_PATH", "{}", "assets");
}
