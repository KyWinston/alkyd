use bevy::{prelude::*, utils::HashMap};

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

#[derive(Component, Clone)]
pub struct AnimationIndices {
    pub frame_dimensions: [u32; 2],
    pub facing_angles: usize,
    pub current_facing: Dir2,
    pub animations: HashMap<String, Animation>,
    pub current_animation: String,
    pub current_frame: usize,
}

#[derive(Copy, Clone)]
pub struct Animation {
    pub start_indices: [usize; 2],
    pub anim_length: u32,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component)]
pub struct AnimationData;

impl AnimationData {
    pub fn new(
        current_animation: String,
        frame_dimensions: [u32; 2],
        facing_angles: usize,
        animations: HashMap<String, Animation>,
        frame_rate: u32,
    ) -> (Self, AnimationIndices, AnimationTimer) {
        (
            Self,
            AnimationIndices {
                frame_dimensions,
                facing_angles,
                current_facing: Dir2::X,
                animations,
                current_animation,
                current_frame: 0,
            },
            AnimationTimer(Timer::from_seconds(
                1.0 / frame_rate as f32,
                TimerMode::Repeating,
            )),
        )
    }
}
