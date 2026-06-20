use crate::{Frame, RenderContext, Rgba, Scene};
use serde::{Deserialize, Serialize};
use std::f32::consts::TAU;

pub struct SineWaveScene {
    pub speed: f32,
}

impl SineWaveScene {
    pub fn new(speed: f32) -> Self {
        Self { speed }
    }
}

impl Scene for SineWaveScene {
    fn name(&self) -> &str {
        "sine_wave"
    }

    fn render(&self, frame: &mut Frame, context: &RenderContext) {
        let w = frame.width as f32;
        let h = frame.height as f32;
        let t = context.normalized_time * self.speed * TAU;

        for x in 0..frame.width {
            let fx = x as f32 / w;
            let y_norm = 0.5 + 0.4 * (fx * TAU * 2.0 + t).sin();
            let y_center = (y_norm * h) as i32;

            for y in 0..frame.height {
                let dist = (y as i32 - y_center).unsigned_abs();
                let brightness = if dist == 0 {
                    255u8
                } else if dist <= 2 {
                    (255 - dist as u32 * 80).min(255) as u8
                } else {
                    0
                };
                if brightness > 0 {
                    frame.set_pixel(x, y, Rgba::new(brightness, brightness, brightness, 255));
                }
            }
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct SineWaveParams {
    #[serde(default = "default_speed")]
    pub speed: f32,
}

impl Default for SineWaveParams {
    fn default() -> Self {
        Self { speed: 1.0 }
    }
}

fn default_speed() -> f32 {
    1.0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn render_sine(frame_index: u32) -> Frame {
        let mut frame = Frame::new(64, 64);
        let ctx = RenderContext::new(frame_index, 60, 24.0, 42);
        SineWaveScene::new(1.0).render(&mut frame, &ctx);
        frame
    }

    #[test]
    fn sine_wave_draws_pixels() {
        let frame = render_sine(0);
        let lit = frame.pixels.iter().any(|p| p.r > 0);
        assert!(lit);
    }

    #[test]
    fn sine_wave_animates() {
        let f0 = render_sine(0);
        let f30 = render_sine(30);
        let any_diff = f0.pixels.iter().zip(f30.pixels.iter()).any(|(a, b)| a != b);
        assert!(any_diff);
    }
}
