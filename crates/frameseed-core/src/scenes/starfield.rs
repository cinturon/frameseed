use crate::{seeded_rng, Frame, RenderContext, Rgba, Scene};
use rand::Rng;
use serde::{Deserialize, Serialize};

pub struct StarfieldScene {
    pub count: u32,
    pub speed: f32,
}

impl StarfieldScene {
    pub fn new(count: u32, speed: f32) -> Self {
        Self { count, speed }
    }
}

impl Scene for StarfieldScene {
    fn name(&self) -> &str {
        "starfield"
    }

    fn render(&self, frame: &mut Frame, context: &RenderContext) {
        let w = frame.width as f32;
        let h = frame.height as f32;
        let cx = w * 0.5;
        let cy = h * 0.5;

        // Each star has a fixed (sx, sy) seed position and a phase offset
        // that controls where in Z it starts. Z ranges (0, 1]: 0 is center, 1 is edge.
        let total_frames = context.total_frames.max(1) as f32;
        let t = context.frame_index as f32 / total_frames * self.speed;

        for i in 0..self.count {
            let mut rng = seeded_rng(context.seed.wrapping_add(i as u64));

            let sx: f32 = rng.random::<f32>() * 2.0 - 1.0; // -1..1
            let sy: f32 = rng.random::<f32>() * 2.0 - 1.0;
            let phase: f32 = rng.random::<f32>();

            // z moves from 1 (far) toward 0 (near) then wraps
            let z = 1.0 - ((phase + t) % 1.0);
            let z = z.max(0.001);

            let px = cx + sx / z * cx;
            let py = cy + sy / z * cy;

            if px < 0.0 || py < 0.0 || px >= w || py >= h {
                continue;
            }

            // Brightness increases as star approaches (z decreases)
            let brightness = ((1.0 - z) * 255.0).clamp(30.0, 255.0) as u8;
            // Size: close stars get a 2x2 blob
            let size = if z < 0.15 { 2u32 } else { 1u32 };

            for dy in 0..size {
                for dx in 0..size {
                    let fx = px as u32 + dx;
                    let fy = py as u32 + dy;
                    if fx < frame.width && fy < frame.height {
                        frame.set_pixel(fx, fy, Rgba::new(brightness, brightness, brightness, 255));
                    }
                }
            }
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct StarfieldParams {
    #[serde(default = "default_count")]
    pub count: u32,
    #[serde(default = "default_speed")]
    pub speed: f32,
}

impl Default for StarfieldParams {
    fn default() -> Self {
        Self {
            count: default_count(),
            speed: default_speed(),
        }
    }
}

fn default_count() -> u32 {
    200
}

fn default_speed() -> f32 {
    0.5
}

#[cfg(test)]
mod tests {
    use super::*;

    fn render_starfield(frame_index: u32) -> Frame {
        let mut frame = Frame::new(64, 64);
        let ctx = RenderContext::new(frame_index, 120, 24.0, 42);
        StarfieldScene::new(200, 0.5).render(&mut frame, &ctx);
        frame
    }

    #[test]
    fn starfield_draws_stars() {
        let frame = render_starfield(0);
        let lit = frame.pixels.iter().any(|p| p.r > 0);
        assert!(lit);
    }

    #[test]
    fn starfield_is_deterministic() {
        let f0 = render_starfield(0);
        let f0b = render_starfield(0);
        assert!(f0.pixels.iter().zip(f0b.pixels.iter()).all(|(a, b)| a == b));
    }

    #[test]
    fn starfield_animates() {
        let f0 = render_starfield(0);
        let f60 = render_starfield(60);
        let any_diff = f0.pixels.iter().zip(f60.pixels.iter()).any(|(a, b)| a != b);
        assert!(any_diff);
    }
}
