#define_import_path utils

#import bevy_pbr::pbr_functions as fns;
#import noise_gen::FBN;
#import bevy_pbr::forward_io::{VertexOutput,FragmentOutput}; 
#import bevy_pbr::pbr_types::{STANDARD_MATERIAL_FLAGS_DOUBLE_SIDED_BIT, PbrInput, pbr_input_new};
#import bevy_pbr::prepass_utils::{prepass_depth,prepass_normal};
#import bevy_pbr::mesh_view_bindings::{globals,view,View};


@group(2) @binding(1) var<storage, read_write> voro_cache: array<vec4<f32>>;

fn hash2(p: vec2<f32>) -> vec2<f32> {
    var p3 = fract(vec3<f32>(p.xyx) * vec3<f32>(.1031, .1030, .0973));
    p3 += dot(p3, p3.yzx + 19.19);
    let o = fract(vec2((p3.x + p3.y) * p3.z, (p3.x + p3.z) * p3.y));
    return o;
}

fn mod289(x: f32) -> f32 {return x - floor(x * (1.0 / 289.0)) * 289.0;}
fn mod289_4(x: vec4f) -> vec4f {return x - floor(x * (1.0 / 289.0)) * 289.0;}
fn perm(x: vec4f) -> vec4f {return mod289_4(((x * 34.0) + 1.0) * x);}

fn fresnel(ior: f32, I: vec3<f32>, N: vec3<f32>) -> f32 {
    let r0 = pow((1.0 - ior) / (1.0 + ior), 2.0);
    return smoothstep(-1.0, 1.0, r0 + (1.0 - r0) * pow(1.0 - dot(N, I), 5.0));
}

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

/// Signed distance field for a Bezier curve.
fn sd_bezier(p: vec2f, A: vec2f, B: vec2f, C: vec2f) -> vec2f {
    let a = B - A;
    let b = A - 2. * B + C;
    let c = a * 2.;
    let d = A - p;
    let kk = 1. / dot(b, b);
    let kx = kk * dot(a, b);
    let ky = kk * (2. * dot(a, a) + dot(d, b)) / 3.;
    let kz = kk * dot(d, a);

    let p1 = ky - kx * kx;
    let p3 = p1 * p1 * p1;
    let q = kx * (2.0 * kx * kx - 3.0 * ky) + kz;
    var h: f32 = q * q + 4. * p3;

    var res: vec2f;
    if h >= 0. {
        h = sqrt(h);
        let x = (vec2f(h, -h) - q) / 2.;
        let uv = sign(x) * pow(abs(x), vec2f(1. / 3.));
        let t = clamp(uv.x + uv.y - kx, 0., 1.);
        let f = d + (c + b * t) * t;
        res = vec2f(dot(f, f), t);
    } else {
        let z = sqrt(-p1);
        let v = acos(q / (p1 * z * 2.)) / 3.;
        let m = cos(v);
        let n = sin(v) * 1.732050808;
        let t = clamp(vec2f(m + m, -n - m) * z - kx, vec2f(0.0), vec2f(1.0));
        let f = d + (c + b * t.x) * t.x;
        var dis: f32 = dot(f, f);
        res = vec2f(dis, t.x);

        let g = d + (c + b * t.y) * t.y;
        dis = dot(g, g);
        res = select(res, vec2f(dis, t.y), dis < res.x);
    }
    res.x = sqrt(res.x);
    return res;
}

fn raymarch_clip(position: vec4<f32>, center: vec3<f32>, radius: f32, color: vec4<f32>) -> vec4<f32> {
    var ro: vec3<f32> = view.world_position;
    var dst: f32 = 999.0;
    let rd: vec3<f32> = normalize(position - vec4(ro, 1.0)).xyz;
    for (var x = 0; x < 50; x++) {
        dst = sphere_hit(ro, center, radius);
        ro += rd * dst;
        if dst < 0.05 {
            return vec4(1.0);
        }
        if dst > 50.0 {
            break;
        }
    }
    return vec4<f32>(0.0);
}

fn cut_sphere_hit(p: vec3<f32>, c: vec3<f32>, r: f32) -> f32 {
    return max(-distance(p, c - vec3(0.2)) - (r + 0.2), distance(p, c) - r);
}


