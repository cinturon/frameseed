use crate::{Frame, RenderContext, Effect, Rgba};
use serde::{Deserialize, Serialize};


pub struct PaletteQuantizationEffect {
    pub palette: Vec<Rgba>,
}

impl PaletteQuantizationEffect {
    pub fn new(palette: &[Rgba]) -> Self {
        Self { palette: palette.to_vec() }
    }

    pub fn from_name(name: &str) -> Self {
        Self::new(&palette_from_name(name)) }
}

impl Effect for PaletteQuantizationEffect {
    fn name(&self) -> &str {
        "palette"
    }

    fn apply(&mut self, frame: &mut Frame, _context: &RenderContext) {
        let palette = self.palette.clone();
        let source_frame = frame.pixels.clone();
        let width = frame.width;

        frame.parallel_for_each_pixel(move |x, y| {
            let index = (y * width + x) as usize;
            let pixel = source_frame[index];
            nearest_palette_color(pixel, &palette)
        });
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct PaletteQuantizationParams {
    #[serde(default = "default_palette_name")]
    pub name: String,
}

impl Default for PaletteQuantizationParams {
    fn default() -> Self {
        Self { name: default_palette_name() }
    }
}

fn default_palette_name() -> String {
    "cga16".to_string()
}

fn nearest_palette_color(pixel: Rgba, palette: &[Rgba]) -> Rgba {
    let mut best = palette[0];
    let mut best_distance_squared = color_distance_squared(pixel, best);
    
    for &color in palette {
        let distance = color_distance_squared(pixel, color);
        if distance < best_distance_squared {
            best = color;
            best_distance_squared = distance;
        }
    }
    best
}

fn color_distance_squared(a: Rgba, b: Rgba) -> f32 {
    let r = a.r as f32 - b.r as f32;
    let g = a.g as f32 - b.g as f32;
    let b = a.b as f32 - b.b as f32;
    r * r + g * g + b * b
}

fn palette_from_name(name: &str) -> Vec<Rgba> {
    match name {
        "cga16" => cga_16_palette(),
        "gameboy" => gameboy_4_palette(),
        _ => cga_16_palette(),
    }
}

fn cga_16_palette() -> Vec<Rgba> {
    vec![
        Rgba::new(0, 0, 0, 255),
        Rgba::new(255, 0, 0, 255),
        Rgba::new(0, 255, 0, 255),
        Rgba::new(0, 0, 255, 255),
        Rgba::new(255, 255, 0, 255),
        Rgba::new(0, 255, 255, 255),
        Rgba::new(255, 0, 255, 255),
        Rgba::new(255, 255, 255, 255),
        Rgba::new(128, 128, 128, 255),
        Rgba::new(128, 0, 0, 255),
        Rgba::new(0, 128, 0, 255),
        Rgba::new(0, 0, 128, 255),
        Rgba::new(128, 128, 0, 255),
        Rgba::new(0, 128, 128, 255),
        Rgba::new(128, 0, 128, 255),
        Rgba::new(0, 0, 170, 255),
    ]
}

fn gameboy_4_palette() -> Vec<Rgba> {
    vec![
        Rgba::new(255, 255, 255, 255),
        Rgba::new(170, 170, 170, 255),
        Rgba::new(85, 85, 85, 255),
        Rgba::new(0, 0, 0, 255),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palette_from_name() {
        assert_eq!(palette_from_name("cga16"), cga_16_palette());
        assert_eq!(palette_from_name("gameboy"), gameboy_4_palette());
    }

    #[test]
    fn color_distance_squared_is_zero_for_same_color() {
        let red = Rgba::new(255, 0, 0, 255);
        assert_eq!(color_distance_squared(red, red), 0.0);
    }

    #[test]
    fn nearest_palette_color_picks_closest_by_distance() {
        let palette = vec![
            Rgba::new(255, 0, 0, 255),
            Rgba::new(0, 0, 255, 255),
        ];
        let orange = Rgba::new(200, 100, 50, 255);
        assert_eq!(nearest_palette_color(orange, &palette), Rgba::new(255, 0, 0, 255));
    }

    #[test]
    fn test_nearest_palette_color() {
        let palette = cga_16_palette();
        let pixel = Rgba::new(255, 0, 0, 255);
        assert_eq!(nearest_palette_color(pixel, &palette), Rgba::new(255, 0, 0, 255));
    }

    #[test]
    fn test_palette_quantization_effect() {
        let mut frame = Frame::new(100, 100);
        frame.set_pixel(0, 0, Rgba::new(255, 0, 0, 255));
        let context = RenderContext::new(0, 120, 24.0, 42);
        PaletteQuantizationEffect::from_name("cga16").apply(&mut frame, &context);
        assert_eq!(frame.get_pixel(0, 0), Some(Rgba::new(255, 0, 0, 255)));
    }
}