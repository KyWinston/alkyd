use bevy::prelude::*;

use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;

#[derive(Reflect, Default, Resource, Debug, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct MaterialsInspector {
    pub turn_table: bool,
    pub painterly: PainterlyInspector,
}

#[derive(Resource, Debug)]
pub struct VoronoiImage(pub [Vec4; 64*64]);

#[derive(Reflect, Resource, Debug, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct PainterlyInspector {
    pub diffuse_color: Color,
    pub roughness: f32,
    pub metallic: f32,
    pub color_varience: f32,
    pub scale: f32,
    pub distort: f32,
    pub influence: f32,
}

impl Default for PainterlyInspector {
    fn default() -> Self {
        Self {
            diffuse_color: Color::BLUE,
            roughness: 0.2,
            metallic: 0.0,
            color_varience: 0.7,
            scale: 5.0,
            distort: 0.3,
            influence: 0.5,
        }
    }
}
