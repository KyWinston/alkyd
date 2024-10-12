#define_import_path noise_gen
#import global_values::NoiseProperties;
#import simplex_4d;
#import bevy_pbr::mesh_view_bindings::globals;


fn random(st: vec3<f32>) -> f32 {
    return fract(sin(dot(st,
        vec3(12.9898, 78.233, 259.2958))) * 43758.5453123);
}

fn FBN(p: vec4f, oct: i32, amp: f32, gain: f32, freq: f32, lac: f32) -> f32 {
    var new_p = p;
    var n_p: NoiseProperties;

    n_p.octaves = oct;
    n_p.amplitude = amp;
    n_p.gain = gain;
    n_p.frequency = freq;
    n_p.lacunarity = lac;

    var value = 0.0;
    for (var i = 0; i < oct; i++) {
        value += n_p.amplitude * simplex_4d::snoise(n_p.frequency * new_p);
        n_p.frequency *= n_p.lacunarity;
        n_p.amplitude *= n_p.gain;
    }

    return value;
}