use bevy::{
    prelude::*,
    render::{
        render_asset::RenderAssets,
        render_resource::{AsBindGroup, AsBindGroupShaderType, ShaderRef, ShaderType},
    },
};

#[derive(Asset, AsBindGroup, Default, Reflect, Debug, Clone)]
#[reflect(Default, Debug)]
#[uniform(0, PainterlyUniform)]
pub struct Painterly {
    pub view_normals: bool,
    pub diffuse_color: Color,
    pub roughness: f32,
    pub normal_strength:f32,
    pub metallic: f32,
    pub brush_distortion: f32,
    pub brush_blur: f32,
    pub brush_angle: f32,
    pub brush_texture_influence: f32,
    pub color_varience: f32,
    pub noise_scale: f32,
    #[texture(1)]
    #[sampler(2)]
    pub brush_handle: Handle<Image>,
    #[texture(3)]
    #[sampler(4)]
    pub brush_handle_normal: Handle<Image>,
}

#[derive(Clone, Default, ShaderType)]
pub struct PainterlyUniform {
    pub view_normals: u32,
    pub diffuse_color: Vec4,
    pub normal_strength:f32,
    pub roughness: f32,
    pub metallic: f32,
    pub brush_distortion: f32,
    pub brush_blur: f32,
    pub brush_angle: f32,
    pub brush_texture_influence: f32,
    pub color_varience: f32,
    pub noise_scale: f32,
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
        let mut view_n = 0;
        if self.view_normals {
            view_n = 1;
        }
        PainterlyUniform {
            view_normals: view_n,
            diffuse_color: self.diffuse_color.as_linear_rgba_f32().into(),
            roughness: self.roughness,
            metallic: self.metallic,
            normal_strength: self.normal_strength,
            brush_distortion: self.brush_distortion,
            brush_blur: self.brush_blur,
            brush_angle: self.brush_angle,
            brush_texture_influence: self.brush_texture_influence,
            color_varience: self.color_varience,
            noise_scale: self.noise_scale,
        }
    }
}
