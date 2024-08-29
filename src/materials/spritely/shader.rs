use bevy::{
    asset::{Asset, Handle},
    math::{Dir2, Dir3, Vec2, Vec3},
    pbr::Material,
    prelude::{AlphaMode, *},
    reflect::TypePath,
    render::{
        render_asset::RenderAssets,
        render_resource::{AsBindGroup, AsBindGroupShaderType, ShaderRef, ShaderType},
        texture::GpuImage,
    },
};

use crate::SPRITELY_HANDLE;

#[derive(Asset, TypePath, AsBindGroup, Clone)]
#[uniform(0, SpritelyUniform)]
pub struct SpritelyMaterial {
    pub directions: u32,
    pub viewing_direction: Dir3,
    pub looking_direction: Dir2,
    pub current_frame: u32,
    pub animation_indices: [u32; 4],
    #[texture(1)]
    #[sampler(2)]
    pub sheet_mask: Option<Handle<Image>>,
    #[texture(3)]
    #[sampler(4)]
    pub color_uv: Option<Handle<Image>>,
    #[texture(5)]
    #[sampler(6)]
    pub normal_map: Option<Handle<Image>>,
    #[texture(7)]
    #[sampler(8)]
    pub ao_map: Option<Handle<Image>>,
    #[texture(9)]
    #[sampler(10)]
    pub volume_map: Option<Handle<Image>>
}

/// the properties necessary to make an animated sprite
/// viewing directions relates to the number of directions on the sprite-sheet, not the number of directions supported.
impl Default for SpritelyMaterial {
    fn default() -> Self {
        Self {
            directions: 4,
            viewing_direction: Dir3::X,
            looking_direction: Dir2::X,
            current_frame: 0,
            animation_indices: [0, 0, 8, 43],
            sheet_mask: None,
            color_uv: None,
            normal_map: None,
            ao_map: None,
            volume_map: None,
        }
    }
}

#[derive(Clone, Default, ShaderType)]
pub struct SpritelyUniform {
    pub sheet_dimensions_x: u32,
    pub sheet_dimensions_y: u32,
    pub directions: u32,
    pub viewing_direction: Vec3,
    pub looking_direction: Vec2,
    pub current_frame: u32,
    pub start_frame: Vec2,
}

impl Material for SpritelyMaterial {
    fn fragment_shader() -> ShaderRef {
        SPRITELY_HANDLE.into()
    }
    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

impl AsBindGroupShaderType<SpritelyUniform> for SpritelyMaterial {
    fn as_bind_group_shader_type(&self, _: &RenderAssets<GpuImage>) -> SpritelyUniform {
        SpritelyUniform {
            sheet_dimensions_x: self.animation_indices[2],
            sheet_dimensions_y: self.animation_indices[3],
            directions: self.directions,
            viewing_direction: *self.viewing_direction,
            looking_direction: *self.looking_direction,
            current_frame: self.current_frame,
            start_frame: Vec2::new(
                self.animation_indices[0] as f32,
                self.animation_indices[1] as f32,
            ),
        }
    }
}
