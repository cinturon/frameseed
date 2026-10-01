use crate::{Effect, Frame, RenderContext, Rgba};
use serde::{Deserialize, Serialize};

pub struct ChromaticAberrationEffect {
    pub offset: f32,
}

impl ChromaticAberrationEffect {
    pub fn new(offset: f32) -> Self {
        Self {
            offset: offset.clamp(0.0, 32.0),
        }
    }
}

impl Effect for ChromaticAberrationEffect {
    fn name(&self) -> &str {
        "chromatic_aberration"
    }

    fn apply(&mut self, frame: &mut Frame, _context: &RenderContext) {
        let src = frame.pixels.clone();
        let w = frame.width as i32;
        let h = frame.height as i32;
        let cx = frame.width as f32 * 0.5;
        let cy = frame.height as f32 * 0.5;

        for y in 0..h {
            for x in 0..w {
                let dx = x as f32 - cx;
                let dy = y as f32 - cy;
                let len = (dx * dx + dy * dy).sqrt().max(1.0);
                let ox = (dx / len * self.offset).round() as i32;
                let oy = (dy / len * self.offset).round() as i32;
                let r = sample(&src, w, h, x + ox, y + oy).r;
                let g = sample(&src, w, h, x, y).g;
                let b = sample(&src, w, h, x - ox, y - oy).b;
                frame.set_pixel(x as u32, y as u32, Rgba::new(r, g, b, 255));
            }
        }
    }
}

fn sample(src: &[Rgba], w: i32, h: i32, x: i32, y: i32) -> Rgba {
    let x = x.clamp(0, w - 1);
    let y = y.clamp(0, h - 1);
    src[(y * w + x) as usize]
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct ChromaticAberrationParams {
    #[serde(default = "default_offset")]
    pub offset: f32,
}

impl Default for ChromaticAberrationParams {
    fn default() -> Self {
        Self {
            offset: default_offset(),
        }
    }
}

fn default_offset() -> f32 {
    2.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chromatic_aberration_keeps_dimensions() {
        let mut frame = Frame::new(8, 8);
        ChromaticAberrationEffect::new(2.0).apply(&mut frame, &RenderContext::new(0, 1, 24.0, 1));
        assert_eq!(frame.pixels.len(), 64);
    }
}
