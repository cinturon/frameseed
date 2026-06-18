use crate::{Frame, RenderContext, Rgba, Scene, value_noise_2d};
use serde::Deserialize;


pub struct NoiseCloudsScene {
    pub speed: f32,
    pub scale: f32,
}

impl NoiseCloudsScene {
    pub fn new(speed: f32, scale: f32) -> Self {
        Self { speed, scale }
    }
}

impl NoiseCloudsParams{
    pub fn new(speed: f32, scale: f32) -> Self {
        Self { speed, scale }
    }
}

impl Scene for NoiseCloudsScene {
    fn name(&self) -> &str {
        "noise_clouds"
    }

    fn render(&self, frame: &mut Frame, context: &RenderContext) {
        for y in 0..frame.height {
            for x in 0..frame.width {
                let drift = context.normalized_time * self.speed;
                let noise = value_noise_2d(
                    x as f32 * self.scale + drift,
                    y as f32 * self.scale,
                    context.seed,
                );
                let lightness = (noise * 255.0) as u8;
                frame.set_pixel(x, y, Rgba::new(lightness, lightness, lightness, 255));
            }
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct NoiseCloudsParams {
    #[serde(default = "default_speed")]
    pub speed: f32,
    #[serde(default = "default_scale")]
    pub scale: f32,
}

impl Default for NoiseCloudsParams {
    fn default() -> Self {
        Self { speed: 1.0, scale: 0.05 }
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
        let scene = NoiseCloudsScene::new(speed, scale);
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