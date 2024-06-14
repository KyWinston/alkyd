use bevy::prelude::*;

#[derive(Component)]
pub struct AnimSprite {
    pub look_direction: Dir2,
}

impl Default for AnimSprite {
    fn default() -> Self {
        Self {
            look_direction: Dir2::X,
        }
    }
}

#[derive(Component, Clone, Copy)]
pub struct AnimationIndices {
    pub facing_angles: usize,
    pub current_facing: Dir2,
    pub animations: AnimationStates,
    pub current_frame: usize,
}

#[derive(Copy, Clone)]
pub struct AnimationStates {
    pub idle: Animation,
    pub movement: Option<Animation>,
}

#[derive(Copy, Clone)]
pub struct Animation {
    pub start_indices: [usize; 2],
    pub anim_length: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Bundle)]
pub struct AnimationBundle {
    indices: AnimationIndices,
    timer: AnimationTimer,
}

impl AnimationBundle {
    pub fn new(facing_angles: usize, animations: AnimationStates, frame_rate: u32) -> Self {
        Self {
            indices: AnimationIndices {
                facing_angles,
                current_facing: Dir2::X,
                animations,
                current_frame: 0,
            },
            timer: AnimationTimer(Timer::from_seconds(
                1.0 / frame_rate as f32,
                TimerMode::Repeating,
            )),
        }
    }
}

