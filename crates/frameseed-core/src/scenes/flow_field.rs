use crate::Rgba;
use crate::seeded_rng;
use crate::value_noise_2d;
use crate::{Frame, RenderContext, Scene};
use rand::Rng;
use serde::Deserialize;
use std::f32::consts::TAU;

pub struct FlowFieldParticle {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
}

pub struct FlowFieldScene {
    pub count: u32,
    pub speed: f32,
    pub scale: f32,
    pub fps: f32,
}

impl FlowFieldScene {
    pub fn new(count: u32, speed: f32, scale: f32, fps: f32) -> Self {
        Self {
            count,
            speed,
            scale,
            fps,
        }
    }
}

impl Scene for FlowFieldScene {
    fn name(&self) -> &str {
        "flow_field"
    }

    fn render(&self, frame: &mut Frame, context: &RenderContext) {
        let delta_time = 1.0 / self.fps;

        //1. Create particles
        let mut particles =
            spawn_flow_particles(self.count, frame.width, frame.height, context.seed);

        //2. Update particles
        for _ in 0..context.frame_index {
            update_flow_particles(
                &mut particles,
                delta_time,
                frame.width,
                frame.height,
                context.seed,
                self.speed,
                self.scale,
            );
        }

        //3. Draw particles
        draw_particles(&particles, frame);
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct FlowFieldParams {
    #[serde(default = "default_count")]
    pub count: u32,
    #[serde(default = "default_scale")]
    pub scale: f32,
    #[serde(default = "default_fps")]
    pub fps: f32,
    #[serde(default = "default_speed")]
    pub speed: f32,
}

impl Default for FlowFieldParams {
    fn default() -> Self {
        Self {
            count: default_count(),
            scale: default_scale(),
            fps: default_fps(),
            speed: default_speed(),
        }
    }
}

fn default_count() -> u32 {
    return 500;
}

fn default_scale() -> f32 {
    return 0.02;
}

fn default_fps() -> f32 {
    return 24.0;
}

fn default_speed() -> f32 {
    return 40.0;
}

fn flow_angle_at(x: f32, y: f32, scale: f32, seed: u64) -> f32 {
    value_noise_2d(x * scale, y * scale, seed) * TAU
}

fn draw_particles(particles: &[FlowFieldParticle], frame: &mut Frame) {
    for particle in particles {
        let x = particle.x as u32;
        let y = particle.y as u32;

        if x < frame.width && y < frame.height {
            frame.set_pixel(x, y, Rgba::white());
        }
    }
}

fn wrap_particle(particle: &mut FlowFieldParticle, width: u32, height: u32) {
    if particle.x < 0.0 {
        particle.x += width as f32;
    }
    if particle.x >= width as f32 {
        particle.x -= width as f32;
    }
    if particle.y < 0.0 {
        particle.y += height as f32;
    }
    if particle.y >= height as f32 {
        particle.y -= height as f32;
    }
}

fn update_flow_particles(
    particles: &mut [FlowFieldParticle],
    delta_time: f32,
    width: u32,
    height: u32,
    seed: u64,
    speed: f32,
    scale: f32,
) {
    for particle in particles.iter_mut() {
        let angle = flow_angle_at(particle.x, particle.y, scale, seed);
        particle.vx = angle.cos() * speed;
        particle.vy = angle.sin() * speed;

        particle.x += particle.vx * delta_time;
        particle.y += particle.vy * delta_time;

        wrap_particle(particle, width, height);
    }
}

fn spawn_flow_particles(count: u32, width: u32, height: u32, seed: u64) -> Vec<FlowFieldParticle> {
    let mut rng = seeded_rng(seed);

    let mut particles = Vec::with_capacity(count as usize);
    for _ in 0..count {
        particles.push(FlowFieldParticle {
            x: rng.random::<f32>() * width as f32,
            y: rng.random::<f32>() * height as f32,
            vx: 0.0,
            vy: 0.0,
        });
    }
    particles
}


#[cfg(test)]
mod tests {
    use super::*;

    fn render_flow_field_frame(width: u32, height: u32, frame_index: u32, seed: u64, speed: f32, scale: f32) -> Frame {
        let mut frame = Frame::new(width, height);
        let context = RenderContext::new(frame_index, 120, 24.0, seed);
        let scene = FlowFieldScene::new(500, speed, scale, 24.0);
        scene.render(&mut frame, &context);
        frame
    }

    #[test]
    fn test_frame0_and_from0_are_the_same() {
        let frame0 = render_flow_field_frame(256, 64, 0, 42, 40.0, 0.02);
        let frame1 = render_flow_field_frame(256, 64, 0, 42, 40.0, 0.02);
        assert_eq!(frame0.get_pixel(0, 0), frame1.get_pixel(0, 0));
    }

    #[test]
    fn later_frames_differ_from_frame_zero() {
        let frame0 = render_flow_field_frame(256, 64, 0, 42, 40.0, 0.02);
        let frame42 = render_flow_field_frame(256, 64, 42, 42, 40.0, 0.02);
        assert_ne!(frame0.get_pixel(50, 0), frame42.get_pixel(50, 0));
    }
}