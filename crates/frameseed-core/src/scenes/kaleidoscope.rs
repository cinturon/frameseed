use crate::{Frame, RenderContext, Rgba, Scene};
use serde::{Deserialize, Serialize};
use std::f32::consts::{PI, TAU};

pub struct KaleidoscopeScene {
    pub segments: u32,
    pub speed: f32,
    pub zoom: f32,
}

impl KaleidoscopeScene {
    pub fn new(segments: u32, speed: f32, zoom: f32) -> Self {
        Self {
            segments: segments.max(2),
            speed,
            zoom: zoom.max(0.1),
        }
    }
}

impl Scene for KaleidoscopeScene {
    fn name(&self) -> &str {
        "kaleidoscope"
    }

    fn render(&self, frame: &mut Frame, context: &RenderContext) {
        let w = frame.width as f32;
        let h = frame.height as f32;
        let aspect = w / h.max(1.0);
        let segments = self.segments as f32;
        let wedge = TAU / segments;
        let t = context.normalized_time * self.speed * TAU;
        let zoom = self.zoom;

        frame.parallel_for_each_pixel(move |x, y| {
            let mut px = (x as f32 / w - 0.5) * 2.0 * aspect;
            let mut py = (y as f32 / h - 0.5) * 2.0;

            let rot = t * 0.15;
            let cos_r = rot.cos();
            let sin_r = rot.sin();
            let rx = px * cos_r - py * sin_r;
            let ry = px * sin_r + py * cos_r;
            px = rx;
            py = ry;

            let radius = (px * px + py * py).sqrt();
            let mut angle = py.atan2(px) + t * 0.08;
            angle = angle.rem_euclid(wedge);
            angle = (angle - wedge * 0.5).abs();

            let folded_x = angle.cos() * radius * zoom;
            let folded_y = angle.sin() * radius * zoom;
            let ripples = ((radius * 8.0 - t).sin() + 1.0) * 0.5;
            let spokes = ((angle / wedge * PI * 2.0).cos() + 1.0) * 0.5;
            let lattice =
                ((folded_x * 9.0 + t).sin() * (folded_y * 7.0 - t * 0.7).cos() + 1.0) * 0.5;
            let v = (ripples * 0.45 + spokes * 0.25 + lattice * 0.30).clamp(0.0, 1.0);

            let hue = v * TAU + radius * 1.7 + t * 0.2;
            let r = (hue.sin() * 127.0 + 128.0) as u8;
            let g = ((hue + TAU / 3.0).sin() * 127.0 + 128.0) as u8;
            let b = ((hue + 2.0 * TAU / 3.0).sin() * 127.0 + 128.0) as u8;
            Rgba::new(r, g, b, 255)
        });
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct KaleidoscopeParams {
    #[serde(default = "default_segments")]
    pub segments: u32,
    #[serde(default = "default_speed")]
    pub speed: f32,
    #[serde(default = "default_zoom")]
    pub zoom: f32,
}

impl Default for KaleidoscopeParams {
    fn default() -> Self {
        Self {
            segments: default_segments(),
            speed: default_speed(),
            zoom: default_zoom(),
        }
    }
}

fn default_segments() -> u32 {
    8
}

fn default_speed() -> f32 {
    0.8
}

fn default_zoom() -> f32 {
    2.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kaleidoscope_draws_color() {
        let mut frame = Frame::new(64, 64);
        let ctx = RenderContext::new(0, 48, 24.0, 42);
        KaleidoscopeScene::new(8, 0.8, 2.0).render(&mut frame, &ctx);
        assert!(frame.pixels.iter().any(|p| p.r != p.g || p.g != p.b));
    }

    #[test]
    fn kaleidoscope_animates() {
        let mut first = Frame::new(64, 64);
        let mut second = Frame::new(64, 64);
        KaleidoscopeScene::new(8, 0.8, 2.0)
            .render(&mut first, &RenderContext::new(0, 48, 24.0, 42));
        KaleidoscopeScene::new(8, 0.8, 2.0)
            .render(&mut second, &RenderContext::new(24, 48, 24.0, 42));
        assert!(
            first
                .pixels
                .iter()
                .zip(second.pixels.iter())
                .any(|(a, b)| a != b)
        );
    }
}
