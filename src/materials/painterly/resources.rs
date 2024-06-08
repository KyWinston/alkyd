use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

#[derive(Reflect, Default, Resource, Debug, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct MaterialsInspector {
    pub turn_table: bool,
    pub painterly: PainterlyInspector,
}

#[derive(Resource, Debug)]
pub struct VoronoiImage(pub [Vec4; 20 * 20 * 20]);

#[derive(Reflect, Resource, Debug, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
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
}

impl Default for PainterlyInspector {
    fn default() -> Self {
        Self {
            diffuse_color: Color::GRAY,
            roughness: 0.2,
            metallic: 0.0,
            color_varience: 0.7,
            scale: Vec3::splat(5.0),
            distort: 0.3,
            influence: 0.5,
            border: 0.02,
            dist_falloff: 20.0,
        }
    }
}
