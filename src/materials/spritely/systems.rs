use bevy::prelude::*;

use super::{
    components::{AnimationIndices, AnimationTimer},
    shader::SpritelyMaterial,
};

pub fn animate(
    mut anim_q: Query<(
        &mut AnimationTimer,
        &AnimationIndices,
        &mut Handle<SpritelyMaterial>,
    )>,
    mut materials: ResMut<Assets<SpritelyMaterial>>,
    time: Res<Time>,
) {
    for (mut timer, anim, anim_handle) in anim_q.iter_mut() {
        if let Some(mat) = materials.get_mut(anim_handle.id()) {
            timer.tick(time.delta());
            mat.looking_direction = Dir2::new_unchecked(anim.current_facing.xy());
            if timer.just_finished() {
                mat.current_frame += 1;
            }
        }
    }
}
