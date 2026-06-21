use crate::{Effect, Frame, RenderContext, lerp_rgba};
use serde::{Deserialize, Serialize};

pub struct MotionBlurEffect {
    strength: f32,
    previous_frame: Option<Frame>,
}

impl MotionBlurEffect {
    pub fn new(strength: f32) -> Self {
        Self {
            strength: strength.clamp(0.0, 1.0),
            previous_frame: None,
        }
    }
}

impl Effect for MotionBlurEffect {
    fn name(&self) -> &str {
        "motion_blur"
    }

    fn apply(&mut self, frame: &mut Frame, _context: &RenderContext) {
        if let Some(previous_frame) = &self.previous_frame {
            for y in 0..frame.height {
                for x in 0..frame.width {
                    if let Some(current_pixel) = frame.get_pixel(x, y) {
                        let previous_pixel = previous_frame.get_pixel(x, y).unwrap();
                        let blended_pixel =
                            lerp_rgba(previous_pixel, current_pixel, 1.0 - self.strength);
                        frame.set_pixel(x, y, blended_pixel);
                    }
                }
            }
        }
        self.previous_frame = Some(frame.clone());
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct MotionBlurParams {
    #[serde(default = "default_strength")]
    pub strength: f32,
}

impl Default for MotionBlurParams {
    fn default() -> Self {
        Self {
            strength: default_strength(),
        }
    }
}

fn default_strength() -> f32 {
    0.5
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Rgba;

    fn ctx(frame_index: u32) -> RenderContext {
        RenderContext::new(frame_index, 10, 24.0, 42)
    }

    fn single_pixel_frame(color: Rgba) -> Frame {
        let mut frame = Frame::new(1, 1);
        frame.set_pixel(0, 0, color);
        frame
    }

    #[test]
    fn name_is_motion_blur() {
        let effect = MotionBlurEffect::new(0.5);
        assert_eq!(effect.name(), "motion_blur");
    }

    #[test]
    fn first_frame_passes_through_unchanged() {
        let color = Rgba::new(200, 100, 50, 255);
        let mut frame = single_pixel_frame(color);
        let mut effect = MotionBlurEffect::new(0.5);

        effect.apply(&mut frame, &ctx(0));

        assert_eq!(frame.get_pixel(0, 0), Some(color));
    }

    #[test]
    fn strength_zero_preserves_current_on_second_frame() {
        let mut effect = MotionBlurEffect::new(0.0);

        let mut frame0 = single_pixel_frame(Rgba::black());
        effect.apply(&mut frame0, &ctx(0));

        let mut frame1 = single_pixel_frame(Rgba::white());
        effect.apply(&mut frame1, &ctx(1));

        assert_eq!(frame1.get_pixel(0, 0), Some(Rgba::white()));
    }

    #[test]
    fn strength_one_preserves_previous_on_second_frame() {
        let mut effect = MotionBlurEffect::new(1.0);

        let mut frame0 = single_pixel_frame(Rgba::black());
        effect.apply(&mut frame0, &ctx(0));

        let mut frame1 = single_pixel_frame(Rgba::white());
        effect.apply(&mut frame1, &ctx(1));

        assert_eq!(frame1.get_pixel(0, 0), Some(Rgba::black()));
    }

    #[test]
    fn strength_half_blends_black_into_white() {
        let mut effect = MotionBlurEffect::new(0.5);

        let mut frame0 = single_pixel_frame(Rgba::black());
        effect.apply(&mut frame0, &ctx(0));

        let mut frame1 = single_pixel_frame(Rgba::white());
        effect.apply(&mut frame1, &ctx(1));

        assert_eq!(
            frame1.get_pixel(0, 0),
            Some(lerp_rgba(Rgba::black(), Rgba::white(), 0.5))
        );
    }

    #[test]
    fn second_frame_blends_when_content_changes() {
        let mut effect = MotionBlurEffect::new(0.5);

        let mut frame0 = single_pixel_frame(Rgba::black());
        effect.apply(&mut frame0, &ctx(0));

        let mut frame1 = single_pixel_frame(Rgba::white());
        effect.apply(&mut frame1, &ctx(1));

        let blended = frame1.get_pixel(0, 0).unwrap();
        assert_ne!(blended, Rgba::black());
        assert_ne!(blended, Rgba::white());
    }

    #[test]
    fn same_sequence_is_deterministic() {
        let run = || {
            let mut effect = MotionBlurEffect::new(0.35);
            let mut frame0 = single_pixel_frame(Rgba::new(40, 80, 120, 255));
            effect.apply(&mut frame0, &ctx(0));
            let mut frame1 = single_pixel_frame(Rgba::new(200, 60, 10, 255));
            effect.apply(&mut frame1, &ctx(1));
            frame1.get_pixel(0, 0)
        };

        assert_eq!(run(), run());
    }

    #[test]
    fn strength_is_clamped_to_unit_interval() {
        let mut high = MotionBlurEffect::new(5.0);
        let mut one = MotionBlurEffect::new(1.0);

        let mut frame0_high = single_pixel_frame(Rgba::black());
        let mut frame0_one = single_pixel_frame(Rgba::black());
        high.apply(&mut frame0_high, &ctx(0));
        one.apply(&mut frame0_one, &ctx(0));

        let mut frame1_high = single_pixel_frame(Rgba::white());
        let mut frame1_one = single_pixel_frame(Rgba::white());
        high.apply(&mut frame1_high, &ctx(1));
        one.apply(&mut frame1_one, &ctx(1));

        assert_eq!(frame1_high.get_pixel(0, 0), frame1_one.get_pixel(0, 0));
    }
}
