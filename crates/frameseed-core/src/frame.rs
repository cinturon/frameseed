use crate::color::Rgba;

#[derive(Debug, Clone)]
pub struct Frame {
    width: u32,
    height: u32,
    pixels: Vec<Rgba>,
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
}
