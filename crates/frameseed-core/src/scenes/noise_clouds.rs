use crate::{Frame, RenderContext, Rgba, Scene, value_noise_2d};
use serde::{Deserialize, Serialize};

pub struct NoiseCloudsScene {
    pub speed: f32,
    pub scale: f32,
    pub colored: bool,
}

impl NoiseCloudsScene {
    pub fn new(speed: f32, scale: f32, colored: bool) -> Self {
        Self { speed, scale, colored }
    }
}

fn noise_to_color(noise: f32, colored: bool) -> Rgba {
    if colored {
        // Map noise to hue; use noise from a second offset sample for value
        let hue = noise;
        let (r, g, b) = hsv_to_rgb(hue, 0.8, 0.9);
        Rgba::new(r, g, b, 255)
    } else {
        let v = (noise * 255.0) as u8;
        Rgba::new(v, v, v, 255)
    }
}

fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (u8, u8, u8) {
    let h6 = h * 6.0;
    let i = h6.floor() as u32 % 6;
    let f = h6 - h6.floor();
    let p = v * (1.0 - s);
    let q = v * (1.0 - f * s);
    let t = v * (1.0 - (1.0 - f) * s);
    let (r, g, b) = match i {
        0 => (v, t, p),
        1 => (q, v, p),
        2 => (p, v, t),
        3 => (p, q, v),
        4 => (t, p, v),
        _ => (v, p, q),
    };
    ((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8)
}

impl NoiseCloudsParams {
    pub fn new(speed: f32, scale: f32) -> Self {
        Self { speed, scale, colored: false }
    }
}

impl Scene for NoiseCloudsScene {
    fn name(&self) -> &str {
        "noise_clouds"
    }

    fn render(&self, frame: &mut Frame, context: &RenderContext) {
        let colored = self.colored;
        frame.parallel_for_each_pixel(move |x, y| {
            let drift = context.normalized_time * self.speed;
            let noise = value_noise_2d(
                x as f32 * self.scale + drift,
                y as f32 * self.scale,
                context.seed,
            );
            noise_to_color(noise, colored)
        });
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct NoiseCloudsParams {
    #[serde(default = "default_speed")]
    pub speed: f32,
    #[serde(default = "default_scale")]
    pub scale: f32,
    #[serde(default)]
    pub colored: bool,
}

impl Default for NoiseCloudsParams {
    fn default() -> Self {
        Self { speed: 1.0, scale: 0.05, colored: false }
    }
}

fn default_speed() -> f32 {
    1.0
}

fn default_scale() -> f32 {
    0.05
}

#[cfg(test)]
mod tests {
    use super::*;

    fn render_noise_clouds_frame(width: u32, height: u32, frame_index: u32, total_frames: u32, speed: f32, scale: f32) -> Frame {
        let mut frame = Frame::new(width, height);
        let context = RenderContext::new(frame_index, total_frames, 24.0, 42);
        let scene = NoiseCloudsScene::new(speed, scale, false);
        scene.render(&mut frame, &context);
        frame
    }

    #[test]
    fn test_noise_clouds_scene_render_frame0_differs_from_frame60() {
        let frame0 = render_noise_clouds_frame(16, 16, 0, 120, 1.0, 0.05);
        let frame60 = render_noise_clouds_frame(16, 16, 60, 120, 1.0, 0.05);
        assert_ne!(frame0.get_pixel(0, 0), frame60.get_pixel(0, 0));
    }

    #[test]
    fn test_noise_clouds_scene_render_frame0_and_from0_are_the_same() {
        let frame0 = render_noise_clouds_frame(16, 16, 0, 120, 1.0, 0.05);
        let frame60 = render_noise_clouds_frame(16, 16, 0, 120, 1.0, 0.05);
        assert_eq!(frame0.get_pixel(0, 0), frame60.get_pixel(0, 0));
    }
}