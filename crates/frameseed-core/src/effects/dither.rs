use crate::{Frame, RenderContext, Effect, Rgba};
use serde::{Deserialize, Serialize};

pub struct OrderedDitherEffect {
    pub spread: f32,
}

impl OrderedDitherEffect {
    pub fn new(spread: f32) -> Self {
        Self { spread }
    }
}

impl Effect for OrderedDitherEffect {
    fn name(&self) -> &str {
        "dither"
    }

    fn apply(&mut self, frame: &mut Frame, _context: &RenderContext) {
        let spread = self.spread;
        let source_frame = frame.pixels.clone();
        let width = frame.width;

        frame.parallel_for_each_pixel(move |x, y| {
            let index = (y * width + x) as usize;
            let pixel = source_frame[index];
            dither_pixel(pixel, x, y, spread)
        });  
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct DitherParams {
    #[serde(default = "default_spread")]
    pub spread: f32
}

impl Default for DitherParams {
    fn default() -> Self {
        Self { spread: default_spread() }
    }
}

fn default_spread() -> f32 {
    48.0
}

const BAYER_SIZE: u32 = 4;
const BAYER_4X4: [[u8; 4]; 4] = [
    [ 0,  8,  2, 10],
    [12,  4, 14,  6],
    [ 3, 11,  1,  9],
    [15,  7, 13,  5],
];

fn bayer_threshold(x: u32, y: u32) -> f32 {
    let bx = (x % BAYER_SIZE) as usize;
    let by = (y % BAYER_SIZE) as usize;
    let value  = BAYER_4X4[by][bx] as f32;
    (value + 0.5) / (BAYER_SIZE * BAYER_SIZE) as f32
}

fn dither_channel(channel: u8, threshold: f32, spread: f32) -> u8 {
    let offset = spread * (threshold - 0.5);
    (channel as f32 + offset).clamp(0.0, 255.0) as u8
}

fn dither_pixel(pixel: Rgba, x: u32, y: u32, spread: f32) -> Rgba {
    Rgba {
        r: dither_channel(pixel.r, bayer_threshold(x, y), spread),
        g: dither_channel(pixel.g, bayer_threshold(x, y), spread),
        b: dither_channel(pixel.b, bayer_threshold(x, y), spread),
        a: pixel.a,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
fn bayer_is_deterministic() {
    assert_eq!(bayer_threshold(3, 7), bayer_threshold(3, 7));
}
#[test]
fn bayer_tiles_every_four_pixels() {
    assert_eq!(bayer_threshold(0, 0), bayer_threshold(4, 0));
}
#[test]
fn dither_changes_neighbor_pixels_differently() {
    let gray = Rgba::new(128, 128, 128, 255);
    let a = dither_pixel(gray, 0, 0, 48.0);
    let b = dither_pixel(gray, 1, 0, 48.0);
    assert_ne!(a, b);
}
#[test]
fn apply_is_deterministic() {
    // render helper: apply twice on same frame → identical pixels
    let mut frame = Frame::new(100, 100);
    frame.set_pixel(0, 0, Rgba::new(128, 128, 128, 255));
    let context = RenderContext::new(0, 120, 24.0, 42);
    OrderedDitherEffect::new(48.0).apply(&mut frame, &context);
    OrderedDitherEffect::new(48.0).apply(&mut frame, &context);
    assert_eq!(frame.get_pixel(0, 0), Some(Rgba::new(82, 82, 82, 255)));
}
}