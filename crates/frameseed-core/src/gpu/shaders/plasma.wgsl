// Plasma scene — layered sine waves producing a psychedelic colour field.
// Params: p0=speed, p1=scale

@compute @workgroup_size(8, 8, 1)
fn main(@builtin(global_invocation_id) id: vec3<u32>) {
    if id.x >= frame.width || id.y >= frame.height { return; }

    let fx = f32(id.x) / f32(frame.width);
    let fy = f32(id.y) / f32(frame.height);
    let speed = frame.p0;
    let scale = frame.p1;
    let t = frame.norm_time * speed * TAU;

    let v1 = sin(fx * scale * TAU + t);
    let v2 = sin(fy * scale * TAU + t);
    let v3 = sin((fx + fy) * scale * 0.5 * TAU + t * 0.7);
    let cx = fx - 0.5 + sin(t * 0.3) * 0.3;
    let cy = fy - 0.5 + cos(t * 0.3) * 0.3;
    let v4 = sin(sqrt(cx * cx + cy * cy) * scale * TAU + t);

    let v    = (v1 + v2 + v3 + v4) * 0.25;
    let norm = (v + 1.0) * 0.5;

    let r = sin(norm * TAU) * 0.5 + 0.5;
    let g = sin(norm * TAU + TAU / 3.0) * 0.5 + 0.5;
    let b = sin(norm * TAU + 2.0 * TAU / 3.0) * 0.5 + 0.5;

    output[id.y * frame.width + id.x] = pack(r, g, b);
}
