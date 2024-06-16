#define_import_path utils


@group(2) @binding(5) var<storage, read_write> voro_cache: array<vec4<f32>>;

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

fn fade3(t: vec3f) -> vec3f { return t * t * t * (t * (t * 6. - 15.) + 10.); }
fn rand11(n: f32) -> f32 { return fract(sin(n) * 43758.5453123); }

fn mod289(x: vec4f) -> vec4<f32> {
    return x - floor(x * (1.0 / 289.0)) * 289.0;
}
fn perm4(x: vec4f) -> vec4<f32> {
    return mod289(((x * 34.) + 1.) * x);
}

fn noise3(p: vec3f) -> f32 {
    let a = floor(p);
    var d: vec3f = p - a;
    d = d * d * (3. - 2. * d);

    let b = a.xxyy + vec4f(0., 1., 0., 1.);
    let k1 = perm4(b.xyxy);
    let k2 = perm4(k1.xyxy + b.zzww);

    let c = k2 + a.zzzz;
    let k3 = perm4(c);
    let k4 = perm4(c + 1.);

    let o1 = fract(k3 * (1. / 41.));
    let o2 = fract(k4 * (1. / 41.));

    let o3 = o2 * d.z + o1 * (1. - d.z);
    let o4 = o3.yw * d.x + o3.xz * (1. - d.x);

    return o4.y * d.y + o4.x * (1. - d.y);
}

fn apply_hue(col: vec3<f32>, hueAdjust: f32) -> vec3<f32> {
    let k = vec3(0.57735, 0.57735, 0.57735);
    let cosAngle = cos(hueAdjust);
    return col * cosAngle + cross(k, col) * sin(hueAdjust) + k * dot(k, col) * (1.0 - cosAngle);
}

fn raymarch_hit(position: vec3<f32>, center: vec3<f32>, radius: f32, fog_color: vec4<f32>) -> vec4<f32> {
    var new_pos = position;
    let direction = normalize(position - center);
    var dist = 99999.0;

    for (var i = 0; i < 20; i++) {
        let dist = sphere_hit(new_pos, center, radius);
        if dist < - 0.2 {
            return vec4<f32>(vec3<f32>(fog_color.rgb), 1.0);
        }
        new_pos += dist * direction;
    }
    return vec4<f32>(1.0);
}

fn sphere_hit(p: vec3<f32>, center: vec3<f32>, r: f32) -> f32 {
    return distance(p, center) - r;
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