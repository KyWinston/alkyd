#define_import_path noise_gen

#import simplex_4d;
#import bevy_pbr::mesh_view_bindings::globals;


struct NoiseProperties {
    octaves: i32,
    lacunarity: f32,
    gain: f32,
    amplitude: f32,
    frequency: f32
}

@group(2) @binding(1) var<uniform> noise_props: NoiseProperties;

fn random(st: vec3<f32>) -> f32 {
    return fract(sin(dot(st,
        vec3(12.9898, 78.233, 259.2958))) * 43758.5453123);
}

fn FBN(p: vec4f) -> f32 {
    var n_p = noise_props;
    var new_p = p;
    var value = 0.0;
    for (var i = 0; i < noise_props.octaves; i++) {
        value += n_p.amplitude * simplex_4d::snoise(new_p);
        new_p *= vec4f(vec3f(n_p.frequency * n_p.lacunarity), 1.5);
        n_p.amplitude *= n_p.gain;
    }

    return value;
}