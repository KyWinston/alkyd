
@group(0) @binding(0)
var<uniform> cell_size: f32;

@group(0) @binding(1)
var<storage,read_write> centroids_out: array<vec4<f32>>;

@compute @workgroup_size(1)
fn main(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
    let location = vec2<i32>(i32(invocation_id.x), i32(invocation_id.y));
    let uv = vec2<f32>(f32(location.x), f32(location.y));
    // let min_dist = 0.3;
    let cell = cell_size;
    // let current = centroids_out[location.x + location.y * 20 + location.z * 200].xyz;
    // let neighbor = centroids_out[(location.x - 1) + (location.y - 1) * 20 + (location.z - 1) * 200].xyz;
    let seed = vec2(noise2(uv),noise2(uv * uv));
    centroids_out[location.x + location.y * 10] = vec4<f32>(seed, noise2(vec2(f32(location.x), f32(location.y))), cell_size);
}


fn hash2(p: vec2<f32>) -> vec2<f32> {
    // Dave Hoskin's hash as in https://www.shadertoy.com/view/4djSRW
    var p3 = fract(vec3<f32>(p.xyx) * vec3<f32>(.1031, .1030, .0973));
    p3 += dot(p3, p3.yzx + 19.19);
    let o = fract(vec2((p3.x + p3.y) * p3.z, (p3.x + p3.z) * p3.y));
    return o;
}

// WTFPL License
fn noise2(n: vec2<f32>) -> f32 {
    let d = vec2f(0., 1.);
    let b = floor(n);
    let f = smoothstep(vec2(0.), vec2(1.), fract(n));
    return mix(mix(rand22(b), rand22(b + d.yx), f.x), mix(rand22(b + d.xy), rand22(b + d.yy), f.x), f.y);
}

fn rand22(n: vec2<f32>) -> f32 { return fract(sin(dot(n, vec2(12.9898, 4.1414))) * 43758.5453); }

// fn fade3(t: vec3f) -> vec3f { return t * t * t * (t * (t * 6. - 15.) + 10.); }
// fn rand11(n: f32) -> f32 { return fract(sin(n) * 43758.5453123); }

// fn mod289(x: vec4f) -> vec4<f32> {
//     return x - floor(x * (1.0 / 289.0)) * 289.0;
// }
// fn perm4(x: vec4f) -> vec4<f32> {
//     return mod289(((x * 34.) + 1.) * x);
// }

// fn noise3(p: vec3f) -> f32 {
//     let a = floor(p);
//     var d: vec3f = p - a;
//     d = d * d * (3. - 2. * d);

//     let b = a.xxyy + vec4f(0., 1., 0., 1.);
//     let k1 = perm4(b.xyxy);
//     let k2 = perm4(k1.xyxy + b.zzww);

//     let c = k2 + a.zzzz;
//     let k3 = perm4(c);
//     let k4 = perm4(c + 1.);

//     let o1 = fract(k3 * (1. / 41.));
//     let o2 = fract(k4 * (1. / 41.));

//     let o3 = o2 * d.z + o1 * (1. - d.z);
//     let o4 = o3.yw * d.x + o3.xz * (1. - d.x);

//     return o4.y * d.y + o4.x * (1. - d.y);
// }