use crate::{Effect, Frame, RenderContext};
use serde::{Deserialize, Serialize};

pub struct BrightnessContrastEffect {
    pub brightness: f32,
    pub contrast: f32,
}

impl BrightnessContrastEffect {
    pub fn new(brightness: f32, contrast: f32) -> Self {
        Self {
            brightness,
            contrast,
        }
    }
}

impl Effect for BrightnessContrastEffect {
    fn name(&self) -> &str {
        "brightness_contrast"
    }

    fn apply(&mut self, frame: &mut Frame, _context: &RenderContext) {
        let b = self.brightness;
        let c = self.contrast;

        for pixel in &mut frame.pixels {
            pixel.r = adjust(pixel.r, b, c);
            pixel.g = adjust(pixel.g, b, c);
            pixel.b = adjust(pixel.b, b, c);
        }
    }
}

fn adjust(channel: u8, brightness: f32, contrast: f32) -> u8 {
    let v = channel as f32 / 255.0;
    // Brightness: add offset
    let v = v + brightness;
    // Contrast: scale around midpoint 0.5
    let v = (v - 0.5) * contrast + 0.5;
    (v.clamp(0.0, 1.0) * 255.0) as u8
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct BrightnessContrastParams {
    #[serde(default = "default_brightness")]
    pub brightness: f32,
    #[serde(default = "default_contrast")]
    pub contrast: f32,
}

impl Default for BrightnessContrastParams {
    fn default() -> Self {
        Self {
            brightness: default_brightness(),
            contrast: default_contrast(),
        }
    }
}

fn default_brightness() -> f32 {
    0.0
}
fn default_contrast() -> f32 {
    1.0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Rgba;

    #[test]
    fn brightness_zero_contrast_one_is_noop() {
        let mut frame = Frame::new(4, 4);
        for p in &mut frame.pixels {
            *p = Rgba::new(128, 64, 200, 255);
        }
        let original = frame.pixels.clone();
        let ctx = RenderContext::new(0, 1, 24.0, 1);
        BrightnessContrastEffect::new(0.0, 1.0).apply(&mut frame, &ctx);
        assert_eq!(frame.pixels, original);
    }

    #[test]
    fn negative_brightness_darkens() {
        let mut frame = Frame::new(1, 1);
        frame.pixels[0] = Rgba::new(200, 200, 200, 255);
        let ctx = RenderContext::new(0, 1, 24.0, 1);
        BrightnessContrastEffect::new(-0.2, 1.0).apply(&mut frame, &ctx);
        assert!(frame.pixels[0].r < 200);
    }

    #[test]
    fn high_contrast_pushes_to_extremes() {
        let mut frame = Frame::new(2, 1);
        frame.pixels[0] = Rgba::new(200, 200, 200, 255); // above midpoint
        frame.pixels[1] = Rgba::new(50, 50, 50, 255); // below midpoint
        let ctx = RenderContext::new(0, 1, 24.0, 1);
        BrightnessContrastEffect::new(0.0, 3.0).apply(&mut frame, &ctx);
        assert!(frame.pixels[0].r > 200);
        assert!(frame.pixels[1].r < 50);
    }
}
