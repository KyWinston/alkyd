#import bevy_pbr::forward_io::{VertexOutput,FragmentOutput}; 
#import bevy_pbr::pbr_types::{STANDARD_MATERIAL_FLAGS_DOUBLE_SIDED_BIT, PbrInput, pbr_input_new},
#import bevy_pbr::pbr_functions as fns;
#import bevy_pbr::prepass_utils::prepass_normal;
#import bevy_pbr::mesh_view_bindings::view;

struct Painterly {
    diffuse_color: vec4<f32>,
    roughness: f32,
    metallic: f32,
    color_varience: f32,
    scale: f32,
    distort: f32,
    influence: f32,
    angle: f32,
    blur: f32,
}

@group(2) @binding(0) var<uniform> material:Painterly;
@group(2) @binding(1) var brush_handle: texture_2d<f32>;
@group(2) @binding(2) var s: sampler;
@group(2) @binding(3) var brush_handle_normal: texture_2d<f32>;
@group(2) @binding(4) var normal_sampler: sampler;
@group(2) @binding(5) var voro_cache: texture_2d<f32>;
@group(2) @binding(6) var v_sampler: sampler;


@fragment
fn fragment(
    in: VertexOutput,
    @builtin(front_facing) is_front: bool
) -> @location(0) vec4<f32> {
    var pbr_input: PbrInput = pbr_input_new();
    let varience = textureSample(voro_cache, v_sampler, in.uv).x * material.distort;
    let grunge_tex = textureSample(brush_handle, s, in.uv);
    let grunge_tex_normal = textureSample(brush_handle_normal, normal_sampler, in.uv * material.scale);
    let grunge_normal_distort = mix(vec3(varience), grunge_tex.rgb, material.influence).xy;
    let double_sided = (pbr_input.material.flags & STANDARD_MATERIAL_FLAGS_DOUBLE_SIDED_BIT) != 0u;
    pbr_input.material.perceptual_roughness = material.roughness;
    pbr_input.material.metallic = material.metallic;
    pbr_input.frag_coord = in.position;
    pbr_input.world_position = in.world_position;
    let voronoi_base = voronoise(mix(in.uv * material.scale, grunge_normal_distort, 0.5), material.angle, material.blur);
    pbr_input.world_normal = fns::prepare_world_normal(
        apply_hue(in.world_normal, voronoi_base),
        double_sided,
        is_front
    );
    pbr_input.N = mix(fns::apply_normal_mapping(
        pbr_input.material.flags,
        pbr_input.world_normal,
        double_sided,
        is_front,
        in.uv,
        view.mip_bias,
    ), grunge_tex_normal.rgb, 0.5);
    pbr_input.material.base_color = vec4(apply_hue(material.diffuse_color.rgb, voronoi_base * material.color_varience), 1.0);
    pbr_input.V = fns::calculate_view(in.world_position, pbr_input.is_orthographic);
    return fns::apply_pbr_lighting(pbr_input);
}

fn hash_three(p: vec2<f32>) -> vec3<f32> {
    let q: vec3<f32>= vec3(dot(p, vec2(127.1, 311.7)),
            dot(p, vec2(269.5, 183.3)),
            dot(p, vec2(419.2, 371.9)));
    return fract(sin(q) * 43758.5453);
}

fn voronoise(p: vec2<f32>, u: f32, v: f32) -> f32{
    let k: f32= 1.0 + 63.0 * pow(1.0 - v, 6.0);

    let i: vec2<f32>= floor(p);
    let f: vec2<f32>= fract(p);
    
    var a: vec2<f32>= vec2(0.0,0.0);
    for (var y = -2; y<=2; y++) {
    for (var x = -2; x<=2; x++)
    {
       let g: vec2<f32> = vec2<f32>( f32(x), f32(y) );
        let o: vec3<f32> = hash_three( i + g ) * vec3(u,u,1.0);
        let d: vec2<f32> = g - f + o.xy;
        let w: f32 = pow( 1.0 - smoothstep(0.0,1.414,length(d)), k );
        a += vec2(o.z*w,w);
    }
    }
    
    return a.x/a.y;
}

fn apply_hue(col: vec3<f32>, hueAdjust:f32) -> vec3<f32>{
    let k = vec3(0.57735, 0.57735, 0.57735);
    let cosAngle = cos(hueAdjust);
    return col * cosAngle + cross(k, col) * sin(hueAdjust) + k * dot(k, col) * (1.0 - cosAngle);
}
