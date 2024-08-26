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

#[derive(Bundle)]
pub struct AnimationBundle {
    indices: AnimationIndices,
    timer: AnimationTimer,
}

impl AnimationBundle {
    pub fn new(
        frame_dimensions: [u32; 2],
        facing_angles: usize,
        animations: HashMap<String, Animation>,
        frame_rate: u32,
    ) -> Self {
        Self {
            indices: AnimationIndices {
                frame_dimensions,
                facing_angles,
                current_facing: Dir2::X,
                animations,
                current_animation: "idle".to_string(),
                current_frame: 0,
            },
            timer: AnimationTimer(Timer::from_seconds(
                1.0 / frame_rate as f32,
                TimerMode::Repeating,
            )),
        }
    }
}
