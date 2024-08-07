use bevy::{
    asset::{Asset, Handle}, math::{Dir2, Dir3, Vec2, Vec3}, pbr::Material, prelude::{AlphaMode, *}, reflect::TypePath, render::{
            render_asset::RenderAssets,
            render_resource::{AsBindGroup, AsBindGroupShaderType, ShaderRef, ShaderType},
            texture::GpuImage,
        }
};

use crate::SPRITELY_HANDLE;

#[derive(Asset, TypePath, AsBindGroup, Clone)]
#[uniform(0, SpritelyUniform)]
pub struct SpritelyMaterial {
    pub directions: u32,
    pub viewing_direction: Dir3,
    pub looking_direction: Dir2,
    pub current_frame: u32,
    pub frames: u32,
    pub frames_per_second: u32,
    pub uv_scale: u32,
    #[texture(1)]
    #[sampler(2)]
    pub sheet_depth_uv_mask: Option<Handle<Image>>,
    #[texture(3)]
    #[sampler(4)]
    pub color_uv: Option<Handle<Image>>,
    #[texture(5)]
    #[sampler(6)]
    pub normals: Option<Handle<Image>>,
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
            frames: 43,
            frames_per_second: 24,
            uv_scale: 4,
            sheet_depth_uv_mask: None,
            color_uv: None,
            normals:None
        }
    }
}

#[derive(Clone, Default, ShaderType)]
pub struct SpritelyUniform {
    pub directions: u32,
    pub viewing_direction: Vec3,
    pub looking_direction: Vec2,
    pub current_frame: u32,
    pub frames: u32,
    pub frames_per_second: u32,
    pub uv_scale: u32,
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
            directions: self.directions,
            viewing_direction: *self.viewing_direction,
            looking_direction: *self.looking_direction,
            current_frame: self.current_frame,
            frames: self.frames,
            frames_per_second: self.frames_per_second,
            uv_scale: self.uv_scale,
        }
    }
}
