// Sine Wave scene — animated white sine wave on a black background.
// Params: p0=speed

@compute @workgroup_size(8, 8, 1)
fn main(@builtin(global_invocation_id) id: vec3<u32>) {
    if id.x >= frame.width || id.y >= frame.height { return; }

    let w = f32(frame.width);
    let h = f32(frame.height);
    let speed = frame.p0;
    let t = frame.norm_time * speed * TAU;

    let fx      = f32(id.x) / w;
    let y_norm  = 0.5 + 0.4 * sin(fx * TAU * 2.0 + t);
    let y_ctr   = i32(y_norm * h);
    let dist    = u32(abs(i32(id.y) - y_ctr));

    var b: f32;
    if dist == 0u {
        b = 1.0;
    } else if dist <= 2u {
        b = clamp(1.0 - f32(dist) * (80.0 / 255.0), 0.0, 1.0);
    } else {
        b = 0.0;
    }

    output[id.y * frame.width + id.x] = pack(b, b, b);
}
