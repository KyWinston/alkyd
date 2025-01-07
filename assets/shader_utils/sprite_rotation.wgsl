#define_import_path sprite

fn read_spritesheet_rotation(sprite_angle: vec2<f32>, view_angle: vec2<f32>, idx: f32, frame_start: vec2<f32>) -> f32 {
    let dir1 = sprite_angle;
    let dir2 = view_angle;
    let dot = dir1.x * dir2.x + dir1.y * dir2.y;
    let det = dir1.x * dir2.y - dir1.y * dir2.x;

    let angle = degrees(atan2(dot, det));
    var offset: f32 = frame_start.x + (f32(step(45.0, abs(angle)) + step(90.0, abs(angle)) + step(135.0, abs(angle))));
    var anim_idx = idx;
    let backface = abs(angle) >= -180.0 && abs(angle) < -170.0;
    let mirror = angle > 44.0 && angle < 170.0;

    var uv_offset: u32 = 0u;

    if backface {
        anim_idx = 0.0;
    }

    if backface || mirror {
        anim_idx = 1.0 - anim_idx;
    }
    return anim_idx;
}