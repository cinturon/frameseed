use crate::{Effect, Frame, RenderContext, Rgba};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

pub struct BoxBlurEffect {
    pub radius: u32,
}

impl BoxBlurEffect {
    pub fn new(radius: u32) -> Self {
        Self {
            radius: radius.clamp(1, 64),
        }
    }
}

impl Effect for BoxBlurEffect {
    fn name(&self) -> &str {
        "blur"
    }

    fn apply(&mut self, frame: &mut Frame, _context: &RenderContext) {
        let r = self.radius as i32;
        let w = frame.width as i32;
        let h = frame.height as i32;
        // Clone once so every parallel row reads from the original pixels.
        let src = frame.pixels.clone();

        frame
            .pixels
            .par_chunks_mut(frame.width as usize)
            .enumerate()
            .for_each(|(y, row)| {
                let y = y as i32;
                for x in 0..w {
                    let mut sum_r = 0u32;
                    let mut sum_g = 0u32;
                    let mut sum_b = 0u32;
                    let mut count = 0u32;
                    for ky in (y - r)..=(y + r) {
                        for kx in (x - r)..=(x + r) {
                            if kx >= 0 && kx < w && ky >= 0 && ky < h {
                                let p = src[(ky * w + kx) as usize];
                                sum_r += p.r as u32;
                                sum_g += p.g as u32;
                                sum_b += p.b as u32;
                                count += 1;
                            }
                        }
                    }
                    row[x as usize] = Rgba::new(
                        (sum_r / count) as u8,
                        (sum_g / count) as u8,
                        (sum_b / count) as u8,
                        255,
                    );
                }
            });
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct BlurParams {
    #[serde(default = "default_radius")]
    pub radius: u32,
}

impl Default for BlurParams {
    fn default() -> Self {
        Self {
            radius: default_radius(),
        }
    }
}

fn default_radius() -> u32 {
    2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blur_averages_a_bright_pixel_into_neighbors() {
        let mut frame = Frame::new(5, 5);
        frame.set_pixel(2, 2, Rgba::new(255, 255, 255, 255));
        let ctx = RenderContext::new(0, 1, 24.0, 1);
        BoxBlurEffect::new(1).apply(&mut frame, &ctx);
        // center should be less than 255 (averaged with black neighbors)
        let center = frame.get_pixel(2, 2).unwrap();
        assert!(center.r < 255);
    }

    #[test]
    fn blur_solid_color_is_unchanged() {
        let mut frame = Frame::new(8, 8);
        for y in 0..8 {
            for x in 0..8 {
                frame.set_pixel(x, y, Rgba::new(100, 150, 200, 255));
            }
        }
        let ctx = RenderContext::new(0, 1, 24.0, 1);
        BoxBlurEffect::new(2).apply(&mut frame, &ctx);
        // solid color should survive a blur exactly
        assert_eq!(frame.get_pixel(4, 4), Some(Rgba::new(100, 150, 200, 255)));
    }

    #[test]
    fn radius_zero_clamps_to_one() {
        let e = BoxBlurEffect::new(0);
        assert_eq!(e.radius, 1);
    }
}
