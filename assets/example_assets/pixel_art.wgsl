#import bevy_pbr::pbr_types::{STANDARD_MATERIAL_FLAGS_DOUBLE_SIDED_BIT, PbrInput, pbr_input_new},
#ifdef PREPASS_PIPELINE
#import bevy_pbr::{
    prepass_io::{VertexOutput, FragmentOutput},
    pbr_deferred_functions::deferred_output,
}
#else
#import bevy_pbr::{
    prepass_utils,
    forward_io::{VertexOutput, FragmentOutput},
    pbr_functions::{apply_pbr_lighting, main_pass_post_lighting_processing},
}
#endif


struct PixelArtMaterial{
    color:vec4<f32>,
    mettalic:f32,
    specular:f32
}


@group(2) @binding(0)
var<uniform> material: PixelArtMaterial;

@group(2) @binding(1)
var diff_tex: texture_2d<f32>;
@group(2) @binding(2)
var text_s: sampler;


@fragment
fn fragment(
    #ifdef MULTISAMPLED
    @builtin(sample_index) sample_index: u32,
    #endif
    mesh: VertexOutput,
) -> FragmentOutput {
    #ifndef MULTISAMPLED
        let sample_index = 0u;
    #endif
    let bayer = mat4x4( 0,  8,  2, 10,
  						12,  4, 14,  6,
   						 3, 11,  1,  9,
   						15,  7, 13,  5)/16.0;

    var pbr_input = pbr_input_new();
    pbr_input.material.base_color = material.color;
    #ifdef PREPASS_PIPELINE
        // in deferred mode we can't modify anything after that, as lighting is run in a separate fullscreen shader.
        let out = deferred_output(in, pbr_input);
    #else
    let depth = prepass_utils::prepass_depth(mesh.position, sample_index);
    let normal = prepass_utils::prepass_normal(mesh.position, sample_index);
    let depth_diff_back = depth - prepass_utils::prepass_depth(mesh.position - 1.0, sample_index);
    let depth_diff_forward = depth - prepass_utils::prepass_depth(mesh.position + 1.0, sample_index);
    var out: FragmentOutput;
    pbr_input.world_normal = normal;
    pbr_input.material.perceptual_roughness = material.specular;
    pbr_input.material.base_color = material.color;
    let buv = vec2<i32>(  mesh.position.xy % 4.0  );
    let ba = bayer[buv.x][buv.y];
    out.color = apply_pbr_lighting(pbr_input);
    if depth_diff_back > 0.01 || depth_diff_forward > 0.01{
        out.color = vec4(0.0);
    }     
    // apply lighting
    #endif
    return out;
}