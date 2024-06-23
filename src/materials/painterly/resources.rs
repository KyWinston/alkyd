use bevy::{color::palettes::css::GRAY, prelude::*};

#[derive(Reflect, Default, Resource, Debug)]
#[reflect(Resource)]
pub struct MaterialsInspector {
    pub turn_table: bool,
    pub painterly: PainterlyInspector,
}

#[derive(Resource, Debug)]
pub struct VoronoiImage(pub [Vec4; 100]);

#[derive(Reflect, Resource, Debug)]
#[reflect(Resource)]
pub struct PainterlyInspector {
    pub diffuse_color: Color,
    pub roughness: f32,
    pub metallic: f32,
    pub color_varience: f32,
    pub scale: Vec3,
    pub distort: f32,
    pub influence: f32,
    pub border: f32,
    pub dist_falloff: f32,
    pub detail_cutoff: f32,
}

impl Default for PainterlyInspector {
    fn default() -> Self {
        Self {
            diffuse_color: Color::srgb_from_array(GRAY.to_f32_array_no_alpha()),
            roughness: 0.2,
            metallic: 0.0,
            color_varience: 0.7,
            scale: Vec2::splat(2.0).extend(0.2),
            distort: 0.3,
            influence: 0.5,
            border: 0.02,
            dist_falloff: 5.0,
            detail_cutoff: 1.9,
        }
    }
}
