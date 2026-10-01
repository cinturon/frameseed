use crate::{Frame, RenderContext, Rgba, Scene};
use serde::{Deserialize, Serialize};
use std::f32::consts::TAU;

pub struct OscilloscopeScene {
    pub frequency: f32,
    pub amplitude: f32,
    pub speed: f32,
    pub glow: f32,
}

impl OscilloscopeScene {
    pub fn new(frequency: f32, amplitude: f32, speed: f32, glow: f32) -> Self {
        Self {
            frequency: frequency.max(0.1),
            amplitude: amplitude.clamp(0.05, 0.95),
            speed,
            glow: glow.max(0.1),
        }
    }
}

impl Scene for OscilloscopeScene {
    fn name(&self) -> &str {
        "oscilloscope"
    }

    fn render(&self, frame: &mut Frame, context: &RenderContext) {
        let w = frame.width as f32;
        let h = frame.height as f32;
        let t = context.normalized_time * self.speed * TAU;
        let frequency = self.frequency;
        let amplitude = self.amplitude;
        let glow = self.glow;
        let grid_x_step = (frame.width / 12).max(1);
        let grid_y_step = (frame.height / 8).max(1);

        frame.parallel_for_each_pixel(move |x, y| {
            let fx = x as f32 / w;
            let fy = y as f32 / h;
            let centered_y = (fy - 0.5) * 2.0;

            let wave = ((fx * frequency * TAU + t).sin() * 0.58
                + (fx * frequency * 0.5 * TAU - t * 1.33).sin() * 0.28
                + (fx * frequency * 1.73 * TAU + t * 0.43).cos() * 0.14)
                * amplitude;

            let dist = (centered_y - wave).abs();
            let beam = (1.0 - dist * 18.0 / glow).clamp(0.0, 1.0);
            let halo = (1.0 - dist * 4.5 / glow).clamp(0.0, 1.0);
            let center_line = (1.0 - centered_y.abs() * 70.0).clamp(0.0, 0.25);
            let grid_x = if x % grid_x_step == 0 { 0.18 } else { 0.0 };
            let grid_y = if y % grid_y_step == 0 { 0.14 } else { 0.0 };
            let vignette = (1.0 - ((fx - 0.5).powi(2) + (fy - 0.5).powi(2)) * 1.35).clamp(0.0, 1.0);

            let green =
                ((beam * 245.0 + halo * 90.0 + center_line * 255.0 + grid_x * 75.0 + grid_y * 65.0)
                    * vignette)
                    .clamp(0.0, 255.0) as u8;
            let red =
                ((beam * 60.0 + halo * 18.0 + grid_y * 20.0) * vignette).clamp(0.0, 255.0) as u8;
            let blue =
                ((beam * 115.0 + halo * 35.0 + grid_x * 35.0) * vignette).clamp(0.0, 255.0) as u8;

            Rgba::new(red, green, blue, 255)
        });
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct OscilloscopeParams {
    #[serde(default = "default_frequency")]
    pub frequency: f32,
    #[serde(default = "default_amplitude")]
    pub amplitude: f32,
    #[serde(default = "default_speed")]
    pub speed: f32,
    #[serde(default = "default_glow")]
    pub glow: f32,
}

impl Default for OscilloscopeParams {
    fn default() -> Self {
        Self {
            frequency: default_frequency(),
            amplitude: default_amplitude(),
            speed: default_speed(),
            glow: default_glow(),
        }
    }
}

fn default_frequency() -> f32 {
    4.0
}

fn default_amplitude() -> f32 {
    0.65
}

fn default_speed() -> f32 {
    1.0
}

fn default_glow() -> f32 {
    1.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn oscilloscope_draws_beam() {
        let mut frame = Frame::new(96, 54);
        OscilloscopeScene::new(4.0, 0.65, 1.0, 1.0)
            .render(&mut frame, &RenderContext::new(0, 48, 24.0, 42));
        assert!(frame.pixels.iter().any(|p| p.g > p.r && p.g > p.b));
    }
}
