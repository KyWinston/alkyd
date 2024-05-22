use bevy::prelude::*;

use super::{components::Showcase, painterly::PainterlyMaterial, resources::MaterialsInspector};

pub fn update_material(
    mut my_res: ResMut<MaterialsInspector>,
    mut paint_q: ResMut<Assets<PainterlyMaterial>>,
    alkyd_q: Query<(Entity, &Handle<PainterlyMaterial>)>,
) {
    if my_res.is_added() {
        if let Ok((_, alk_handle)) = alkyd_q.get_single() {
            if let Some(mat) = paint_q.get_mut(alk_handle.id()) {
                my_res.painterly.diffuse_color = mat.diffuse_color;
                my_res.painterly.color_varience = mat.color_varience;
                my_res.painterly.roughness = mat.roughness;
                my_res.painterly.metallic = mat.metallic;
                my_res.painterly.distort = mat.distort;
                my_res.painterly.blur = mat.blur;
                my_res.painterly.angle = mat.angle;
                my_res.painterly.influence = mat.influence;
                my_res.painterly.color_varience = mat.color_varience;
                my_res.painterly.scale = mat.scale;
            }
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
                mat.color_varience = src_mat.color_varience;
                mat.roughness = src_mat.roughness;
                mat.metallic = src_mat.metallic;
                mat.distort = src_mat.distort;
                mat.blur = src_mat.blur;
                mat.angle = src_mat.angle;
                mat.influence = src_mat.influence;
                mat.color_varience = src_mat.color_varience;
                mat.scale = src_mat.scale;
            }
        }
    }
}
