use crate::{Frame, RenderContext, Rgba, Scene};
use serde::{Deserialize, Serialize};
use std::f32::consts::TAU;

pub struct LissajousScene {
    pub a: f32,
    pub b: f32,
    pub delta: f32,
    pub speed: f32,
    pub thickness: u32,
    pub trail_frames: u32,
}

impl LissajousScene {
    pub fn new(a: f32, b: f32, delta: f32, speed: f32, thickness: u32, trail_frames: u32) -> Self {
        Self { a, b, delta, speed, thickness, trail_frames }
    }
}

impl Scene for LissajousScene {
    fn name(&self) -> &str {
        "lissajous"
    }

    fn render(&self, frame: &mut Frame, context: &RenderContext) {
        let w = frame.width as f32;
        let h = frame.height as f32;
        let cx = w * 0.5;
        let cy = h * 0.5;
        let rx = w * 0.45;
        let ry = h * 0.45;

        let steps = 2000u32;
        let trail = self.trail_frames.max(1);

        for trail_i in 0..=trail {
            let age = trail - trail_i;
            let alpha = ((trail_i as f32 / trail as f32) * 255.0) as u8;

            let frame_offset = context.frame_index.saturating_sub(age);
            let phase = (frame_offset as f32 / context.total_frames as f32) * self.speed * TAU;

            for step in 0..steps {
                let t = (step as f32 / steps as f32) * TAU;
                let x = cx + rx * (self.a * t + phase + self.delta).sin();
                let y = cy + ry * (self.b * t).sin();

                let px = x as i32;
                let py = y as i32;
                let r = self.thickness as i32;

                for dy in -r..=r {
                    for dx in -r..=r {
                        if dx * dx + dy * dy <= r * r {
                            let nx = px + dx;
                            let ny = py + dy;
                            if nx >= 0 && nx < frame.width as i32 && ny >= 0 && ny < frame.height as i32 {
                                frame.set_pixel(nx as u32, ny as u32, Rgba::new(0, alpha, alpha / 2 + 100, 255));
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct LissajousParams {
    #[serde(default = "default_a")]
    pub a: f32,
    #[serde(default = "default_b")]
    pub b: f32,
    #[serde(default = "default_delta")]
    pub delta: f32,
    #[serde(default = "default_speed")]
    pub speed: f32,
    #[serde(default = "default_thickness")]
    pub thickness: u32,
    #[serde(default = "default_trail_frames")]
    pub trail_frames: u32,
}

impl Default for LissajousParams {
    fn default() -> Self {
        Self {
            a: default_a(),
            b: default_b(),
            delta: default_delta(),
            speed: default_speed(),
            thickness: default_thickness(),
            trail_frames: default_trail_frames(),
        }
    }
}

fn default_a() -> f32 { 3.0 }
fn default_b() -> f32 { 2.0 }
fn default_delta() -> f32 { std::f32::consts::FRAC_PI_4 }
fn default_speed() -> f32 { 0.5 }
fn default_thickness() -> u32 { 1 }
fn default_trail_frames() -> u32 { 8 }

#[cfg(test)]
mod tests {
    use super::*;

    fn render_lissajous(frame_index: u32) -> Frame {
        let mut frame = Frame::new(64, 64);
        let ctx = RenderContext::new(frame_index, 60, 24.0, 42);
        let scene = LissajousScene::new(3.0, 2.0, std::f32::consts::FRAC_PI_4, 0.5, 1, 4);
        scene.render(&mut frame, &ctx);
        frame
    }

    #[test]
    fn lissajous_draws_some_pixels() {
        let frame = render_lissajous(0);
        let lit = frame.pixels.iter().any(|p| p.r > 0 || p.g > 0 || p.b > 0);
        assert!(lit);
    }

    #[test]
    fn lissajous_frame0_differs_from_frame15() {
        let f0 = render_lissajous(0);
        let f15 = render_lissajous(15);
        let any_diff = f0.pixels.iter().zip(f15.pixels.iter()).any(|(a, b)| a != b);
        assert!(any_diff);
    }
}
