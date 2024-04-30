use super::painterly::Painterly;
use bevy::prelude::*;

use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;

#[derive(Reflect, Default, Resource, Debug, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct MaterialsInspector {
    pub turn_table: bool,
    painterly: PainterlyInspector,
}

#[derive(Reflect, Resource, Debug, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct PainterlyInspector {
    view_normals: bool,
    diffuse_color: Color,
    tiling_period: Vec3,
    #[inspector(min = 0.0, max = 1.0)]
    pub roughness: f32,
    #[inspector(min = 0.0, max = 1.0)]
    pub metallic: f32,
    #[inspector(min = 0.0, max = 1.0)]
    // brush_blur: f32,
    // brush_distortion: f32,
    #[inspector(min = 0.0, max = 1.0)]
    // brush_angle: f32,
    // pub brush_texture_influence: f32,
    pub noise_scale: f32,
    pub color_varience: f32,
}

impl Default for PainterlyInspector {
    fn default() -> Self {
        Self {
            view_normals: false,
            diffuse_color: Color::BLUE,
            tiling_period: Vec3::splat(4.0),
            // brush_blur: 20.0,
            // brush_distortion: 34.0,
            // brush_angle: 15.6,
            // brush_texture_influence: 20.0,
            color_varience: 0.5,
            roughness: 15.0,
            metallic: 0.0,
            noise_scale: 5.0,
        }
    }
}

pub fn material_changed(
    my_res: ResMut<MaterialsInspector>,
    mut paint_q: ResMut<Assets<Painterly>>,
    alkyd_q: Query<&Handle<Painterly>>,
) {
    if my_res.is_changed() {
        if let Ok(alk_handle) = alkyd_q.get_single() {
            if let Some(mat) = paint_q.get_mut(alk_handle.id()) {
                let src_mat = &my_res.painterly;
                mat.view_normals = src_mat.view_normals;
                mat.diffuse_color = src_mat.diffuse_color;
                // mat.brush_distortion = src_mat.brush_distortion;
                // mat.brush_texture_influence = src_mat.brush_texture_influence;
                mat.color_varience = src_mat.color_varience;
                mat.roughness = src_mat.roughness;
                mat.metallic = src_mat.metallic;
                mat.tiling_period = src_mat.tiling_period;
                mat.noise_scale = src_mat.noise_scale;
            }
        }
    }
}
