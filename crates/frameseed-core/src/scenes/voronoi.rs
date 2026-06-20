use crate::seeded_rng;
use crate::{Frame, RenderContext, Rgba, Scene};
use rand::Rng;
use serde::Deserialize;
use std::f32::consts::TAU;

struct VoronoiSeed {
    x: f32,
    y: f32,
    phase: f32,
    hue: u8,
}

pub struct VoronoiScene {
    seed_count: u32,
    speed: f32,
    edge_width: f32,
}

impl VoronoiScene {
    pub fn new(seed_count: u32, speed: f32, edge_width: f32) -> Self {
        Self {
            seed_count,
            speed,
            edge_width,
        }
    }
}

impl Scene for VoronoiScene {
    fn name(&self) -> &str {
        "voronoi"
    }

    fn render(&self, frame: &mut Frame, context: &RenderContext) {
        let time = context.normalized_time * self.speed;

        let mut seeds = spawn_seeds(self.seed_count, frame.width, frame.height, context.seed);

        update_seeds(&mut seeds, time, self.speed, frame.width, frame.height);

        frame.parallel_for_each_pixel(move |x, y| {
            let (index, distance_squared) = nearest_seed_index(&seeds, x as f32, y as f32);
            color_for_cell(seeds[index].hue, distance_squared, self.edge_width)
        });
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct VoronoiParams {
    #[serde(default = "default_seed_count")]
    pub seed_count: u32,
    #[serde(default = "default_speed")]
    pub speed: f32,
    #[serde(default = "default_edge_width")]
    pub edge_width: f32,
}

impl Default for VoronoiParams {
    fn default() -> Self {
        Self {
            seed_count: default_seed_count(),
            speed: default_speed(),
            edge_width: default_edge_width(),
        }
    }
}

fn default_seed_count() -> u32 {
    return 12;
}

fn default_speed() -> f32 {
    return 1.0;
}

fn default_edge_width() -> f32 {
    return 2.0;
}

fn spawn_seeds(seed_count: u32, width: u32, height: u32, seed: u64) -> Vec<VoronoiSeed> {
    let mut rng = seeded_rng(seed);
    let mut seeds = Vec::with_capacity(seed_count as usize);

    for _ in 0..seed_count {
        let x = rng.random::<f32>() * width as f32;
        let y = rng.random::<f32>() * height as f32;
        let phase = rng.random::<f32>() * TAU;
        let hue = (rng.random::<f32>() * 255.0) as u8;
        seeds.push(VoronoiSeed { x, y, phase, hue });
    }

    seeds
}

fn update_seeds(seeds: &mut [VoronoiSeed], time: f32, speed: f32, width: u32, height: u32) {
    let radius = speed * width.min(height) as f32 * 0.08;

    for seed in seeds.iter_mut() {
        let anchor_x = seed.x;
        let anchor_y = seed.y;
        let angle = time * TAU + seed.phase;

        seed.x = anchor_x + radius * angle.cos();
        seed.y = anchor_y + radius * angle.sin();
    }
}

fn nearest_seed_index(seeds: &[VoronoiSeed], x: f32, y: f32) -> (usize, f32) {
    let mut min_distance_squared = f32::MAX;
    let mut min_index = 0;
    for (index, seed) in seeds.iter().enumerate() {
        let distance = (seed.x - x).powf(2.0) + (seed.y - y).powf(2.0);
        if distance < min_distance_squared {
            min_distance_squared = distance;
            min_index = index;
        }
    }
    (min_index, min_distance_squared)
}

fn color_for_cell(hue: u8, distance_squared: f32, edge_width: f32) -> Rgba {
    let fill = Rgba::new(hue, hue.wrapping_add(85), hue.wrapping_add(170), 255);

    let distance = distance_squared.sqrt();

    if distance < edge_width {
        Rgba::black()
    } else {
        fill
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn render_voronoi_frame(
        width: u32,
        height: u32,
        frame_index: u32,
        seed: u64,
        seed_count: u32,
        speed: f32,
        edge_width: f32,
    ) -> Frame {
        let mut frame = Frame::new(width, height);
        let context = RenderContext::new(frame_index, 120, 24.0, seed);
        let scene = VoronoiScene::new(seed_count, speed, edge_width);
        scene.render(&mut frame, &context);
        frame
    }

    #[test]
    fn nearest_seed_picks_closest() {
        let seeds = vec![
            VoronoiSeed {
                x: 0.0,
                y: 0.0,
                phase: 0.0,
                hue: 10,
            },
            VoronoiSeed {
                x: 100.0,
                y: 0.0,
                phase: 0.0,
                hue: 20,
            },
        ];

        let (idx, dist_sq) = nearest_seed_index(&seeds, 10.0, 0.0);
        assert_eq!(idx, 0);
        assert_eq!(dist_sq, 100.0);

        let (idx, dist_sq) = nearest_seed_index(&seeds, 90.0, 0.0);
        assert_eq!(idx, 1);
        assert_eq!(dist_sq, 100.0);
    }

    #[test]
    fn phase_offsets_orbit_position_at_same_time() {
        let mut seed_a = VoronoiSeed {
            x: 50.0,
            y: 50.0,
            phase: 0.0,
            hue: 0,
        };
        let mut seed_b = VoronoiSeed {
            x: 50.0,
            y: 50.0,
            phase: TAU / 4.0,
            hue: 0,
        };

        update_seeds(std::slice::from_mut(&mut seed_a), 0.0, 1.0, 100, 100);
        update_seeds(std::slice::from_mut(&mut seed_b), 0.0, 1.0, 100, 100);

        assert_ne!(seed_a.x, seed_b.x);
        assert_ne!(seed_a.y, seed_b.y);
    }

    #[test]
    fn orbit_moves_seed_over_time() {
        let mut at_start = VoronoiSeed {
            x: 50.0,
            y: 50.0,
            phase: 1.0,
            hue: 0,
        };
        let mut later = VoronoiSeed {
            x: 50.0,
            y: 50.0,
            phase: 1.0,
            hue: 0,
        };

        update_seeds(std::slice::from_mut(&mut at_start), 0.0, 1.0, 100, 100);
        update_seeds(std::slice::from_mut(&mut later), 0.5, 1.0, 100, 100);

        assert_ne!(at_start.x, later.x);
        assert_ne!(at_start.y, later.y);
    }

    #[test]
    fn color_for_cell_darkens_near_seed_center() {
        let hue = 128;
        let edge = 2.0;
        let fill = Rgba::new(hue, hue.wrapping_add(85), hue.wrapping_add(170), 255);

        assert_eq!(color_for_cell(hue, 0.0, edge), Rgba::black());
        assert_eq!(color_for_cell(hue, 4.0, edge), fill);
    }

    #[test]
    fn same_frame_is_deterministic() {
        let frame_a = render_voronoi_frame(100, 100, 0, 42, 12, 1.0, 2.0);
        let frame_b = render_voronoi_frame(100, 100, 0, 42, 12, 1.0, 2.0);
        assert_eq!(frame_a.get_pixel(50, 50), frame_b.get_pixel(50, 50));
    }

    #[test]
    fn later_frames_differ_from_frame_zero() {
        let frame0 = render_voronoi_frame(100, 100, 0, 42, 12, 1.0, 2.0);
        let frame100 = render_voronoi_frame(100, 100, 100, 42, 12, 1.0, 2.0);

        let mut found_difference = false;
        for y in 0..100 {
            for x in 0..100 {
                if frame0.get_pixel(x, y) != frame100.get_pixel(x, y) {
                    found_difference = true;
                    break;
                }
            }
            if found_difference {
                break;
            }
        }
        assert!(found_difference);
    }
}
