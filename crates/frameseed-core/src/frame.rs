use crate::color::{Rgba, lerp_rgba};
use image::RgbaImage;
use rayon::prelude::*;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Frame {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<Rgba>,
}

impl Frame {
    pub fn new(width: u32, height: u32) -> Self {
        let pixel_count = width * height;

        Self {
            width,
            height,
            pixels: vec![Rgba::black(); pixel_count as usize],
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: Rgba) -> bool {
        if x >= self.width || y >= self.height {
            return false;
        }
        let index = (y * self.width + x) as usize;
        self.pixels[index] = color;

        true
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> Option<Rgba> {
        if x >= self.width || y >= self.height {
            return None;
        }
        let index = (y * self.width + x) as usize;
        self.pixels.get(index).copied()
    }

    /// View the pixel buffer as raw RGBA bytes without copying.
    pub fn as_raw_rgba(&self) -> &[u8] {
        // Safety: Rgba is repr(Rust) with fields [r, g, b, a: u8] — no padding,
        // same size and alignment as [u8; 4]. The cast is valid.
        unsafe {
            std::slice::from_raw_parts(self.pixels.as_ptr() as *const u8, self.pixels.len() * 4)
        }
    }

    pub fn save_png(&self, path: &Path) -> Result<(), image::ImageError> {
        RgbaImage::from_raw(self.width, self.height, self.as_raw_rgba().to_vec())
            .expect("pixel buffer size mismatch")
            .save(path)?;
        Ok(())
    }

    pub fn fill_solid(&mut self, color: Rgba) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.set_pixel(x, y, color);
            }
        }
    }

    pub fn fill_vertical_gradient(&mut self, start_color: Rgba, end_color: Rgba) {
        for y in 0..self.height {
            for x in 0..self.width {
                let t = y as f32 / (self.height - 1) as f32;
                self.set_pixel(x, y, lerp_rgba(start_color, end_color, t));
            }
        }
    }

    pub fn fill_horizontal_gradient(&mut self, start_color: Rgba, end_color: Rgba) {
        for y in 0..self.height {
            for x in 0..self.width {
                let t = x as f32 / (self.width - 1) as f32;
                self.set_pixel(x, y, lerp_rgba(start_color, end_color, t));
            }
        }
    }

    pub fn fill_sliding_horizontal_gradient(
        &mut self,
        start_color: Rgba,
        end_color: Rgba,
        offset: f32,
    ) {
        for y in 0..self.height {
            for x in 0..self.width {
                let t = (x as f32 / (self.width - 1) as f32 + offset).fract();
                self.set_pixel(x, y, lerp_rgba(start_color, end_color, t));
            }
        }
    }

    pub fn fill_radial_gradient(&mut self, start_color: Rgba, end_color: Rgba) {
        let center_x = (self.width - 1) as f32 / 2.0;
        let center_y = (self.height - 1) as f32 / 2.0;
        let max_distance = (center_x * center_x + center_y * center_y).sqrt();

        for y in 0..self.height {
            for x in 0..self.width {
                let dx = x as f32 - center_x;
                let dy = y as f32 - center_y;

                let distance = (dx * dx + dy * dy).sqrt();

                let t = (distance / max_distance).min(1.0);
                self.set_pixel(x, y, lerp_rgba(start_color, end_color, t));
            }
        }
    }

    pub fn fill_sine_wave(&mut self, color: Rgba, time: f32, frequency: f32) {
        use std::f32::consts::TAU;
        let phase = TAU * time;

        for y in 0..self.height {
            for x in 0..self.width {
                let nx = x as f32 / (self.width - 1) as f32;
                let wave = (nx * frequency + phase).sin();
                let t = (wave + 1.0) / 2.0;
                self.set_pixel(x, y, lerp_rgba(color, Rgba::black(), t));
            }
        }
    }

    pub fn parallel_for_each_pixel<F>(&mut self, callback: F)
    where
        F: Fn(u32, u32) -> Rgba + Send + Sync,
    {
        let width = self.width;
        self.pixels
            .par_iter_mut()
            .enumerate()
            .for_each(|(index, pixel)| {
                let x = (index as u32) % width;
                let y = (index as u32) / width;
                *pixel = callback(x, y);
            });
    }

    pub fn clear(&mut self, color: Rgba) {
        self.pixels.fill(color);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let frame = Frame::new(100, 100);
        assert_eq!(frame.width, 100);
        assert_eq!(frame.height, 100);
        assert_eq!(frame.pixels.len(), 10_000)
    }

    #[test]
    fn test_set_pixel() {
        let mut frame = Frame::new(100, 100);
        frame.set_pixel(0, 0, Rgba::new(255, 0, 0, 255));
        assert_eq!(frame.get_pixel(0, 0), Some(Rgba::new(255, 0, 0, 255)));
    }

    #[test]
    fn multiple_set_pixel() {
        let mut frame = Frame::new(4, 4);
        frame.set_pixel(0, 0, Rgba::white());
        frame.set_pixel(3, 3, Rgba::new(255, 0, 0, 255));

        assert_eq!(frame.get_pixel(0, 0), Some(Rgba::new(255, 255, 255, 255)));
        assert_eq!(frame.get_pixel(3, 3), Some(Rgba::new(255, 0, 0, 255)));
        assert_eq!(frame.get_pixel(1, 1), Some(Rgba::new(0, 0, 0, 255)));
    }

    #[test]
    fn test_get_pixel() {
        let frame = Frame::new(100, 100);
        assert_eq!(frame.get_pixel(0, 0), Some(Rgba::black()));
    }

    #[test]
    fn out_bounds_return_none() {
        let frame = Frame::new(3, 2);
        assert_eq!(frame.get_pixel(4, 0), None);
        assert_eq!(frame.get_pixel(0, 3), None);
    }

    #[test]
    fn vertical_top_differs_from_bottom() {
        let mut frame = Frame::new(10, 10);
        frame.fill_vertical_gradient(Rgba::black(), Rgba::white());
        assert_ne!(frame.get_pixel(0, 0), frame.get_pixel(0, 9));
    }
    #[test]
    fn radial_center_differs_from_corner() {
        let mut frame = Frame::new(10, 10);
        frame.fill_radial_gradient(Rgba::black(), Rgba::white());
        assert_ne!(frame.get_pixel(5, 5), frame.get_pixel(0, 0));
    }

    #[test]
    fn parallel_for_each_pixel_matches_serial_loop() {
        let mut serial = Frame::new(16, 16);
        for y in 0..16 {
            for x in 0..16 {
                serial.set_pixel(x, y, Rgba::new((x * 16) as u8, (y * 16) as u8, 128, 255));
            }
        }

        let mut parallel = Frame::new(16, 16);
        parallel
            .parallel_for_each_pixel(|x, y| Rgba::new((x * 16) as u8, (y * 16) as u8, 128, 255));

        assert_eq!(serial.pixels, parallel.pixels);
    }

    #[test]
    fn clear_resets_all_pixels_without_reallocating() {
        let mut frame = Frame::new(4, 4);
        frame.set_pixel(0, 0, Rgba::white());
        let capacity = frame.pixels.capacity();
        frame.clear(Rgba::black());
        assert_eq!(frame.get_pixel(0, 0), Some(Rgba::black()));
        assert_eq!(frame.pixels.capacity(), capacity);
    }
}
