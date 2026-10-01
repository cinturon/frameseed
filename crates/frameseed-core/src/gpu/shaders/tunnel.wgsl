// Tunnel scene — infinite-zoom tunnel with depth stripes and angle-based hue.
// Params: p0=speed, p1=rings (float, cast to int)

@compute @workgroup_size(8, 8, 1)
fn main(@builtin(global_invocation_id) id: vec3<u32>) {
    if id.x >= frame.width || id.y >= frame.height { return; }

    let cx = f32(frame.width)  * 0.5;
    let cy = f32(frame.height) * 0.5;
    let dx = f32(id.x) - cx;
    let dy = f32(id.y) - cy;
    let dist = sqrt(dx * dx + dy * dy);

    var c: vec3<f32>;
    if dist < 0.001 {
        c = vec3(0.0);
    } else {
        let speed = frame.p0;
        let rings = frame.p1;
        let t = frame.norm_time * speed;

        let angle = atan2(dy, dx);
        let depth = 64.0 / dist;
        let depth_phase = (depth + t) % 1.0;
        let stripe = fract(depth_phase * rings);
        let brightness = select(80.0 / 255.0, 200.0 / 255.0, stripe < 0.5);

        let hue = fract(angle / TAU + 0.5 + t * 0.1);
        c = hsv_to_rgb(hue, 0.7, brightness);
    }

    output[id.y * frame.width + id.x] = pack(c.x, c.y, c.z);
}
