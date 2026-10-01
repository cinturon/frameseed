use crate::{Frame, RenderContext, Rgba, Scene};
use serde::{Deserialize, Serialize};
use std::f32::consts::TAU;

pub struct PlasmaScene {
    pub speed: f32,
    pub scale: f32,
}

impl PlasmaScene {
    pub fn new(speed: f32, scale: f32) -> Self {
        Self { speed, scale }
    }
}

impl Scene for PlasmaScene {
    fn name(&self) -> &str {
        "plasma"
    }

    fn render(&self, frame: &mut Frame, context: &RenderContext) {
        let t = context.normalized_time * self.speed * TAU;
        let scale = self.scale;
        let w = frame.width as f32;
        let h = frame.height as f32;

        frame.parallel_for_each_pixel(move |x, y| {
            let fx = x as f32 / w;
            let fy = y as f32 / h;

            let v1 = (fx * scale * TAU + t).sin();
            let v2 = (fy * scale * TAU + t).sin();
            let v3 = ((fx + fy) * scale * 0.5 * TAU + t * 0.7).sin();
            let cx = fx - 0.5 + (t * 0.3).sin() * 0.3;
            let cy = fy - 0.5 + (t * 0.3).cos() * 0.3;
            let v4 = (((cx * cx + cy * cy).sqrt()) * scale * TAU + t).sin();

            let v = (v1 + v2 + v3 + v4) * 0.25;
            let norm = (v + 1.0) * 0.5;

            let r = ((norm * TAU).sin() * 127.0 + 128.0) as u8;
            let g = ((norm * TAU + TAU / 3.0).sin() * 127.0 + 128.0) as u8;
            let b = ((norm * TAU + 2.0 * TAU / 3.0).sin() * 127.0 + 128.0) as u8;

            Rgba::new(r, g, b, 255)
        });
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct PlasmaParams {
    #[serde(default = "default_speed")]
    pub speed: f32,
    #[serde(default = "default_scale")]
    pub scale: f32,
}

impl Default for PlasmaParams {
    fn default() -> Self {
        Self {
            speed: 1.0,
            scale: 3.0,
        }
    }
}

fn default_speed() -> f32 {
    1.0
}

fn default_scale() -> f32 {
    3.0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn render_plasma(frame_index: u32) -> Frame {
        let mut frame = Frame::new(64, 64);
        let ctx = RenderContext::new(frame_index, 60, 24.0, 42);
        let scene = PlasmaScene::new(1.0, 3.0);
        scene.render(&mut frame, &ctx);
        frame
    }

    #[test]
    fn plasma_frame0_is_colored() {
        let frame = render_plasma(0);
        let p = frame.get_pixel(32, 32).unwrap();
        assert!(p.r > 0 || p.g > 0 || p.b > 0);
    }

    #[test]
    fn plasma_frame0_differs_from_frame30() {
        let f0 = render_plasma(0);
        let f30 = render_plasma(30);
        let any_diff = (0..64).any(|x| f0.get_pixel(x, 32) != f30.get_pixel(x, 32));
        assert!(any_diff);
    }
}
