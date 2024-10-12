#define_import_path blend_modes

fn color_dodge(color1: vec3f, color2: vec3f) -> vec3f {
    if color2.r == 0.0 && color2.g == 0.0 && color2.b == 0.0 {
        return vec3f(0.0);
    } else if color1.r == 1.0 && color1.g == 1.0 && color1.b == 1.0 {
        return vec3f(1.0);
    } else {
        return min(vec3(1.0), color2 / (vec3(1.0) - color1));
    }
}

fn color_burn(color1: vec3f, color2: vec3f) -> vec3f {
    if color2.r == 1.0 && color2.g == 1.0 && color2.b == 1.0 {
        return vec3f(1.0);
    } else if color1.r == 0.0 && color1.g == 0.0 && color1.b == 0.0 {
        return vec3f(0.0);
    } else {
        return min(vec3(1.0), color2 / (vec3(1.0) - color1));
    }
}

fn screen(color1: vec3f, color2: vec3f) -> vec3f {
    return color2 + color1 - (color2 * color1);
}

fn hard_light(color1: vec3f, color2: vec3f) -> vec3f {
    if (color1.r <= 0.5) && (color1.g <= 0.5) && (color1.b <= 0.5) {
        return color2 * (2.0 * color1);
    } else {
        return screen(color2, 2.0 * color1);
    }
}

fn soft_light(color1: vec3f, color2: vec3f) -> vec3f {
    var d = vec3f(0.0);
    if (color2.r <= 0.25) && (color2.g <= 0.25) && (color2.b <= 0.25) {
        d = ((16.0 * color2 - 12.0) * color2 + 4.0) * color2;
    } else {
        d = sqrt(color2);
    }
    if (color1.r <= 0.5) && (color1.g <= 0.5) && (color1.b <= 0.5) {
        return color2 - (1.0 - 2.0 * color1) * color2 * (1.0 - color2);
    } else {
        return color2 + (2.0 * color1 - 1.0) * (d - color2);
    }
}

fn overlay(color1:vec3f, color2:vec3f)-> vec3f{
    return hard_light(color2, color1);
}