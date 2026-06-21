// Shared struct and helpers included at the top of every frameseed compute shader.

struct FrameParams {
    width:        u32,
    height:       u32,
    frame_index:  u32,
    total_frames: u32,
    norm_time:    f32,   // frame_index / (total_frames - 1), or 0 for single frame
    seed:         u32,   // lower 32 bits of RenderContext::seed
    _pad0:        u32,
    _pad1:        u32,
    p0: f32,
    p1: f32,
    p2: f32,
    p3: f32,
    p4: f32,
    p5: f32,
    p6: f32,
    p7: f32,
}

@group(0) @binding(0) var<uniform> frame: FrameParams;
@group(0) @binding(1) var<storage, read_write> output: array<u32>;

const TAU: f32 = 6.283185307179586;

// Pack normalised [0,1] r,g,b into a little-endian RGBA u32.
// Memory layout: [r8, g8, b8, 255] which ffmpeg and Frame::as_raw_rgba expect.
fn pack(r: f32, g: f32, b: f32) -> u32 {
    let ri = u32(clamp(r, 0.0, 1.0) * 255.0);
    let gi = u32(clamp(g, 0.0, 1.0) * 255.0);
    let bi = u32(clamp(b, 0.0, 1.0) * 255.0);
    return ri | (gi << 8u) | (bi << 16u) | (255u << 24u);
}

fn hsv_to_rgb(h: f32, s: f32, v: f32) -> vec3<f32> {
    let h6 = h * 6.0;
    let i  = i32(floor(h6)) % 6;
    let f  = h6 - floor(h6);
    let p  = v * (1.0 - s);
    let q  = v * (1.0 - f * s);
    let t  = v * (1.0 - (1.0 - f) * s);
    switch i {
        case 0:  { return vec3(v, t, p); }
        case 1:  { return vec3(q, v, p); }
        case 2:  { return vec3(p, v, t); }
        case 3:  { return vec3(p, q, v); }
        case 4:  { return vec3(t, p, v); }
        default: { return vec3(v, p, q); }
    }
}
