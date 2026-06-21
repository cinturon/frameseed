// SDF Shapes scene — smooth-min blended circle + box, animated orbit.
// Params: p0=circle_radius, p1=box_half_width, p2=box_half_height, p3=speed

fn sdf_circle(px: f32, py: f32, cx: f32, cy: f32, r: f32) -> f32 {
    let dx = px - cx;
    let dy = py - cy;
    return sqrt(dx * dx + dy * dy) - r;
}

// Matches the CPU version exactly (no interior negative distance).
fn sdf_rect(px: f32, py: f32, cx: f32, cy: f32, hx: f32, hy: f32) -> f32 {
    let dx = abs(px - cx) - hx;
    let dy = abs(py - cy) - hy;
    let ox = max(dx, 0.0);
    let oy = max(dy, 0.0);
    return max(max(dx, dy), 0.0) + sqrt(ox * ox + oy * oy);
}

fn smin(a: f32, b: f32, k: f32) -> f32 {
    let h = clamp(0.5 + 0.5 * (b - a) / k, 0.0, 1.0);
    return a * h + b * (1.0 - h) - k * h * (1.0 - h);
}

@compute @workgroup_size(8, 8, 1)
fn main(@builtin(global_invocation_id) id: vec3<u32>) {
    if id.x >= frame.width || id.y >= frame.height { return; }

    let circle_r = frame.p0;
    let box_hw   = frame.p1;
    let box_hh   = frame.p2;
    let speed    = frame.p3;
    let time     = frame.norm_time * speed;

    let cx = f32(frame.width)  * 0.5;
    let cy = f32(frame.height) * 0.5;

    let orbit_x  = cx + sin(time * TAU) * 30.0;
    let circle_r2 = circle_r + 8.0 * sin(time * TAU * 2.0);

    let px = f32(id.x) + 0.5;
    let py = f32(id.y) + 0.5;

    let d_circle = sdf_circle(px, py, orbit_x, cy, circle_r2);
    let d_box    = sdf_rect  (px, py, orbit_x, cy, box_hw, box_hh);
    let d        = smin(d_circle, d_box, 20.0);

    let edge = 1.5;
    let v    = 1.0 - clamp(d / edge, 0.0, 1.0);

    output[id.y * frame.width + id.x] = pack(v, v, v);
}
