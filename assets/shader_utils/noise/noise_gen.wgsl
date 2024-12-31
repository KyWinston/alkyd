#define_import_path noise_gen
#import global_values::NoiseProperties;
#import simplex_4d;
#import bevy_pbr::mesh_view_bindings::globals;


@group(2) @binding(1) var<uniform> props:NoiseProperties;

fn random(st: vec3<f32>) -> f32 {
    return fract(sin(dot(st,
        vec3(12.9898, 78.233, 259.2958))) * 43758.5453123);
}

fn FBN(p: vec4f) -> f32 {
    var new_p = p;
    var n_p = props;

    var value = 0.0;
    for (var i = 0; i < props.octaves; i++) {
        value += n_p.amplitude * simplex_4d::snoise(n_p.frequency * new_p);
        n_p.frequency *= n_p.lacunarity;
        n_p.amplitude *= n_p.gain;
    }

    return value;
}