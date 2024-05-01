#import bevy_pbr::forward_io::{VertexOutput,FragmentOutput}; 
#import bevy_pbr::pbr_types::{STANDARD_MATERIAL_FLAGS_DOUBLE_SIDED_BIT, PbrInput, pbr_input_new},
#import bevy_pbr::pbr_functions as fns;
#import bevy_pbr::prepass_utils::prepass_normal;
#import bevy_pbr::mesh_view_bindings::view;


struct Painterly {
    view_normals: u32,
    diffuse_color: vec4<f32>,
    roughness: f32,
    metallic: f32,
    brush_distortion: f32,
    brush_blur: f32,
    brush_angle: f32,
    brush_texture_influence: f32,
    color_varience: f32,   
    noise_scale: f32,
}

@group(2) @binding(0) var<uniform> material:Painterly;
@group(2) @binding(1) var brush_handle: texture_2d<f32>;
@group(2) @binding(2) var nearest_sampler: sampler;
@group(2) @binding(3) var brush_handle_normal: texture_2d<f32>;
@group(2) @binding(4) var normal_sampler: sampler;

@fragment
fn fragment(
    in: VertexOutput,
    @builtin(front_facing) is_front: bool
) -> @location(0) vec4<f32> {
    var pbr_input: PbrInput = pbr_input_new();
    let grunge_tex = textureSample(brush_handle, nearest_sampler, in.uv * material.noise_scale * 0.5);
    let grunge_tex_normal = textureSample(brush_handle_normal, normal_sampler, in.uv * material.noise_scale);
    let double_sided = (pbr_input.material.flags & STANDARD_MATERIAL_FLAGS_DOUBLE_SIDED_BIT) != 0u;
    pbr_input.material.perceptual_roughness = smooth_knob(material.roughness);
    pbr_input.material.metallic = smooth_knob(material.metallic);
    pbr_input.frag_coord = in.position;
    pbr_input.world_position = in.world_position;
    let varience = snoise2(in.uv).x;
    let voronoi_base = voronoise(mix(in.uv * material.noise_scale, mix(vec3(snoise2(in.uv).x * material.brush_distortion), grunge_tex.rgb, smooth_knob(material.brush_texture_influence)).xy, 0.5), smooth_knob(material.brush_angle), smooth_knob(material.brush_blur));
    pbr_input.world_normal = fns::prepare_world_normal(
        apply_hue(in.world_normal, voronoi_base),
        double_sided,
        is_front
    );
    pbr_input.N = fns::apply_normal_mapping(
        pbr_input.material.flags,
        pbr_input.world_normal,
        double_sided,
        is_front,
        #ifdef VERTEX_TANGENTS
        #ifdef STANDARD_MATERIAL_NORMAL_MAP
        in.world_tangent,
        #endif
        #endif
        in.uv,
        view.mip_bias,
    );
    pbr_input.material.base_color = vec4(apply_hue(material.diffuse_color.rgb, voronoi_base * material.color_varience), 1.0);
    pbr_input.V = fns::calculate_view(in.world_position, pbr_input.is_orthographic);
    if material.view_normals == 1 {
        return vec4(pbr_input.N, 1.0);
        // return vec4(vec3(voronoi_base), 1.0);
    } else {
        return fns::apply_pbr_lighting(pbr_input);
    }
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

fn mod289(x: vec2<f32>) -> vec2<f32> { 
    return x - floor(x * (1.0 / 289.0)) * 289.0;
}

fn mod289_3(x: vec3<f32>) -> vec3<f32> { 
    return x - floor(x * (1.0 / 289.0)) * 289.0;
}

fn permute(v: vec3<f32>) -> vec3<f32> { 
    return mod289_3(((v * 34.0) + 1.0) * v);
}

fn apply_hue(col: vec3<f32>, hueAdjust:f32) -> vec3<f32>{
    let k = vec3(0.57735, 0.57735, 0.57735);
    let cosAngle = cos(hueAdjust);
    return col * cosAngle + cross(k, col) * sin(hueAdjust) + k * dot(k, col) * (1.0 - cosAngle);
}

fn snoise(v:vec2<f32>) -> f32 {
    let C = vec4<f32>(0.211324865405187,  0.366025403784439, -0.577350269189626, 0.024390243902439); 
    var i:vec2<f32>  = floor(v + dot(v, C.yy) );
    let x0:vec2<f32> = v - i + dot(i, C.xx);

    var i1:vec2<f32>;
   
    i1.x = step(x0.y,x0.x);
    i1.y = 1.0 - i1.x;
  
    var x12 = vec4<f32>(x0.xyxy + C.xxzz);
    x12 = vec4<f32>(x12.xy - vec2<f32>(i1),vec2<f32>(x12.zw));

    i = mod289(i); 
    let p:vec3<f32> = permute( permute( i.y + vec3(0.0, i1.y, 1.0 ))
    + i.x + vec3(0.0, i1.x, 1.0 ));

    let x0_dot = dot(x0,x0);
    let x12_dot_a = dot(x12.xy,x12.xy);
    let x12_dot_b = dot(x12.zw,x12.zw);

    var m = max(0.5 - vec3<f32>(x0_dot, x12_dot_a, x12_dot_b), vec3<f32>(0.0));
    m = m*m;
    m = m*m;

    let x = 2.0 * fract(p * C.www) - 1.0;
    let h = abs(x) - 0.5;
    let ox = floor(x + 0.5);
    let a0 = x - ox;

   
    m *= 1.79284291400159 - 0.85373472095314 * ( a0*a0 + h*h );

    var g:vec3<f32>;
    g.x  = a0.x  * x0.x  + h.x  * x0.y;
    g = vec3<f32>(g.x,vec2<f32>(a0.yz * x12.xz + h.yz * x12.yw));
    return 130.0 * dot(m, g);
}

fn snoise2(  x:vec2<f32> ) -> vec2<f32>{
    let s = snoise(vec2( x ));
    let s1 = snoise(vec2( x.y - 19.1, x.x + 47.2 ));
    return vec2( s , s1 );
}

fn smooth_knob(x:f32) -> f32 {
    return clamp(x / 50.0, 0.0, 1.0);
}