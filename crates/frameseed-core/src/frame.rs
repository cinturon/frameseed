use crate::color::{Rgba, lerp_rgba};
use image::{RgbaImage};
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

    pub fn save_png(&self, path: &Path) -> Result<(), image::ImageError> {
        let mut image = RgbaImage::new(self.width, self.height);

        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(color) = self.get_pixel(x, y) {
                    image.put_pixel(x, y, image::Rgba([color.r, color.g, color.b, color.a]));
                }
            }
        }

        image.save(path)?;
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
}
