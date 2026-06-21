use crate::Effect;
use crate::Frame;
use crate::RenderContext;

pub struct InvertEffect;

impl Default for InvertEffect {
    fn default() -> Self {
        Self
    }
}

impl Effect for InvertEffect {
    fn name(&self) -> &str {
        "invert"
    }

    fn apply(&mut self, frame: &mut Frame, _context: &RenderContext) {
        for x in 0..frame.width {
            for y in 0..frame.height {
                if let Some(pixel) = frame.get_pixel(x, y) {
                    frame.set_pixel(x, y, pixel.invert());
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Rgba;

    #[test]
    fn test_name() {
        let effect = InvertEffect::default();
        assert_eq!(effect.name(), "invert");
    }

    #[test]
    fn test_apply() {
        let mut frame = Frame::new(100, 100);
        frame.set_pixel(0, 0, Rgba::new(255, 0, 0, 255));
        let context = RenderContext::new(0, 120, 24.0, 42);
        InvertEffect::default().apply(&mut frame, &context);
        assert_eq!(frame.get_pixel(0, 0), Some(Rgba::new(0, 255, 255, 255)));
    }
}
