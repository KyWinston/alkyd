#define_import_path utils

#import noise_gen::FBN;
#import bevy_pbr::prepass_utils::{prepass_depth,prepass_normal};
#import bevy_pbr::mesh_view_bindings::{globals,view,View};


@group(2) @binding(5) var<storage, read_write> voro_cache: array<vec4<f32>>;
fn hash2(p: vec2<f32>) -> vec2<f32> {
    // Dave Hoskin's hash as in https://www.shadertoy.com/view/4djSRW
    var p3 = fract(vec3<f32>(p.xyx) * vec3<f32>(.1031, .1030, .0973));
    p3 += dot(p3, p3.yzx + 19.19);
    let o = fract(vec2((p3.x + p3.y) * p3.z, (p3.x + p3.z) * p3.y));
    return o;
}

fn mod289(x: f32) -> f32 {return x - floor(x * (1.0 / 289.0)) * 289.0;}
fn mod289_4(x: vec4f) -> vec4f {return x - floor(x * (1.0 / 289.0)) * 289.0;}
fn perm(x: vec4f) -> vec4f {return mod289_4(((x * 34.0) + 1.0) * x);}


// WTFPL License
fn noise2(n: vec2<f32>) -> f32 {
    let d = vec2f(0., 1.);
    let b = floor(n);
    let f = smoothstep(vec2(0.), vec2(1.), fract(n));
    return mix(mix(rand22(b), rand22(b + d.yx), f.x), mix(rand22(b + d.xy), rand22(b + d.yy), f.x), f.y);
}

fn rand22(n: vec2<f32>) -> f32 { return fract(sin(dot(n, vec2(12.9898, 4.1414))) * 43758.5453); }

fn fade3(t: vec3f) -> vec3f { return t * t * t * (t * (t * 6. - 15.) + 10.); }
fn rand11(n: f32) -> f32 { return fract(sin(n) * 43758.5453123); }


fn apply_hue(col: vec3<f32>, hueAdjust: f32) -> vec3<f32> {
    let k = vec3(0.57735, 0.57735, 0.57735);
    let cosAngle = cos(hueAdjust);
    return col * cosAngle + cross(k, col) * sin(hueAdjust) + k * dot(k, col) * (1.0 - cosAngle);
}

fn raymarch_hit(position: vec4<f32>, center: vec3<f32>, radius: f32, fog_color: vec4<f32>, steps: u32, prec: f32) -> vec4<f32> {
    var ro: vec3<f32> = view.world_position;
    var dst: f32 = 999.0;
    let rd: vec3<f32> = normalize(position - vec4(ro, 1.0)).xyz;
    let tolerance = 1.0 / pow(10.0, f32(steps) / prec);
    var norm: vec3f;
    for (var x = 0; x < i32(steps); x++) {
        let noise_offset = FBN(vec4f(vec3<f32>((ro)), globals.time / 4.0));
        dst = sphere_hit(ro + noise_offset, vec3<f32>(0.0), radius);
        ro += rd * dst;
        norm = get_ray_normal(ro);
        if dst < tolerance {
            let darken_ramp = 1.0 - f32(x) / f32(steps) * 2.0;
            let diffuse_str = max(0.0, dot(normalize(vec3(5.0, 5.0, 0.0)), norm));
            let diffuse = darken_ramp * diffuse_str;
            // let ref_source = normalize(reflect(-view.world_position, norm));
            return vec4<f32>(fog_color.rgb * diffuse, 1.0);
        }
        if dst > 50.0 {
            break;
        }
    }
    return vec4<f32>(10.0 - dst);
}
    

fn sphere_hit(p: vec3<f32>, c: vec3<f32>, r: f32) -> f32 {
    return distance(p, c) - r;
}

fn get_ray_normal(p: vec3f) -> vec3f {
    let d = vec2(0.01, 0.0);
    let gx = map(p, p - d.xyy, p + d.xyy, -p + d.xyy, p + d.xyy);
    let gy = map(p, p - d.yxy, p + d.yxy, -p + d.yxy, p + d.yxy);
    let gz = map(p, p - d.yyx, p + d.yyx, -p + d.yyx, p + d.yyx);
    return normalize(vec3(gx, gy, gz));
}

fn map(value: vec3f, min1: vec3f, max1: vec3f, min2: vec3f, max2: vec3f) -> f32 {
    return length(min2 + (value - min1) * (max2 - min2) / (max1 - min1));
}

fn voronoi(p: vec2<f32>, depth: f32) -> vec3<f32> {
    var md = 10.0;
    var med = 10.0;
    var tcc: vec2<f32>;
    var cc: vec2<f32>;
    let n = floor(p);
    var cell_id = 0.0;

    for (var x = -1; x <= 1; x++) {
        for (var y = -1; y <= 1; y++) {
            let g = n + vec2(f32(x), f32(y));
            let cache = voro_cache[i32(g.x) + (i32(g.y) * 10)];
            let o = g + cache.xy;
            let r = o - p;
            let d = length(r);
            if depth < 0.0015 {
                let dcc = abs(cc - g);
                if !(dcc.x + dcc.y < 0.05) {
                    let tc = (tcc + r) * 0.5;
                    let cd = normalize(r - tcc);
                    let ed = dot(tc, cd);
                    med = min(med, ed);
                }
            }
            if d < md {
                md = d;
                cc = g;
                tcc = r;
                cell_id = cache.z;
                if d < cache.w {
                    break;
                }
            }
        }
    }
    return vec3<f32>(md, cell_id, med);
}
