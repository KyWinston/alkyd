use super::candle_flame::CandleFlameMaterial;
use bevy::prelude::*;

pub fn update_material(
    mut fog: ResMut<Assets<CandleFlameMaterial>>,
    alkyd_q: Query<(Entity, &MeshMaterial3d<CandleFlameMaterial>)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if let Ok((_, alk_handle)) = alkyd_q.get_single() {
        if let Some(mat) = fog.get_mut(alk_handle.id()) {
            if input.just_pressed(KeyCode::ArrowUp) {
                mat.steps += 1;
            }
            if input.just_pressed(KeyCode::ArrowDown) {
                mat.steps -= 1;
            }
            if input.just_pressed(KeyCode::ArrowLeft) {
                mat.props.octaves -= 1;
            }
            if input.just_pressed(KeyCode::ArrowRight) {
                mat.props.octaves += 1;
            }
            if input.just_pressed(KeyCode::KeyQ) {
                mat.props.lacunarity -= 0.1;
            }
            if input.just_pressed(KeyCode::KeyE) {
                mat.props.lacunarity += 0.1;
            }
            if input.just_pressed(KeyCode::KeyS) {
                mat.precision -= 1.0;
            }
            if input.just_pressed(KeyCode::KeyW) {
                mat.precision += 1.0;
            }
        }
    }
}
