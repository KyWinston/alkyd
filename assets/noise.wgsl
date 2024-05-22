@group(0) @binding(0)
var<storage, read_write> texture: array<f32>;

@compute @workgroup_size(1)
fn main(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
    let location = vec2<i32>(i32(invocation_id.x), i32(invocation_id.y));
    let uv = vec2<f32>(f32(location.x) / 2048., f32(location.y) / 2048.);
    let noise = snoise2(uv).x;
    texture[location.x * location.y] = noise;
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

fn snoise(v: vec2<f32>) -> f32 {
    let C = vec4<f32>(0.211324865405187, 0.366025403784439, -0.577350269189626, 0.024390243902439);
    var i: vec2<f32> = floor(v + dot(v, C.yy));
    let x0: vec2<f32> = v - i + dot(i, C.xx);

    var i1: vec2<f32>;

    i1.x = step(x0.y, x0.x);
    i1.y = 1.0 - i1.x;

    var x12 = vec4<f32>(x0.xyxy + C.xxzz);
    x12 = vec4<f32>(x12.xy - vec2<f32>(i1), vec2<f32>(x12.zw));

    i = mod289(i);
    let p: vec3<f32> = permute(permute(i.y + vec3(0.0, i1.y, 1.0)) + i.x + vec3(0.0, i1.x, 1.0));

    let x0_dot = dot(x0, x0);
    let x12_dot_a = dot(x12.xy, x12.xy);
    let x12_dot_b = dot(x12.zw, x12.zw);

    var m = max(0.5 - vec3<f32>(x0_dot, x12_dot_a, x12_dot_b), vec3<f32>(0.0));
    m *= m * m;

    let x = 2.0 * fract(p * C.www) - 1.0;
    let h = abs(x) - 0.5;
    let ox = floor(x + 0.5);
    let a0 = x - ox;

    m *= 1.79284291400159 - 0.85373472095314 * (a0 * a0 + h * h);

    var g: vec3<f32>;
    g.x = a0.x * x0.x + h.x * x0.y;
    g = vec3<f32>(g.x, vec2<f32>(a0.yz * x12.xz + h.yz * x12.yw));
    return 130.0 * dot(m, g);
}

fn snoise2(x: vec2<f32>) -> vec2<f32> {
    let s = snoise(vec2(x));
    let s1 = snoise(vec2(x.y - 19.1, x.x + 47.2));
    return vec2(s, s1);
}
