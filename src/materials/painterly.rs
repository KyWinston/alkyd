use bevy::{
    prelude::*,
    render::{
        render_asset::RenderAssets,
        render_resource::{AsBindGroup, AsBindGroupShaderType, ShaderRef, ShaderType},
    },
};

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
#[reflect(Default, Debug)]
#[uniform(0, PainterlyUniform)]
pub struct Painterly {
    pub diffuse_color: Color,
    pub brush_distortion: f32,
    pub brush_blur: f32,
    pub brush_angle: f32,
}

impl Default for Painterly {
    fn default() -> Self {
        Painterly {
            diffuse_color: Color::BLUE,
            brush_distortion: 0.6,
            brush_blur: 0.0,
            brush_angle: 1.0,
        }
    }
}

impl From<Color> for Painterly {
    fn from(color: Color) -> Self {
        Painterly {
            diffuse_color: color,
            ..Default::default()
        }
    }
}

#[derive(Clone, Default, ShaderType)]
pub struct PainterlyUniform {
    pub diffuse_color: Vec4,
    pub brush_distortion: f32,
    pub brush_blur: f32,
    pub brush_angle: f32,
}

impl Material for Painterly {
    fn fragment_shader() -> ShaderRef {
        "painterly_material.wgsl".into()
    }
    fn deferred_fragment_shader() -> ShaderRef {
        "painterly_material.wgsl".into()
    }
}

impl AsBindGroupShaderType<PainterlyUniform> for Painterly {
    fn as_bind_group_shader_type(&self, _: &RenderAssets<Image>) -> PainterlyUniform {
        PainterlyUniform {
            diffuse_color: self.diffuse_color.as_linear_rgba_f32().into(),
            brush_distortion: self.brush_distortion,
            brush_blur: self.brush_blur,
            brush_angle: self.brush_angle,
        }
    }
}