//given an origin and signed distance function, march one step and return a new origin, distance from a surface
fn raymarch(origin: vec3<f32>, direction: vec3<f32>, dst: f32) -> vec4<f32> {
    var ro = origin;
    var rd = direction;

    ro += rd * abs(dst);

    return vec4(vec3f(ro), dst);
}
    

fn sdf_cone(p: vec3f, r1: f32, r2: f32, h: f32) -> f32 {
    let b = (r1 - r2) / h;
    let a = sqrt(1.0 - b * b);

    // if r1 == 0.0 {
    //     let q = length(p.xz);
    // } else {
    //     let q = vec2f(length(p.xz), p.y);
    // }
    let q = vec2f(length(p.xz), p.y);

    let k = dot(q, vec2f(-b, a));
    if k < 0.0 {return length(q) - r1;}
    if k > a * h {return length(q - vec2f(0.0, h)) - r2;}
    return dot(q, vec2f(a, b)) - r1;
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

fn voronoi(p: vec2<f32>, depth: f32, dist_fn: u32, exp: f32) -> vec3<f32> {
    var md = 100.0;
    var med = 100.0;
    var tcc: vec2<f32>;
    var cc: vec2<f32>;
    let n = floor(p);
    var cell_id = 0.0;

    for (var x = -1; x <= 1; x++) {
        for (var y = -1; y <= 1; y++) {
            let g = n + vec2(f32(x), f32(y));
            let cache = voro_cache[i32(g.x) + (i32(g.y) * 100)];
            let o = g + cache.xy;
            let r = o - p;
            var d: f32 = length(r);
            if dist_fn == 1u {
                d = abs(o.x - p.x) + abs(o.y - p.y);
            } else if dist_fn == 2u {
                d = max(abs(o.x - p.x), abs(o.y - p.y));
            } else if dist_fn >= 3u {
                d = pow(pow(abs(o.x - p.x), exp) + pow(abs(o.y - p.y), exp), 1.0 / exp);
            }

            if d < md {
                md = d;
                cc = g;
                tcc = r;
                cell_id = cache.z;
                if dist_fn == 0u && d < cache.w {
                    break;
                }
            }
        }
    }
    for (var x2 = -1; x2 <= 1; x2++) {
        for (var y2 = -1; y2 <= 1; y2++) {
            let g = n + vec2(f32(x2), f32(y2));
            let cache = voro_cache[i32(g.x) + (i32(g.y) * 100)];
            let o = g + cache.xy;
            let r = o - p;

            if depth < 0.0015 {
                let dcc = abs(cc - g);
                if !(dcc.x + dcc.y < 0.05) {
                    let tc = (tcc + r) * 0.5;
                    let cd = normalize(r - tcc);
                    let ed = dot(tc, cd);
                    med = min(med, ed);
                }
            }
        }
    }
    return vec3<f32>(md, cell_id, med);
}

fn sincosbundle(val: f32) -> f32 {
    return sin(cos(2. * val) + sin(4. * val) - cos(5. * val) + sin(3. * val)) * 0.05;
}

//random noise function
fn nrand(n: vec2f) -> f32 {
    return fract(sin(dot(n.xy, vec2(12.9898, 78.233))) * 43758.5453);
}
    
// 2-tone patterns
fn half_tone(rotation: f32) -> mat2x2<f32> {
    return rotate2D(rotation);
}    

fn wave_texture(freq: f32, amplitude: f32, gain: f32, sharpness: f32) -> f32 {
    let peak = (1.0 * amplitude + gain) * (1.0 - sharpness);
    return smoothstep(-peak, peak, sin(freq) * amplitude + gain);
}    

fn grid_texture(uv: vec2<f32>, outline: f32) -> f32 {
    if outline > 1.0 {
        return abs(floor(uv.x) + floor(uv.y)) % 2.0;
    }
    return wave_texture(uv.x, 0.7, 0.5, outline) * wave_texture(uv.y, 0.7, 0.5, outline);
} 

fn hash3(p: vec2f) -> vec3f {
    let q = vec3f(dot(p, vec2f(127.1, 311.7)),
        dot(p, vec2f(269.5, 183.3)),
        dot(p, vec2f(419.2, 371.9)));
    return fract(sin(q) * 43758.5453);
}

fn brick_texture(uv: vec2f, gap: f32, ratio: f32, mortar: f32) -> f32 {
    let coord = floor(uv);
    let gv = fract(uv);

    let movingValue = -sincosbundle(coord.y) * gap;

    let offset = floor(uv.y % ratio) * movingValue;
    let verticalEdge = abs(cos(uv.x + offset));

    let vrtEdge = step(1. - mortar, verticalEdge) == 1.;
    let hrtEdge = gv.y > (0.9) || gv.y < (mortar);

    if hrtEdge || vrtEdge {
        return 0.0;
    }
    return 1.0;
}

fn trace(origin: vec3f, r: vec3f) -> f32 {
    var t = 0.0;
    for (var i = 0; i < 64; i++) {
        let p = origin + r * t;
        let d = mmap(p);
        t += d * 0.22;
    }
    return t ;
}

fn mmap(ip: vec3f) -> f32 {
    var p = ip;
    var q = p;
    var qa = p;

    q = pmod3(q, vec3f(0.8, 1.0, 0.23));
    qa = pmod3(qa, vec3f(0.8, 1.0, 0.18));
    p.x = pmod1(p.x, 1.0);

    let s1 = sd_sphere(p, 0.75);
    let s2 = sd_sphere(q, 0.5);
    let s3 = sd_sphere(qa, 0.555);

    return min(min(s1, s2), s3);
}

fn pmod1(in: f32, size: f32) -> f32 {
    let halfsize = size * 0.5;
    return (in + halfsize % size) - halfsize;
}

fn pmod3(in: vec3f, size: vec3f) -> vec3f {
    let out = (in % size * 0.5) - (size * 0.5);

    return out;
}

fn sd_sphere(p: vec3f, radius: f32) -> f32 {
    return (length(p) - radius);
}

// fn fade(col: vec4f, uv: vec2f) {
//     let fade = max(abs(uv.x), abs(uv.y)) - 1.0 ; // This is really cool.
//     let col = col * (fade / (0.005 + fade));
//     return col;
// }

fn hsv2rgb(c: vec3f) -> vec3f {
    var rgb: vec3f = clamp(
        abs((c.x * 6.0 + vec3f(0.0, 4.0, 2.0)) % 6.0 - 3.0) - 1.0,
        vec3f(0.0),
        vec3f(1.0)
    );
    return c.z * mix(vec3f(1.0), rgb, c.y);
}

fn gradient(t: f32) -> vec3f {
    let h: f32 = 0.6666 * (1.0 - t * t);
    let s: f32 = 0.75;
    let v: f32 = 1.0 - 0.9 * (1.0 - t) * (1.0 - t);
    return hsv2rgb(vec3f(h, s, v));
}

/// Clockwise by `theta`
fn rotate2D(theta: f32) -> mat2x2<f32> {
    let c = cos(theta);
    let s = sin(theta);
    return mat2x2<f32>(c, s, -s, c);
}

struct ColorStop {
    color: vec3f,
    position: f32
}

fn color_ramp(color_stops: array<ColorStop,3>, factor: f32) -> vec3<f32> {
    var index = 0;
    var current: ColorStop;
    var next: ColorStop;
    var stops = color_stops;
    for (var i = 0; i < 3; i++) {
        current = stops[i];
        next = stops[i + 1];

        if current.position <= factor && factor <= next.position {
            index = i;
        }
    }
    current = stops[index];
    next = stops[index + 1];

    let range: f32 = next.position - current.position;
    let lerp_factor = (factor - current.position) / range;
    return mix(current.color, next.color, lerp_factor);
}


fn extend_pbr(in: VertexOutput, is_front: bool) -> PbrInput {
    var pbrInput = pbr_input_new();
    pbrInput.frag_coord = in.position;
    pbrInput.world_position = in.world_position;

    let double_sided = (pbrInput.material.flags & STANDARD_MATERIAL_FLAGS_DOUBLE_SIDED_BIT) != 0u;
    pbrInput.world_normal = fns::prepare_world_normal(
        in.world_normal,
        double_sided,
        is_front
    );
    
    #ifdef VERTEX_TANGENTS
    let Nt = pbrInput.world_normal.rgb;
    let TBN = fns::calculate_tbn_mikktspace(pbrInput.world_normal.rgb,
        in.world_tangent);
    pbrInput.N = fns::apply_normal_mapping(
        pbr_input.material.flags,
        TBN,
        double_sided,
        is_front,
        Nt,
    );
    #else
    pbrInput.N = normalize(pbrInput.world_normal);
    #endif
    return pbrInput;
}