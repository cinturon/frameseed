use crate::{Frame, RenderContext, Rgba, Scene};
use serde::{Deserialize, Serialize};
use std::f32::consts::TAU;

pub struct TunnelScene {
    pub speed: f32,
    pub rings: u32,
}

impl TunnelScene {
    pub fn new(speed: f32, rings: u32) -> Self {
        Self { speed, rings }
    }
}

impl Scene for TunnelScene {
    fn name(&self) -> &str {
        "tunnel"
    }

    fn render(&self, frame: &mut Frame, context: &RenderContext) {
        let w = frame.width as f32;
        let h = frame.height as f32;
        let cx = w * 0.5;
        let cy = h * 0.5;
        let t = context.normalized_time * self.speed;
        let rings = self.rings as f32;

        frame.parallel_for_each_pixel(move |x, y| {
            let dx = x as f32 - cx;
            let dy = y as f32 - cy;

            let dist = (dx * dx + dy * dy).sqrt();
            if dist < 0.001 {
                return Rgba::black();
            }

            let angle = dy.atan2(dx);

            // Tunnel depth: inverse of distance — closer to center = deeper
            let depth = 64.0 / dist;
            let depth_phase = (depth + t) % 1.0;

            // Stripe pattern in depth
            let stripe = (depth_phase * rings).fract();
            let brightness = if stripe < 0.5 { 200u8 } else { 80u8 };

            // Color based on angle
            let hue = (angle / TAU + 0.5 + t * 0.1) % 1.0;
            let (r, g, b) = hsv_to_rgb(hue, 0.7, brightness as f32 / 255.0);

            Rgba::new(r, g, b, 255)
        });
    }
}

fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (u8, u8, u8) {
    let h = h * 6.0;
    let i = h.floor() as u32 % 6;
    let f = h - h.floor();
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

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct TunnelParams {
    #[serde(default = "default_speed")]
    pub speed: f32,
    #[serde(default = "default_rings")]
    pub rings: u32,
}

impl Default for TunnelParams {
    fn default() -> Self {
        Self {
            speed: 1.0,
            rings: 8,
        }
    }
}

fn default_speed() -> f32 {
    1.0
}
fn default_rings() -> u32 {
    8
}

#[cfg(test)]
mod tests {
    use super::*;

    fn render_tunnel(frame_index: u32) -> Frame {
        let mut frame = Frame::new(64, 64);
        let ctx = RenderContext::new(frame_index, 60, 24.0, 42);
        TunnelScene::new(1.0, 8).render(&mut frame, &ctx);
        frame
    }

    #[test]
    fn tunnel_draws_colored_pixels() {
        let frame = render_tunnel(0);
        let any_color = frame.pixels.iter().any(|p| p.r > 0 || p.g > 0 || p.b > 0);
        assert!(any_color);
    }

    #[test]
    fn tunnel_animates() {
        let f0 = render_tunnel(0);
        let f30 = render_tunnel(30);
        let any_diff = f0.pixels.iter().zip(f30.pixels.iter()).any(|(a, b)| a != b);
        assert!(any_diff);
    }

    #[test]
    fn tunnel_is_deterministic() {
        let f0a = render_tunnel(0);
        let f0b = render_tunnel(0);
        assert!(
            f0a.pixels
                .iter()
                .zip(f0b.pixels.iter())
                .all(|(a, b)| a == b)
        );
    }
}
