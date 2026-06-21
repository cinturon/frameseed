// Mandelbrot scene — animated zoom into a configurable point on the Mandelbrot set.
// Params: p0=max_iter, p1=center_re, p2=center_im, p3=initial_view_width, p4=zoom_speed

fn mandelbrot_iter(c_re: f32, c_im: f32, max_iter: u32) -> u32 {
    var z_re = 0.0;
    var z_im = 0.0;
    for (var i = 0u; i < max_iter; i++) {
        let z_re2 = z_re * z_re;
        let z_im2 = z_im * z_im;
        if z_re2 + z_im2 > 4.0 { return i; }
        let new_re = z_re2 - z_im2 + c_re;
        z_im  = 2.0 * z_re * z_im + c_im;
        z_re  = new_re;
    }
    return max_iter;
}

@compute @workgroup_size(8, 8, 1)
fn main(@builtin(global_invocation_id) id: vec3<u32>) {
    if id.x >= frame.width || id.y >= frame.height { return; }

    let max_iter       = u32(frame.p0 + 0.5);
    let center_re      = frame.p1;
    let center_im      = frame.p2;
    let init_view_w    = frame.p3;
    let zoom_speed     = frame.p4;

    let time       = frame.norm_time * zoom_speed;
    let view_width = init_view_w * pow(0.5, time);
    let scale      = view_width / f32(frame.width);

    let half_w = f32(frame.width)  * 0.5;
    let half_h = f32(frame.height) * 0.5;

    let c_re = center_re + (f32(id.x) - half_w) * scale;
    let c_im = center_im + (f32(id.y) - half_h) * scale;

    let count = mandelbrot_iter(c_re, c_im, max_iter);
    var v: f32 = 0.0;
    if count < max_iter {
        v = f32(count) / f32(max_iter);
    }

    output[id.y * frame.width + id.x] = pack(v, v, v);
}
