use crate::{Effect, Frame, RenderContext};
use serde::{Deserialize, Serialize};

pub struct VignetteEffect {
    pub strength: f32,
    pub radius: f32,
}

impl VignetteEffect {
    pub fn new(strength: f32, radius: f32) -> Self {
        Self {
            strength: strength.clamp(0.0, 1.0),
            radius: radius.clamp(0.05, 1.35),
        }
    }
}

impl Effect for VignetteEffect {
    fn name(&self) -> &str {
        "vignette"
    }

    fn apply(&mut self, frame: &mut Frame, _context: &RenderContext) {
        let w = frame.width as f32;
        let h = frame.height as f32;
        let aspect = w / h.max(1.0);

        for y in 0..frame.height {
            for x in 0..frame.width {
                let px = (x as f32 / w - 0.5) * 2.0 * aspect;
                let py = (y as f32 / h - 0.5) * 2.0;
                let distance = (px * px + py * py).sqrt();
                let falloff = (1.4 - self.radius).max(0.05);
                let darken = ((distance - self.radius) / falloff).clamp(0.0, 1.0);
                let scale = 1.0 - darken * self.strength;
                let idx = (y * frame.width + x) as usize;
                frame.pixels[idx].r = (frame.pixels[idx].r as f32 * scale) as u8;
                frame.pixels[idx].g = (frame.pixels[idx].g as f32 * scale) as u8;
                frame.pixels[idx].b = (frame.pixels[idx].b as f32 * scale) as u8;
            }
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct VignetteParams {
    #[serde(default = "default_strength")]
    pub strength: f32,
    #[serde(default = "default_radius")]
    pub radius: f32,
}

impl Default for VignetteParams {
    fn default() -> Self {
        Self {
            strength: default_strength(),
            radius: default_radius(),
        }
    }
}

fn default_strength() -> f32 {
    0.55
}

fn default_radius() -> f32 {
    0.65
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Rgba;

    #[test]
    fn vignette_darkens_corners() {
        let mut frame = Frame::new(8, 8);
        frame.clear(Rgba::white());
        VignetteEffect::new(1.0, 0.2).apply(&mut frame, &RenderContext::new(0, 1, 24.0, 1));
        assert!(frame.get_pixel(0, 0).unwrap().r < frame.get_pixel(4, 4).unwrap().r);
    }
}
