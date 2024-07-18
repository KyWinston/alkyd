use bevy::prelude::*;

use super::{components::Showcase, painterly::PainterlyMaterial, resources::{MaterialsInspector, VoronoiImage}};

pub fn update_material(
    mut my_res: ResMut<MaterialsInspector>,
    mut paint_q: ResMut<Assets<PainterlyMaterial>>,
    alkyd_q: Query<(Entity, &Handle<PainterlyMaterial>)>,
    voro: Res<VoronoiImage>,
) {
    if my_res.is_added() {
        if let Ok((_, alk_handle)) = alkyd_q.get_single() {
            if let Some(mat) = paint_q.get_mut(alk_handle.id()) {
                my_res.painterly.diffuse_color = mat.diffuse_color;
                my_res.painterly.color_varience = mat.color_varience;
                my_res.painterly.roughness = mat.roughness;
                my_res.painterly.metallic = mat.metallic;
                my_res.painterly.distort = mat.distort;
                my_res.painterly.influence = mat.influence;
                my_res.painterly.color_varience = mat.color_varience;
                my_res.painterly.scale = mat.scale;
                my_res.painterly.border = mat.border;
                my_res.painterly.dist_falloff = mat.dist_falloff;
                my_res.painterly.detail_cutoff = mat.detail_cutoff;
                mat.voro_cache = voro.0.clone();
            }
        }
    }
}

pub fn material_changed(
    my_res: ResMut<MaterialsInspector>,
    voro: Res<VoronoiImage>,
    mut paint_q: ResMut<Assets<PainterlyMaterial>>,
    alkyd_q: Query<&Handle<PainterlyMaterial>, With<Showcase>>,
) {
    if my_res.is_changed() && !my_res.is_added() {
        if let Ok(alk_handle) = alkyd_q.get_single() {
            if let Some(mat) = paint_q.get_mut(alk_handle.id()) {
                let src_mat = &my_res.painterly;
                mat.diffuse_color = src_mat.diffuse_color;
                mat.color_varience = src_mat.color_varience;
                mat.roughness = src_mat.roughness;
                mat.metallic = src_mat.metallic;
                mat.distort = src_mat.distort;
                mat.influence = src_mat.influence;
                mat.color_varience = src_mat.color_varience;
                mat.scale = src_mat.scale;
                mat.border = src_mat.border;
                mat.dist_falloff = src_mat.dist_falloff;
                mat.detail_cutoff = src_mat.detail_cutoff;
                mat.voro_cache = voro.0;
            }
        }
    }
}
