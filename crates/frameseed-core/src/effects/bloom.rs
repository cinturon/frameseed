use crate::{Effect, Frame, RenderContext, Rgba};
use serde::{Deserialize, Serialize};

pub struct BloomEffect {
    pub threshold: f32,
    pub intensity: f32,
    pub radius: u32,
}

impl BloomEffect {
    pub fn new(threshold: f32, intensity: f32, radius: u32) -> Self {
        Self {
            threshold: threshold.clamp(0.0, 1.0),
            intensity: intensity.clamp(0.0, 4.0),
            radius: radius.clamp(1, 24),
        }
    }
}

impl Effect for BloomEffect {
    fn name(&self) -> &str {
        "bloom"
    }

    fn apply(&mut self, frame: &mut Frame, _context: &RenderContext) {
        let w = frame.width as i32;
        let h = frame.height as i32;
        let r = self.radius as i32;
        let threshold = self.threshold * 255.0;
        let src = frame.pixels.clone();
        let mut bright = vec![Rgba::black(); src.len()];

        for (i, pixel) in src.iter().enumerate() {
            let luma = pixel.r as f32 * 0.2126 + pixel.g as f32 * 0.7152 + pixel.b as f32 * 0.0722;
            if luma >= threshold {
                bright[i] = *pixel;
            }
        }

        for y in 0..h {
            for x in 0..w {
                let mut sum_r = 0u32;
                let mut sum_g = 0u32;
                let mut sum_b = 0u32;
                let mut count = 0u32;
                for ky in (y - r)..=(y + r) {
                    for kx in (x - r)..=(x + r) {
                        if kx >= 0 && kx < w && ky >= 0 && ky < h {
                            let p = bright[(ky * w + kx) as usize];
                            sum_r += p.r as u32;
                            sum_g += p.g as u32;
                            sum_b += p.b as u32;
                            count += 1;
                        }
                    }
                }

                let idx = (y * w + x) as usize;
                let base = src[idx];
                let scale = self.intensity / count.max(1) as f32;
                frame.pixels[idx] = Rgba::new(
                    add_scaled(base.r, sum_r, scale),
                    add_scaled(base.g, sum_g, scale),
                    add_scaled(base.b, sum_b, scale),
                    255,
                );
            }
        }
    }
}

fn add_scaled(channel: u8, glow_sum: u32, scale: f32) -> u8 {
    (channel as f32 + glow_sum as f32 * scale).clamp(0.0, 255.0) as u8
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct BloomParams {
    #[serde(default = "default_threshold")]
    pub threshold: f32,
    #[serde(default = "default_intensity")]
    pub intensity: f32,
    #[serde(default = "default_radius")]
    pub radius: u32,
}

impl Default for BloomParams {
    fn default() -> Self {
        Self {
            threshold: default_threshold(),
            intensity: default_intensity(),
            radius: default_radius(),
        }
    }
}

fn default_threshold() -> f32 {
    0.65
}

fn default_intensity() -> f32 {
    0.8
}

fn default_radius() -> u32 {
    6
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bloom_spreads_bright_pixel() {
        let mut frame = Frame::new(5, 5);
        frame.set_pixel(2, 2, Rgba::white());
        BloomEffect::new(0.5, 1.0, 1).apply(&mut frame, &RenderContext::new(0, 1, 24.0, 1));
        assert!(frame.get_pixel(1, 2).unwrap().r > 0);
    }
}
