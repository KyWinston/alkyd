use super::painterly::Painterly;
use bevy::prelude::*;

use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;

#[derive(Reflect, Default, Resource, Debug, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct MaterialsInspector {
    painterly: PainterlyInspector,
}

#[derive(Reflect, Resource, Debug, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct PainterlyInspector {
    diffuse_color: Color,
    #[inspector(min = 0.0, max = 1.0)]
    brush_blur: f32,
    brush_distortion: f32,
    #[inspector(min = 0.0, max = 1.0)]
    brush_angle: f32,
}

impl Default for PainterlyInspector {
    fn default() -> Self {
        Self {
            diffuse_color: Color::BLUE,
            brush_blur: 0.0,
            brush_distortion: 4.0,
            brush_angle: 0.6,
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
                mat.brush_angle = my_res.painterly.brush_angle;
                mat.diffuse_color = my_res.painterly.diffuse_color;
                mat.brush_blur = my_res.painterly.brush_blur;
                mat.brush_distortion = my_res.painterly.brush_distortion;
            }
        }
    }
}
