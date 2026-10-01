// Gradient scene — animated lerp between two colours in one of four directions.
// Params: p0=speed, p1=direction (0=H,1=V,2=radial,3=diagonal),
//         p2-4=start RGB (0-1), p5-7=end RGB (0-1)

@compute @workgroup_size(8, 8, 1)
fn main(@builtin(global_invocation_id) id: vec3<u32>) {
    if id.x >= frame.width || id.y >= frame.height { return; }

    let speed  = frame.p0;
    let dir    = u32(frame.p1 + 0.5);
    let start  = vec3<f32>(frame.p2, frame.p3, frame.p4);
    let end_c  = vec3<f32>(frame.p5, frame.p6, frame.p7);
    let offset = fract(frame.norm_time * speed);

    // Normalised pixel coords.  Match CPU divisions exactly.
    let w1 = f32(frame.width  - 1u);
    let h1 = f32(frame.height - 1u);
    let fx = f32(id.x) / w1;
    let fy = f32(id.y) / h1;
    let fw = f32(frame.width);
    let fh = f32(frame.height);

    var t: f32;
    switch dir {
        // Horizontal (matches fill_sliding_horizontal_gradient)
        case 0u: { t = fract(fx + offset); }
        // Vertical
        case 1u: { t = fract(fy + offset); }
        // Radial
        case 2u: {
            let max_r = sqrt(0.5 * 0.5 + 0.5 * 0.5);
            let cx = fx - 0.5;
            let cy = fy - 0.5;
            t = fract(sqrt(cx * cx + cy * cy) / max_r + offset);
        }
        // Diagonal (matches CPU: x/w * 0.5 + y/h * 0.5 + offset)
        default: { t = fract(f32(id.x) / fw * 0.5 + f32(id.y) / fh * 0.5 + offset); }
    }

    let c = start + (end_c - start) * t;
    output[id.y * frame.width + id.x] = pack(c.x, c.y, c.z);
}
