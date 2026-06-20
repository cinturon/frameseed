use crate::{Frame, RenderContext, Scene, Rgba};
use serde::{Deserialize, Serialize};

pub struct MandelbrotScene {
    pub max_iter: u32,
    pub center_re: f32,
    pub center_im: f32,
    pub initial_view_width: f32,
    pub zoom_speed: f32,
}

impl MandelbrotScene {
    pub fn new(max_iter: u32, center_re: f32, center_im: f32, initial_view_width: f32, zoom_speed: f32) -> Self {
        Self { max_iter, center_re, center_im, initial_view_width, zoom_speed }
    }
}

impl Scene for MandelbrotScene {
    fn name(&self) -> &str {
        "mandelbrot"
    }

    fn render(&self, frame: &mut Frame, context: &RenderContext) {
        let time = context.normalized_time * self.zoom_speed;
        let view_width = self.initial_view_width * (0.5_f32.powf(time));
        let scale = view_width / frame.width as f32;

        let half_width = frame.width as f32 * 0.5;
        let half_height = frame.height as f32 * 0.5;

        let max_iter = self.max_iter;
        let center_re = self.center_re;
        let center_im = self.center_im;
        
        frame.parallel_for_each_pixel(move |x, y| {
            let c_re = center_re + (x as f32 - half_width) * scale;
            let c_im = center_im + (y as f32 - half_height) * scale;
            let count = mandelbrot_iteration(c_re, c_im, max_iter);
            iteration_to_color(count, max_iter)
        });
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct MandelbrotParams {
    #[serde(default = "default_max_iter")]
    pub max_iter: u32,
    #[serde(default = "default_center_re")]
    pub center_re: f32,
    #[serde(default = "default_center_im")]
    pub center_im: f32,
    #[serde(default = "default_initial_view_width")]
    pub initial_view_width: f32,
    #[serde(default = "default_zoom_speed")]
    pub zoom_speed: f32,
}

impl Default for MandelbrotParams {
    fn default() -> Self {
        Self {
            max_iter: default_max_iter(),
            center_re: default_center_re(),
            center_im: default_center_im(),
            initial_view_width: default_initial_view_width(),
            zoom_speed: default_zoom_speed(),
        }
    }
}

fn default_max_iter() -> u32 {
    return 100;
}

fn default_center_re() -> f32 {
    return -0.5;
}

fn default_center_im() -> f32 {
    return 0.0;
}

fn default_initial_view_width() -> f32 {
    return 1.0;
}

fn default_zoom_speed() -> f32 {
    return 1.0;
}   

fn mandelbrot_iteration(c_re: f32, c_im: f32, max_iter: u32) -> u32 {
    let mut z_re = 0.0;
    let mut z_im = 0.0;
    let bailout_sq = 4.0;

    for i in 0..max_iter {
        let z_re_squared = z_re * z_re;
        let z_im_squared = z_im * z_im;
        if z_re_squared + z_im_squared > bailout_sq {
            return i;
        }

        let new_z_re = z_re_squared - z_im_squared + c_re;
        let new_z_im = 2.0 * z_re * z_im + c_im;
        z_re = new_z_re;
        z_im = new_z_im;
    }
    max_iter
}

fn iteration_to_color(count: u32, max_iter: u32) -> Rgba {
    if count >= max_iter {
        return Rgba::black();
    }

    let t = count as f32 / max_iter as f32;
    let hue = (t * 255.0) as u8;
    Rgba::new(hue, hue, hue, 255)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn render_mandelbrot_frame(
        width: u32,
        height: u32,
        frame_index: u32,
        max_iter: u32,
        center_re: f32,
        center_im: f32,
        initial_view_width: f32,
        zoom_speed: f32,
    ) -> Frame {
        let mut frame = Frame::new(width, height);
        let context = RenderContext::new(frame_index, 120, 24.0, 42);
        let scene =
            MandelbrotScene::new(max_iter, center_re, center_im, initial_view_width, zoom_speed);
        scene.render(&mut frame, &context);
        frame
    }

    fn render_mandelbrot_serial(
        width: u32,
        height: u32,
        frame_index: u32,
        max_iter: u32,
        center_re: f32,
        center_im: f32,
        initial_view_width: f32,
        zoom_speed: f32,
    ) -> Frame {
        let mut frame = Frame::new(width, height);
        let context = RenderContext::new(frame_index, 120, 24.0, 42);
        let time = context.normalized_time * zoom_speed;
        let view_width = initial_view_width * (0.5_f32.powf(time));
        let scale = view_width / frame.width as f32;
        let half_width = frame.width as f32 * 0.5;
        let half_height = frame.height as f32 * 0.5;

        for y in 0..frame.height {
            for x in 0..frame.width {
                let c_re = center_re + (x as f32 - half_width) * scale;
                let c_im = center_im + (y as f32 - half_height) * scale;
                let count = mandelbrot_iteration(c_re, c_im, max_iter);
                frame.set_pixel(x, y, iteration_to_color(count, max_iter));
            }
        }

        frame
    }

    #[test]
    fn origin_is_inside_set() {
        assert_eq!(mandelbrot_iteration(0.0, 0.0, 100), 100);
    }

    #[test]
    fn outside_point_escapes_quickly() {
        assert!(mandelbrot_iteration(2.0, 2.0, 100) < 5);
    }

    #[test]
    fn view_width_shrinks_over_time() {
        let initial = 3.0;
        let width_at_start = initial * 0.5_f32.powf(0.0);
        let width_at_end = initial * 0.5_f32.powf(1.0);
        assert!(width_at_end < width_at_start);
    }

    #[test]
    fn same_frame_is_deterministic() {
        let frame0a = render_mandelbrot_frame(100, 100, 0, 100, -0.5, 0.0, 3.0, 1.0);
        let frame0b = render_mandelbrot_frame(100, 100, 0, 100, -0.5, 0.0, 3.0, 1.0);
        assert_eq!(frame0a.get_pixel(99, 50), frame0b.get_pixel(99, 50));
    }

    #[test]
    fn parallel_render_matches_serial() {
        let serial = render_mandelbrot_serial(64, 64, 42, 100, -0.5, 0.0, 3.0, 1.0);
        let parallel = render_mandelbrot_frame(64, 64, 42, 100, -0.5, 0.0, 3.0, 1.0);
        assert_eq!(serial.pixels, parallel.pixels);
    }

    #[test]
    fn later_frames_differ_from_frame_zero() {
        let frame0 = render_mandelbrot_frame(100, 100, 0, 100, -0.5, 0.0, 3.0, 1.0);
        let frame100 = render_mandelbrot_frame(100, 100, 100, 100, -0.5, 0.0, 3.0, 1.0);
        // Positive-real fringe: escape count rises as the view zooms in.
        assert_ne!(frame0.get_pixel(99, 50), frame100.get_pixel(99, 50));
    }
}