use std::f32::consts::TAU;

use crate::{Frame, RenderContext, Rgba, Scene};
use serde::{Deserialize, Serialize};

pub struct SdfShapeScene {
    pub circle_radius: f32,
    pub box_half_width: f32,
    pub box_half_height: f32,
    pub speed: f32,
}

impl SdfShapeScene {
    pub fn new(circle_radius: f32, box_half_width: f32, box_half_height: f32, speed: f32) -> Self {
        Self {
            circle_radius,
            box_half_width,
            box_half_height,
            speed,
        }
    }
}

impl Scene for SdfShapeScene {
    fn name(&self) -> &str {
        "sdf_shapes"
    }

    fn render(&self, frame: &mut Frame, context: &RenderContext) {
        let time = context.normalized_time * self.speed;
        let edge = 1.5;
        let blend = 20.0;

        let center_x = frame.width as f32 * 0.5;
        let center_y = frame.height as f32 * 0.5;

        let circle_cx = center_x + (time * TAU).sin() * 30.0;
        let circle_cy = center_y;
        let circle_radius = self.circle_radius + 8.0 * (time * TAU * 2.0).sin();

        let box_cx = center_x + (time * TAU).sin() * 30.0;
        let box_cy = center_y;
        let box_half_height = self.box_half_height;
        let box_half_width = self.box_half_width;

        frame.parallel_for_each_pixel(move |x, y| {
            let px = x as f32 + 0.5;
            let py = y as f32 + 0.5;
            let circle_d = sdf_circle(px, py, circle_cx, circle_cy, circle_radius);
            let box_d = sdf_rectangle(px, py, box_cx, box_cy, box_half_width, box_half_height);
            let d = smin(circle_d, box_d, blend);
            let v = sdf_to_gray(d, edge);
            Rgba::new(v, v, v, 255)
        });
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct SdfShapeParams {
    #[serde(default = "default_circle_radius")]
    pub circle_radius: f32,
    #[serde(default = "default_box_half_width")]
    pub box_half_width: f32,
    #[serde(default = "default_box_half_height")]
    pub box_half_height: f32,
    #[serde(default = "default_speed")]
    pub speed: f32,
}

impl Default for SdfShapeParams {
    fn default() -> Self {
        Self {
            circle_radius: default_circle_radius(),
            box_half_width: default_box_half_width(),
            box_half_height: default_box_half_height(),
            speed: default_speed(),
        }
    }
}

fn default_circle_radius() -> f32 {
    return 40.0;
}

fn default_box_half_width() -> f32 {
    return 20.0;
}

fn default_box_half_height() -> f32 {
    return 10.0;
}

fn default_speed() -> f32 {
    return 1.0;
}

fn sdf_circle(px: f32, py: f32, cx: f32, cy: f32, radius: f32) -> f32 {
    let dx = px - cx;
    let dy = py - cy;
    (dx * dx + dy * dy).sqrt() - radius
}

fn sdf_rectangle(px: f32, py: f32, cx: f32, cy: f32, hx: f32, hy: f32) -> f32 {
    let dx = (px - cx).abs() - hx;
    let dy = (py - cy).abs() - hy;

    let outside = dx.max(dy).max(0.0);
    let inside = (dx.max(0.0).powi(2) + dy.max(0.0).powi(2)).sqrt();
    outside + inside
}

// Smooth minimum function
fn smin(a: f32, b: f32, k: f32) -> f32 {
    let h = (0.5 + 0.5 * (b - a) / k).clamp(0.0, 1.0);
    a * h + b * (1.0 - h) - k * h * (1.0 - h)
}

fn sdf_to_gray(d: f32, edge: f32) -> u8 {
    let t = 1.0 - (d / edge).clamp(0.0, 1.0);
    (t * 255.0) as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    fn render_sdf_frame(
        width: u32,
        height: u32,
        frame_index: u32,
        circle_radius: f32,
        box_half_width: f32,
        box_half_height: f32,
        speed: f32,
    ) -> Frame {
        let mut frame = Frame::new(width, height);
        let context = RenderContext::new(frame_index, 120, 24.0, 42);
        let scene = SdfShapeScene::new(circle_radius, box_half_width, box_half_height, speed);
        scene.render(&mut frame, &context);
        frame
    }

    #[test]
    fn sdf_to_gray_is_bright_inside_and_dark_outside() {
        assert_eq!(sdf_to_gray(-1.0, 1.5), 255);
        assert_eq!(sdf_to_gray(0.0, 1.5), 255);
        assert_eq!(sdf_to_gray(1.5, 1.5), 0);
    }

    #[test]
    fn sdf_circle_center_is_inside() {
        assert!(sdf_circle(32.0, 32.0, 32.0, 32.0, 20.0) < 0.0);
    }

    #[test]
    fn center_pixel_is_bright_at_frame_zero() {
        let frame = render_sdf_frame(64, 64, 0, 40.0, 20.0, 10.0, 1.0);
        let center = frame.get_pixel(32, 32).unwrap();
        assert!(center.r > 200);
    }

    #[test]
    fn corner_pixel_is_dark_at_frame_zero() {
        let frame = render_sdf_frame(64, 64, 0, 40.0, 20.0, 10.0, 1.0);
        assert_eq!(frame.get_pixel(0, 0), Some(Rgba::black()));
    }

    #[test]
    fn same_frame_is_deterministic() {
        let a = render_sdf_frame(128, 64, 0, 40.0, 20.0, 10.0, 1.0);
        let b = render_sdf_frame(128, 64, 0, 40.0, 20.0, 10.0, 1.0);
        assert_eq!(a.get_pixel(64, 32), b.get_pixel(64, 32));
    }

    #[test]
    fn later_frames_differ_from_frame_zero() {
        let frame0 = render_sdf_frame(128, 64, 0, 40.0, 20.0, 10.0, 1.0);
        // Radius pulse minimum (~32) occurs late in the clip; inside at r=40, outside at r=32
        let frame89 = render_sdf_frame(128, 64, 89, 40.0, 20.0, 10.0, 1.0);
        assert_ne!(frame0.get_pixel(99, 32), frame89.get_pixel(99, 32));
    }
}
