use super::components::Showcase;
use super::painterly::PainterlyMaterial;
use bevy::prelude::*;

use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;

#[derive(Reflect, Default, Resource, Debug, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct MaterialsInspector {
    pub turn_table: bool,
    painterly: PainterlyInspector,
}

#[derive(Reflect, Default, Resource, Debug, InspectorOptions)]
pub struct NoiseCache(Vec<Handle<Image>>);

#[derive(Reflect, Default, Resource, Debug, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct PainterlyInspector {
    diffuse_color: Color,
    roughness: f32,
    normal_strength: f32,
    metallic: f32,
    brush_distortion: f32,
    brush_blur: f32,
    brush_angle: f32,
    brush_texture_influence: f32,
    color_varience: f32,
    noise_scale: f32,
}

pub fn init_material(
    mut commands: Commands,
    mut my_res: ResMut<MaterialsInspector>,
    paint_q: Res<Assets<PainterlyMaterial>>,
    alkyd_q: Query<(Entity, &Handle<PainterlyMaterial>)>,
) {
    if my_res.is_added() {
        if let Ok((ent, alk_handle)) = alkyd_q.get_single() {
            if let Some(mat) = paint_q.get(alk_handle.id()) {
                my_res.painterly.brush_angle = mat.brush_angle;
                my_res.painterly.diffuse_color = mat.diffuse_color;
                my_res.painterly.brush_distortion = mat.brush_distortion;
                my_res.painterly.brush_blur = mat.brush_blur;
                my_res.painterly.color_varience = mat.color_varience;
                my_res.painterly.metallic = mat.metallic;
                my_res.painterly.normal_strength = mat.normal_strength;
                my_res.painterly.noise_scale = mat.noise_scale;
                my_res.painterly.brush_texture_influence = mat.brush_texture_influence;
            }
            commands.entity(ent).insert(Showcase);
        }
    }
}

pub fn material_changed(
    my_res: ResMut<MaterialsInspector>,
    mut paint_q: ResMut<Assets<PainterlyMaterial>>,
    alkyd_q: Query<&Handle<PainterlyMaterial>, With<Showcase>>,
) {
    if my_res.is_changed() && !my_res.is_added() {
        if let Ok(alk_handle) = alkyd_q.get_single() {
            if let Some(mat) = paint_q.get_mut(alk_handle.id()) {
                let src_mat = &my_res.painterly;
                mat.diffuse_color = src_mat.diffuse_color;
                mat.brush_distortion = src_mat.brush_distortion;
                mat.normal_strength = src_mat.normal_strength;
                mat.brush_blur = src_mat.brush_blur;
                mat.brush_angle = src_mat.brush_angle;
                mat.brush_texture_influence = src_mat.brush_texture_influence;
                mat.color_varience = src_mat.color_varience;
                mat.roughness = src_mat.roughness;
                mat.metallic = src_mat.metallic;
                mat.noise_scale = src_mat.noise_scale;
            }
        }
    }
}
