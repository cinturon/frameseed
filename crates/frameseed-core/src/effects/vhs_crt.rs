use std::f32::consts::TAU;

use crate::{Effect, Frame, RenderContext, Rgba, seeded_rng};
use rand::Rng;
use serde::{Deserialize, Serialize};

pub struct VhsCrtEffect {
    pub scanlines_strength: f32,
    pub chromatic_offset: f32,
    pub noise_amount: f32,
    pub warp_amount: f32,
}

impl VhsCrtEffect {
    pub fn new(
        scanlines_strength: f32,
        chromatic_offset: f32,
        noise_amount: f32,
        warp_amount: f32,
    ) -> Self {
        Self {
            scanlines_strength: scanlines_strength.clamp(0.0, 1.0),
            chromatic_offset: chromatic_offset,
            noise_amount: noise_amount.clamp(0.0, 1.0),
            warp_amount: warp_amount,
        }
    }
}

impl Effect for VhsCrtEffect {
    fn name(&self) -> &str {
        "vhs_crt"
    }

    fn apply(&mut self, frame: &mut Frame, context: &RenderContext) {
        apply_scanlines(frame, self.scanlines_strength);

        let before_chromatic = frame.clone();
        apply_chromatic_offset(&before_chromatic, frame, self.chromatic_offset);

        let before_warp = frame.clone();
        apply_warp(&before_warp, frame, self.warp_amount, context);

        apply_noise(frame, self.noise_amount, context);
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct VhsCrtParams {
    #[serde(default = "default_scanlines_strength")]
    pub scanlines_strength: f32,
    #[serde(default = "default_chromatic_offset")]
    pub chromatic_offset: f32,
    #[serde(default = "default_noise_amount")]
    pub noise_amount: f32,
    #[serde(default = "default_warp_amount")]
    pub warp_amount: f32,
}

impl Default for VhsCrtParams {
    fn default() -> Self {
        Self {
            scanlines_strength: default_scanlines_strength(),
            chromatic_offset: default_chromatic_offset(),
            noise_amount: default_noise_amount(),
            warp_amount: default_warp_amount(),
        }
    }
}

fn default_scanlines_strength() -> f32 {
    0.35
}

fn default_chromatic_offset() -> f32 {
    2.0
}

fn default_noise_amount() -> f32 {
    0.08
}

fn default_warp_amount() -> f32 {
    2.5
}

fn darken_channel(channel: u8, strength: f32) -> u8 {
    let factor = 1.0 - strength.clamp(0.0, 1.0);
    (channel as f32 * factor).clamp(0.0, 255.0) as u8
}

fn apply_scanlines(frame: &mut Frame, scanlines_strength: f32) {
    if scanlines_strength <= 0.0 {
        return;
    }

    for y in 0..frame.height {
        if y % 2 != 0 {
            continue;
        }
        for x in 0..frame.width {
            if let Some(pixel) = frame.get_pixel(x, y) {
                frame.set_pixel(
                    x,
                    y,
                    Rgba::new(
                        darken_channel(pixel.r, scanlines_strength),
                        darken_channel(pixel.g, scanlines_strength),
                        darken_channel(pixel.b, scanlines_strength),
                        pixel.a,
                    ),
                );
            }
        }
    }
}

fn sample_pixel(source_frame: &Frame, x: i32, y: i32) -> Rgba {
    let max_x = source_frame.width.saturating_sub(1) as i32;
    let max_y = source_frame.height.saturating_sub(1) as i32;

    let x = x.clamp(0, max_x) as u32;
    let y = y.clamp(0, max_y) as u32;

    source_frame.get_pixel(x, y).unwrap_or(Rgba::black())
}

fn apply_chromatic_offset(source_frame: &Frame, output_frame: &mut Frame, chromatic_offset: f32) {
    let offset = chromatic_offset.round() as i32;
    if offset == 0 {
        output_frame.pixels = source_frame.pixels.clone();
        return;
    }

    for y in 0..source_frame.height {
        for x in 0..source_frame.width {
            let xi = x as i32;

            let r = sample_pixel(source_frame, xi + offset, y as i32).r;
            let g = sample_pixel(source_frame, xi, y as i32).g;
            let b = sample_pixel(source_frame, xi - offset, y as i32).b;
            let a = sample_pixel(source_frame, xi, y as i32).a;
            output_frame.set_pixel(x, y, Rgba::new(r, g, b, a));
        }
    }
}

fn apply_warp(
    source_frame: &Frame,
    output_frame: &mut Frame,
    warp_amount: f32,
    context: &RenderContext,
) {
    if warp_amount <= 0.0 {
        output_frame.pixels = source_frame.pixels.clone();
        return;
    }

    for y in 0..source_frame.height {
        let wave = (y as f32 * 0.5 + context.normalized_time * TAU).sin();
        let shift = (wave * warp_amount).round() as i32;

        for x in 0..source_frame.width {
            let sample = sample_pixel(source_frame, x as i32 + shift, y as i32);
            output_frame.set_pixel(x, y, sample);
        }
    }
}

fn pixel_noise(context: &RenderContext, x: i32, y: i32) -> f32 {
    let cell_seed = context
        .seed
        .wrapping_add(context.frame_index as u64)
        .wrapping_add(x as u64)
        .wrapping_add(y as u64)
        .wrapping_shl(16);
    seeded_rng(cell_seed).random::<f32>()
}

fn add_grain(channel: u8, grain: f32, noise_amount: f32) -> u8 {
    let delta = (grain - 0.5) * noise_amount * 255.0;
    (channel as f32 + delta).clamp(0.0, 255.0) as u8
}

fn apply_noise(frame: &mut Frame, noise_amount: f32, context: &RenderContext) {
    if noise_amount <= 0.0 {
        return;
    }

    for y in 0..frame.height {
        for x in 0..frame.width {
            if let Some(pixel) = frame.get_pixel(x, y) {
                let grain = pixel_noise(context, x as i32, y as i32);
                frame.set_pixel(
                    x,
                    y,
                    Rgba::new(
                        add_grain(pixel.r, grain, noise_amount),
                        add_grain(pixel.g, grain, noise_amount),
                        add_grain(pixel.b, grain, noise_amount),
                        pixel.a,
                    ),
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx(frame_index: u32) -> RenderContext {
        RenderContext::new(frame_index, 10, 24.0, 42)
    }

    fn solid_frame(width: u32, height: u32, color: Rgba) -> Frame {
        let mut frame = Frame::new(width, height);
        for y in 0..height {
            for x in 0..width {
                frame.set_pixel(x, y, color);
            }
        }
        frame
    }

    #[test]
    fn name_is_vhs_crt() {
        let effect = VhsCrtEffect::new(0.35, 2.0, 0.08, 2.5);
        assert_eq!(effect.name(), "vhs_crt");
    }

    #[test]
    fn sample_pixel_clamps_to_edges() {
        let mut frame = Frame::new(2, 1);
        frame.set_pixel(0, 0, Rgba::new(255, 0, 0, 255));
        frame.set_pixel(1, 0, Rgba::new(0, 255, 0, 255));

        assert_eq!(sample_pixel(&frame, -5, 0), Rgba::new(255, 0, 0, 255));
        assert_eq!(sample_pixel(&frame, 10, 0), Rgba::new(0, 255, 0, 255));
    }

    #[test]
    fn scanlines_darken_even_rows_only() {
        let gray = Rgba::new(100, 100, 100, 255);
        let mut frame = Frame::new(1, 2);
        frame.set_pixel(0, 0, gray);
        frame.set_pixel(0, 1, gray);

        apply_scanlines(&mut frame, 0.5);

        let even = frame.get_pixel(0, 0).unwrap();
        let odd = frame.get_pixel(0, 1).unwrap();

        assert_eq!(even, Rgba::new(50, 50, 50, 255));
        assert_eq!(odd, gray);
    }

    #[test]
    fn scanlines_skipped_when_strength_is_zero() {
        let gray = Rgba::new(100, 100, 100, 255);
        let mut frame = Frame::new(1, 2);
        frame.set_pixel(0, 0, gray);
        frame.set_pixel(0, 1, gray);

        apply_scanlines(&mut frame, 0.0);

        assert_eq!(frame.get_pixel(0, 0), Some(gray));
        assert_eq!(frame.get_pixel(0, 1), Some(gray));
    }

    #[test]
    fn chromatic_offset_splits_rgb_channels() {
        let mut source = Frame::new(3, 1);
        source.set_pixel(0, 0, Rgba::new(255, 0, 0, 255));
        source.set_pixel(1, 0, Rgba::new(255, 0, 0, 255));
        source.set_pixel(2, 0, Rgba::new(0, 0, 255, 255));

        let mut output = Frame::new(3, 1);
        apply_chromatic_offset(&source, &mut output, 1.0);

        assert_eq!(output.get_pixel(1, 0), Some(Rgba::new(0, 0, 0, 255)));
    }

    #[test]
    fn warp_shifts_pixels_horizontally() {
        let mut source = Frame::new(4, 2);
        for x in 0..4 {
            let color = if x % 2 == 0 {
                Rgba::black()
            } else {
                Rgba::white()
            };
            source.set_pixel(x, 0, color);
            source.set_pixel(x, 1, color);
        }

        let mut output = Frame::new(4, 2);
        apply_warp(&source, &mut output, 2.5, &ctx(0));

        assert_eq!(output.get_pixel(0, 0), source.get_pixel(0, 0));
        assert_ne!(output.get_pixel(0, 1), source.get_pixel(0, 1));
    }

    #[test]
    fn warp_changes_with_frame_index() {
        let mut source = Frame::new(4, 2);
        for x in 0..4 {
            let color = if x % 2 == 0 {
                Rgba::black()
            } else {
                Rgba::white()
            };
            source.set_pixel(x, 0, color);
            source.set_pixel(x, 1, color);
        }

        let mut at_frame_0 = Frame::new(4, 2);
        apply_warp(&source, &mut at_frame_0, 2.5, &ctx(0));

        let mut at_frame_5 = Frame::new(4, 2);
        apply_warp(&source, &mut at_frame_5, 2.5, &ctx(5));

        assert_ne!(at_frame_0.get_pixel(0, 1), at_frame_5.get_pixel(0, 1));
    }

    #[test]
    fn noise_is_deterministic_per_pixel() {
        assert_eq!(pixel_noise(&ctx(0), 3, 4), pixel_noise(&ctx(0), 3, 4));
        assert_ne!(pixel_noise(&ctx(0), 3, 4), pixel_noise(&ctx(1), 3, 4));
    }

    #[test]
    fn apply_noise_changes_pixels() {
        let gray = Rgba::new(128, 128, 128, 255);
        let mut frame = solid_frame(2, 2, gray);

        apply_noise(&mut frame, 0.5, &ctx(0));

        assert_ne!(frame.get_pixel(0, 0), Some(gray));
    }

    #[test]
    fn apply_changes_flat_frame() {
        let gray = Rgba::new(128, 128, 128, 255);
        let mut frame = solid_frame(8, 8, gray);
        let mut effect = VhsCrtEffect::new(0.5, 2.0, 0.2, 2.5);

        effect.apply(&mut frame, &ctx(0));

        assert_ne!(frame.get_pixel(0, 0), Some(gray));
        assert_ne!(frame.get_pixel(0, 1), frame.get_pixel(0, 0));
    }

    #[test]
    fn same_apply_is_deterministic() {
        let run = || {
            let gray = Rgba::new(128, 128, 128, 255);
            let mut frame = solid_frame(8, 8, gray);
            let mut effect = VhsCrtEffect::new(0.5, 2.0, 0.2, 2.5);
            effect.apply(&mut frame, &ctx(3));
            frame.get_pixel(4, 4)
        };

        assert_eq!(run(), run());
    }
}
