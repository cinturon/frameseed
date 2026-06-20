use crate::seeded_rng;
use crate::{Frame, RenderContext, Rgba, Scene};
use rand::Rng;
use serde::{Deserialize, Serialize};

pub struct Particle {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub lifetime: f32,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ParticleKind {
    Snow,
    Rain,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct ParticlesScene {
    pub count: u32,
    pub speed: f32,
    pub kind: ParticleKind,
    pub fps: f32,
}

impl ParticlesScene {
    pub fn new(count: u32, speed: f32, kind: ParticleKind, fps: f32) -> Self {
        Self { count, speed, kind, fps }
    }
}

impl Scene for ParticlesScene {
    fn name(&self) -> &str {
        "particles"
    }

    fn render(&self, frame: &mut Frame, context: &RenderContext) {
        let delta_time = 1.0 / self.fps;

        let mut particles = spawn_particles(
            self.count,
            frame.width,
            frame.height,
            context.seed,
            self.speed,
            &self.kind,
        );

        for _ in 0..context.frame_index{
            update_particles(&mut particles, delta_time, frame.width, frame.height, context.seed, self.speed, &self.kind);
        }
        draw_particles(&particles, frame);
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct ParticleParams {
    #[serde(default = "default_count")]
    pub count: u32,
    #[serde(default = "default_speed")]
    pub speed: f32,
    #[serde(default = "default_kind")]
    pub kind: ParticleKind,
    #[serde(default = "default_fps")]
    pub fps: f32,
}

impl Default for ParticleParams {
    fn default() -> Self {
        Self {
            count: default_count(),
            speed: default_speed(),
            kind: default_kind(),
            fps: default_fps(),
        }
    }
}

fn default_count() -> u32 {
    100
}

fn default_speed() -> f32 {
    1.0
}

fn default_kind() -> ParticleKind {
    ParticleKind::Snow
}

fn default_fps() -> f32 {
    24.0
}

fn draw_particles(particles: &[Particle], frame: &mut Frame) {
    for particle in particles {
        let x = particle.x as u32;
        let y = particle.y as u32;

        if x < frame.width && y < frame.height {
            frame.set_pixel(x, y, Rgba::white());
        }
    }
}

fn spawn_particles(
    count: u32,
    width: u32,
    height: u32,
    seed: u64,
    speed: f32,
    kind: &ParticleKind,
) -> Vec<Particle> {
    let mut particles = Vec::with_capacity(count as usize);

    for i in 0..count {
        let mut particle = Particle {
            x: 0.0,
            y: 0.0,
            vx: 0.0,
            vy: 0.0,
            lifetime: 0.0,
        };
        respawn_particle(&mut particle, i as usize, width, height, seed, speed, kind);
        particles.push(particle);
    }
    particles
}

fn respawn_particle(
    particle: &mut Particle,
    index: usize,
    width: u32,
    _height: u32,
    seed: u64,
    speed: f32,
    kind: &ParticleKind,
) {
    let mut rng = seeded_rng(seed.wrapping_add(index as u64));

    particle.x = rng.random::<f32>() * width as f32;
    particle.y = 0.0;

    match kind {
        ParticleKind::Snow => {
            particle.vx = (rng.random::<f32>() - 0.5) * 20.0 * speed;
            particle.vy = (30.0 + rng.random::<f32>() * 40.0) * speed;
        }
        ParticleKind::Rain => {
            particle.vx = (rng.random::<f32>() - 0.5) * 5.0 * speed;
            particle.vy = (80.0 + rng.random::<f32>() * 60.0) * speed;
        }
    }

    particle.lifetime = 2.0 + rng.random::<f32>() * 4.0;
}

fn update_particles(
    particles: &mut [Particle],
    delta_time: f32,
    width: u32,
    height: u32,
    seed: u64,
    speed: f32,
    kind: &ParticleKind,
) {
    for (index, particle) in particles.iter_mut().enumerate() {
        particle.x += particle.vx * delta_time;
        particle.y += particle.vy * delta_time;

        particle.lifetime -= delta_time;

        let off_screen =
            particle.y > height as f32 || particle.x < 0.0 || particle.x > width as f32;

        if particle.lifetime <= 0.0 || off_screen {
            respawn_particle(particle, index, width, height, seed, speed, kind);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn render_particles_frame(
        width: u32,
        height: u32,
        frame_index: u32,
        seed: u64,
        speed: f32,
        kind: ParticleKind,
    ) -> Frame {
        let mut frame = Frame::new(width, height);
        let context = RenderContext::new(frame_index, 120, 24.0, seed);
        let scene = ParticlesScene::new(100, speed, kind, 24.0);
        scene.render(&mut frame, &context);
        frame
    }

    #[test]
    fn test_frame0_differs_from_frame1() {
        let frame0 = render_particles_frame(100, 100, 0, 42, 1.0, ParticleKind::Snow);
        let frame1 = render_particles_frame(100, 100, 1, 42, 1.0, ParticleKind::Snow);
        assert_ne!(frame0.get_pixel(0, 0), frame1.get_pixel(0, 0));
    }

    #[test]
    fn test_frame0_and_from0_are_the_same() {
        let frame0 = render_particles_frame(100, 100, 0, 42, 1.0, ParticleKind::Snow);
        let frame1 = render_particles_frame(100, 100, 0, 42, 1.0, ParticleKind::Snow);
        assert_eq!(frame0.get_pixel(0, 0), frame1.get_pixel(0, 0));
    }
}