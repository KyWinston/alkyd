#import bevy_pbr::{
    pbr_fragment::pbr_input_from_standard_material,
}

#import bevy_pbr::mesh_view_bindings::{view,View};

#import utils::{fresnel,apply_hue, hsv2rgb}
#import simplex_3d::{snoise}

#ifdef PREPASS_PIPELINE
#import bevy_pbr::{
    prepass_io::{VertexOutput, FragmentOutput},
    pbr_deferred_functions::deferred_output,
}
#else
#import bevy_pbr::{
    forward_io::{VertexOutput, FragmentOutput},
    pbr_functions::{apply_pbr_lighting, main_pass_post_lighting_processing},
}
#endif

struct OutputMaterial {
    color: vec4<f32>
}

@group(2) @binding(100)
var<uniform> my_extended_material: OutputMaterial;

@fragment
fn fragment(
    in: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> FragmentOutput {
    // generate a PbrInput struct from the StandardMaterial bindings
    var pbr_input = pbr_input_from_standard_material(in, is_front);
    let view_direction = normalize(in.world_position.xyz - view.world_position.xyz);
    pbr_input.N *= 0.2;
    let facing = dot(view_direction, pbr_input.N.rgb);
    let fresnel = fresnel(my_extended_material.ior, view_direction, in.world_normal.rgb);

    let color_noise = apply_hue(snoise(in.position.xyz * 0.03) * pbr_input.material.base_color.rgb, fresnel);
    pbr_input.material.base_color = vec4(mix(vec3(pbr_input.material.base_color.rgb), color_noise, facing), 1.0);
    pbr_input.material.metallic = 1.0;
    #ifdef PREPASS_PIPELINE
    // in deferred mode we can't modify anything after that, as lighting is run in a separate fullscreen shader.
    let out = deferred_output(in, pbr_input);
#else
    var out: FragmentOutput;
    // apply lighting
    out.color = apply_pbr_lighting(pbr_input);

    // apply in-shader post processing (fog, alpha-premultiply, and also tonemapping, debanding if the camera is non-hdr)
    // note this does not include fullscreen postprocessing effects like bloom.
    out.color = main_pass_post_lighting_processing(pbr_input, out.color);

    // we can optionally modify the final result here
    out.color = out.color;
#endif
    return out;
}