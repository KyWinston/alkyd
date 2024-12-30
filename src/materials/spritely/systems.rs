use bevy::prelude::*;

use super::{
    components::{AnimationIndices, AnimationTimer},
    shader::SpritelyMaterial,
};

pub fn animate(
    mut anim_q: Query<(
        &mut AnimationTimer,
        &AnimationIndices,
        &mut MeshMaterial3d<SpritelyMaterial>,
    )>,
    mut materials: ResMut<Assets<SpritelyMaterial>>,
    time: Res<Time>,
) {
    for (mut timer, anim, anim_handle) in anim_q.iter_mut() {
        if let Some(mat) = materials.get_mut(anim_handle.id()) {
            timer.tick(time.delta());
            let start_indices = anim.animations[&anim.current_animation].start_indices;
            let dimensions = anim.frame_dimensions;
            mat.animation_indices = [
                start_indices[0] as u32,
                start_indices[1] as u32,
                dimensions[0],
                dimensions[1],
            ];
            mat.looking_direction = Dir2::new_unchecked(anim.current_facing.normalize().xy());
            if timer.just_finished() {
                mat.current_frame += 1;
                if mat.current_frame >= anim.animations[&anim.current_animation].anim_length {
                    mat.current_frame = 0
                }
            }
        }
    }
}
