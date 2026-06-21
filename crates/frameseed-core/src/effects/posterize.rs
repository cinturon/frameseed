use crate::{Effect, Frame, RenderContext};
use serde::{Deserialize, Serialize};

pub struct PosterizeEffect {
    pub levels: u8,
}

impl PosterizeEffect {
    pub fn new(levels: u8) -> Self {
        Self {
            levels: levels.clamp(2, 32),
        }
    }
}

impl Effect for PosterizeEffect {
    fn name(&self) -> &str {
        "posterize"
    }

    fn apply(&mut self, frame: &mut Frame, _context: &RenderContext) {
        for pixel in &mut frame.pixels {
            pixel.r = posterize_channel(pixel.r, self.levels);
            pixel.g = posterize_channel(pixel.g, self.levels);
            pixel.b = posterize_channel(pixel.b, self.levels);
        }
    }
}

fn posterize_channel(channel: u8, levels: u8) -> u8 {
    let levels = levels.max(2) as f32;
    let step = 255.0 / (levels - 1.0);
    ((channel as f32 / step).round() * step).clamp(0.0, 255.0) as u8
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct PosterizeParams {
    #[serde(default = "default_levels")]
    pub levels: u8,
}

impl Default for PosterizeParams {
    fn default() -> Self {
        Self {
            levels: default_levels(),
        }
    }
}

fn default_levels() -> u8 {
    6
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn posterize_reduces_channel_levels() {
        assert_eq!(posterize_channel(130, 2), 255);
        assert_eq!(posterize_channel(100, 4), 85);
    }
}
