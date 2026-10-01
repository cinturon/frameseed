// Noise Clouds scene — value noise producing a drifting cloud texture.
// Params: p0=speed, p1=scale, p2=colored (0=grey, 1=HSV)
//
// The noise algorithm here is a hash-based value noise that produces visually
// equivalent results to the CPU ChaCha-RNG lattice — not bit-identical but the
// same character and seeded deterministically with the same seed.

fn hash2(ix: i32, iy: i32, seed: u32) -> f32 {
    // Bitcast so negative integers become unique u32 values.
    let ux = bitcast<u32>(ix);
    let uy = bitcast<u32>(iy);
    var h = ux ^ (uy * 2246822519u) ^ (seed * 3266489917u);
    h = h ^ (h >> 16u);
    h = h * 0x45d9f3bu;
    h = h ^ (h >> 16u);
    return f32(h) / 4294967295.0;
}

fn value_noise(fx: f32, fy: f32, seed: u32) -> f32 {
    let x0 = i32(floor(fx));
    let y0 = i32(floor(fy));
    let ux = fx - f32(x0);
    let uy = fy - f32(y0);
    // Smoothstep (fade curve matching CPU lerp fade)
    let sx = ux * ux * (3.0 - 2.0 * ux);
    let sy = uy * uy * (3.0 - 2.0 * uy);
    let v00 = hash2(x0,     y0,     seed);
    let v10 = hash2(x0 + 1, y0,     seed);
    let v01 = hash2(x0,     y0 + 1, seed);
    let v11 = hash2(x0 + 1, y0 + 1, seed);
    return mix(mix(v00, v10, sx), mix(v01, v11, sx), sy);
}

@compute @workgroup_size(8, 8, 1)
fn main(@builtin(global_invocation_id) id: vec3<u32>) {
    if id.x >= frame.width || id.y >= frame.height { return; }

    let speed   = frame.p0;
    let scale   = frame.p1;
    let colored = frame.p2 > 0.5;
    let drift   = frame.norm_time * speed;

    let nx = f32(id.x) * scale + drift;
    let ny = f32(id.y) * scale;
    let n  = value_noise(nx, ny, frame.seed);

    var c: vec3<f32>;
    if colored {
        c = hsv_to_rgb(n, 0.8, 0.9);
    } else {
        c = vec3(n, n, n);
    }

    output[id.y * frame.width + id.x] = pack(c.x, c.y, c.z);
}
