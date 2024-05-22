use bevy::{
    pbr::{MaterialPipeline, MaterialPipelineKey},
    prelude::*,
    render::{
        mesh::MeshVertexBufferLayout,
        render_asset::RenderAssets,
        render_resource::{
            AsBindGroup, AsBindGroupShaderType, RenderPipelineDescriptor, ShaderRef, ShaderType,
            SpecializedMeshPipelineError,
        },
    },
};

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct PainterlyFlags: u8 {
        const BASE_COLOR_TEXTURE         = 0x01;
        const METALLIC_ROUGHNESS         = 0x02;
        const NORMAL_TEXTURE             = 0x04;
        const BRUSH_TEXTURE              = 0x08;
        const VARIANCE                   = 0x16;
        const NONE                       = 0;
        const UNINITIALIZED              = 0xF;
    }
}

impl Default for PainterlyFlags {
    fn default() -> Self {
        PainterlyFlags::NONE
    }
}

#[derive(Asset, TypePath, AsBindGroup, Clone)]
#[uniform(0, PainterlyUniform)]
#[bind_group_data(CustomMaterialKey)]
pub struct PainterlyMaterial {
    pub diffuse_color: Color,
    pub roughness: f32,
    pub metallic: f32,
    pub color_varience: f32,
    pub scale: f32,
    pub distort: f32,
    pub influence: f32,
    pub angle: f32,
    pub blur: f32,
    #[texture(1)]
    #[sampler(2)]
    pub brush_handle: Option<Handle<Image>>,
    #[texture(3)]
    #[sampler(4)]
    pub brush_handle_normal: Option<Handle<Image>>,
    #[texture(5)]
    #[sampler(6)]
    pub voro_cache: Option<Handle<Image>>,
}

impl Default for PainterlyMaterial {
    fn default() -> Self {
        Self {
            diffuse_color: Color::BLUE,
            roughness: 0.4,
            metallic: 0.0,
            color_varience: 0.5,
            scale: 2.0,
            distort: 3.3,
            influence: 0.5,
            angle: 0.4,
            blur: 0.05,
            brush_handle: None,
            brush_handle_normal: None,
            voro_cache: None,
        }
    }
}

#[derive(Clone, Default, ShaderType)]
pub struct PainterlyUniform {
    pub diffuse_color: Vec4,
    pub roughness: f32,
    pub metallic: f32,
    pub color_varience: f32,
    pub scale: f32,
    pub distort: f32,
    pub influence: f32,
    pub angle: f32,
    pub blur: f32,
}

impl Material for PainterlyMaterial {
    fn fragment_shader() -> ShaderRef {
        "embedded://alkyd/materials/painterly/painterly_material.wgsl".into()
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayout,
        key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        let fragment = descriptor.fragment.as_mut().unwrap();

        if key.bind_group_data.normal_texture {
            fragment.shader_defs.push("NORMAL_TEXTURE".into());
        }
        if key.bind_group_data.metallic_roughness {
            fragment.shader_defs.push("METALLIC_ROUGHNESS".into());
        }
        if key.bind_group_data.normal_texture {
            fragment.shader_defs.push("BRUSH_TEXTURE".into());
        }
        if key.bind_group_data.metallic_roughness {
            fragment.shader_defs.push("VARIANCE".into());
        }
        Ok(())
    }
}

impl AsBindGroupShaderType<PainterlyUniform> for PainterlyMaterial {
    fn as_bind_group_shader_type(&self, _: &RenderAssets<Image>) -> PainterlyUniform {
        PainterlyUniform {
            diffuse_color: self.diffuse_color.as_linear_rgba_f32().into(),
            roughness: self.roughness,
            metallic: self.metallic,
            color_varience: self.color_varience,
            scale: self.scale,
            distort: self.distort,
            influence: self.influence,
            angle: self.angle,
            blur: self.blur,
        }
    }
}

#[derive(ShaderType, Clone)]
pub struct VoronoiUniform {
    pub scale: f32,
    pub distort: f32,
    pub influence: f32,
    pub angle: f32,
    pub blur: f32,
}

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct CustomMaterialKey {
    pub normal_texture: bool,
    pub metallic_roughness: bool,
    pub brush_texture: bool,
    pub variance: bool,
}

impl From<&PainterlyMaterial> for CustomMaterialKey {
    fn from(material: &PainterlyMaterial) -> Self {
        let mut flags = PainterlyFlags::NONE;

        if material.metallic > 0.0 {
            flags |= PainterlyFlags::METALLIC_ROUGHNESS;
        }
        if material.color_varience > 0.0 {
            flags |= PainterlyFlags::VARIANCE;
        }

        Self {
            metallic_roughness: flags.contains(PainterlyFlags::METALLIC_ROUGHNESS),
            normal_texture: flags.contains(PainterlyFlags::NORMAL_TEXTURE),
            brush_texture: flags.contains(PainterlyFlags::BRUSH_TEXTURE),
            variance: flags.contains(PainterlyFlags::VARIANCE),
        }
    }
}
