use crate::{Frame, RenderContext, Rgba, Scene};
use serde::{Deserialize, Serialize};

pub struct GradientScene {
    pub start_color: Rgba,
    pub end_color: Rgba,
    pub speed: f32,
}

impl GradientScene {
    pub fn new(start_color: Rgba, end_color: Rgba, speed: f32) -> Self {
        Self {
            start_color,
            end_color,
            speed,
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct GradientParams {
    #[serde(default = "default_speed")]
    pub speed: f32,
    #[serde(default)]
    pub palette: String,
}

impl Default for GradientParams {
    fn default() -> Self {
        Self {
            speed: 1.0,
            palette: String::new(),
        }
    }
}

fn default_speed() -> f32 {
    1.0
}

impl Scene for GradientScene {
    fn name(&self) -> &str {
        "gradient"
    }

    fn render(&self, frame: &mut Frame, context: &RenderContext) {
        let offset = context.normalized_time * self.speed;
        frame.fill_sliding_horizontal_gradient(self.start_color, self.end_color, offset)
    }
}

pub fn palette_from_name(name: &str) -> (Rgba, Rgba) {
    match name {
        "sunset"   => (Rgba::new(255,  94,  77, 255), Rgba::new(255, 200,  87, 255)),
        "ocean"    => (Rgba::new(  0,  32,  96, 255), Rgba::new( 32, 178, 170, 255)),
        "forest"   => (Rgba::new( 10,  60,  10, 255), Rgba::new(144, 238, 144, 255)),
        "fire"     => (Rgba::new(255,  69,   0, 255), Rgba::new(255, 215,   0, 255)),
        "purple"   => (Rgba::new( 75,   0, 130, 255), Rgba::new(238, 130, 238, 255)),
        "ice"      => (Rgba::new(173, 216, 230, 255), Rgba::new(255, 255, 255, 255)),
        "rose"     => (Rgba::new(255,  20, 147, 255), Rgba::new(255, 182, 193, 255)),
        "midnight" => (Rgba::new(  0,   0,  50, 255), Rgba::new( 25,  25, 112, 255)),
        _          => (Rgba::black(), Rgba::white()),
    }
}

pub const KNOWN_PALETTES: &[&str] = &[
    "sunset", "ocean", "forest", "fire", "purple", "ice", "rose", "midnight",
];

#[cfg(test)]
mod tests {
    use super::*;

    // Render a frame with the sunset palette
    fn render_sunset_frame(
        width: u32,
        height: u32,
        frame_index: u32,
        total_frames: u32,
        speed: f32,
    ) -> Frame {
        let mut frame = Frame::new(width, height);
        let ctx = RenderContext::new(frame_index, total_frames, 24.0, 42);
        let (start_color, end_color) = palette_from_name("sunset");
        let scene = GradientScene::new(start_color, end_color, speed);
        scene.render(&mut frame, &ctx);
        frame
    }

    #[test]
    fn snapshot_frame0_left_edge_is_sunset_start() {
        let frame = render_sunset_frame(16, 16, 0, 120, 1.0);
        assert_eq!(frame.get_pixel(0, 0), Some(Rgba::new(255, 94, 77, 255)));
    }

    #[test]
    fn snapshot_frame0_mid_gradient_pixel() {
        let frame = render_sunset_frame(16, 16, 0, 120, 1.0);
        assert_eq!(frame.get_pixel(8, 0), Some(Rgba::new(255, 150, 82, 255)),);
    }
    #[test]
    fn snapshot_frame0_right_edge_wraps_to_start() {
        let frame = render_sunset_frame(64, 64, 0, 120, 1.0);
        // x = 63 → t = (63/63 + 0).fract() = 0.0, same as left edge
        assert_eq!(frame.get_pixel(63, 0), Some(Rgba::new(255, 94, 77, 255)),);
    }

    #[test]
    fn snapshot_mid_frame_differs_from_frame0() {
        let frame0 = render_sunset_frame(64, 64, 0, 120, 1.0);
        let frame60 = render_sunset_frame(64, 64, 60, 120, 1.0);

        assert_eq!(frame60.get_pixel(32, 0), Some(Rgba::new(255, 95, 77, 255)),);
        assert_ne!(frame0.get_pixel(32, 0), frame60.get_pixel(32, 0),);
    }
}
