use crate::{Frame, RenderContext, Rgba, Scene, seeded_rng};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::f32::consts::TAU;

pub struct MetaballsScene {
    pub count: u32,
    pub speed: f32,
    pub threshold: f32,
}

impl MetaballsScene {
    pub fn new(count: u32, speed: f32, threshold: f32) -> Self {
        Self {
            count: count.clamp(1, 32),
            speed,
            threshold: threshold.max(0.05),
        }
    }
}

impl Scene for MetaballsScene {
    fn name(&self) -> &str {
        "metaballs"
    }

    fn render(&self, frame: &mut Frame, context: &RenderContext) {
        let w = frame.width as f32;
        let h = frame.height as f32;
        let aspect = w / h.max(1.0);
        let t = context.normalized_time * self.speed * TAU;

        let balls: Vec<(f32, f32, f32)> = (0..self.count)
            .map(|i| {
                let mut rng = seeded_rng(context.seed.wrapping_add(9_001 + i as u64 * 97));
                let phase_a = rng.random::<f32>() * TAU;
                let phase_b = rng.random::<f32>() * TAU;
                let orbit = 0.18 + rng.random::<f32>() * 0.50;
                let radius = 0.055 + rng.random::<f32>() * 0.080;
                let wobble = 0.6 + rng.random::<f32>() * 1.4;
                let x = (phase_a + t * wobble).cos() * orbit * aspect;
                let y = (phase_b + t * (1.7 - wobble * 0.35)).sin() * orbit;
                (x, y, radius)
            })
            .collect();

        let threshold = self.threshold;
        frame.parallel_for_each_pixel(move |x, y| {
            let px = (x as f32 / w - 0.5) * 2.0 * aspect;
            let py = (y as f32 / h - 0.5) * 2.0;

            let mut field = 0.0;
            for (bx, by, radius) in &balls {
                let dx = px - bx;
                let dy = py - by;
                field += radius * radius / (dx * dx + dy * dy + 0.0008);
            }

            let edge = ((field - threshold) * 5.0).clamp(0.0, 1.0);
            let core = ((field - threshold * 1.7) * 2.0).clamp(0.0, 1.0);
            let glow = ((field - threshold * 0.30) / threshold * 0.45).clamp(0.0, 1.0);
            let hue = field * 0.9 + t * 0.16;

            let r = (glow * 18.0 + edge * 35.0 + core * (hue.sin() * 85.0 + 170.0))
                .clamp(0.0, 255.0) as u8;
            let g = (glow * 34.0 + edge * 95.0 + core * ((hue + TAU / 3.0).sin() * 65.0 + 155.0))
                .clamp(0.0, 255.0) as u8;
            let b = (glow * 70.0
                + edge * 210.0
                + core * ((hue + 2.0 * TAU / 3.0).sin() * 55.0 + 150.0))
                .clamp(0.0, 255.0) as u8;

            Rgba::new(r, g, b, 255)
        });
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct MetaballsParams {
    #[serde(default = "default_count")]
    pub count: u32,
    #[serde(default = "default_speed")]
    pub speed: f32,
    #[serde(default = "default_threshold")]
    pub threshold: f32,
}

impl Default for MetaballsParams {
    fn default() -> Self {
        Self {
            count: default_count(),
            speed: default_speed(),
            threshold: default_threshold(),
        }
    }
}

fn default_count() -> u32 {
    7
}

fn default_speed() -> f32 {
    0.8
}

fn default_threshold() -> f32 {
    0.75
}

#[cfg(test)]
mod tests {
    use super::*;

    fn render(frame_index: u32) -> Frame {
        let mut frame = Frame::new(64, 64);
        MetaballsScene::new(7, 0.8, 0.75)
            .render(&mut frame, &RenderContext::new(frame_index, 48, 24.0, 42));
        frame
    }

    #[test]
    fn metaballs_draws_glow() {
        let frame = render(0);
        assert!(frame.pixels.iter().any(|p| p.b > 0));
    }

    #[test]
    fn metaballs_is_deterministic() {
        assert_eq!(render(0).pixels, render(0).pixels);
    }
}
