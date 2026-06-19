use crate::{Frame, RenderContext, Effect, Rgba};
use serde::Deserialize;

pub struct PixelationEffect{
    pub block_size: u32,
}

impl PixelationEffect {
    pub fn new(block_size: u32) -> Self {
        let block_size = block_size.clamp(1, u32::MAX);
        Self { block_size }
    }
}

impl Effect for PixelationEffect {
    fn name(&self) -> &str {
        "pixelation"
    }

    fn apply(&self, frame: &mut Frame, _context: &RenderContext) {
        let block = self.block_size.max(1);

        // Create a new frame with the same dimensions as the input frame
        let mut output = frame.clone();

        // Iterate over the blocks in the frame
        for block_y in (0..frame.height).step_by(block as usize) {
            for block_x in (0..frame.width).step_by(block as usize) {
                let sample = frame.get_pixel(block_x, block_y).unwrap_or(Rgba::black());

                for y in block_y..(block_y + block).min(frame.height) {
                    for x in block_x..(block_x + block).min(frame.width) {
                        output.set_pixel(x, y, sample);
                    }
                }
            }
        }
        // Copy the pixels from the output frame to the input frame
        frame.pixels = output.pixels;
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct PixelationParams {
    #[serde(default = "default_block_size")]
    pub block_size: u32,
}

impl Default for PixelationParams {
    fn default() -> Self {
        Self { block_size: default_block_size() }
    }
}

fn default_block_size() -> u32 {
    return 8;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        let effect = PixelationEffect::new(8);
        assert_eq!(effect.name(), "pixelation");
    }

    #[test]
    fn test_apply() {
        let mut frame = Frame::new(100, 100);
        frame.set_pixel(0, 0, Rgba::new(255, 0, 0, 255));
        let context = RenderContext::new(0, 120, 24.0, 42);
        PixelationEffect::new(8).apply(&mut frame, &context);
        assert_eq!(frame.get_pixel(0, 0), Some(Rgba::new(255, 0, 0, 255)));
    }

    #[test]
    fn block_size_zero_is_one() {
        let effect = PixelationEffect::new(0);
        assert_eq!(effect.block_size, 1);
    }

    #[test]
    fn block_size_one_is_noop() {
        let effect = PixelationEffect::new(1);
        assert_eq!(effect.block_size, 1);
    }

    #[test]
    fn frame_dimensions_are_preserved() {
        let mut frame = Frame::new(100, 100);
        frame.set_pixel(0, 0, Rgba::new(255, 0, 0, 255));
        let context = RenderContext::new(0, 120, 24.0, 42);
        PixelationEffect::new(8).apply(&mut frame, &context);
        assert_eq!(frame.width, 100);
        assert_eq!(frame.height, 100);
        assert_eq!(frame.get_pixel(0, 0), Some(Rgba::new(255, 0, 0, 255)));
        assert_eq!(frame.get_pixel(1, 1), Some(Rgba::new(255, 0, 0, 255)));
    }

    #[test]
    fn block_size_is_clamped_to_one() {
        let effect = PixelationEffect::new(0);
        assert_eq!(effect.block_size, 1);   
    }
}