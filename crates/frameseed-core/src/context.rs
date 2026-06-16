use rand_chacha::ChaCha8Rng;

use crate::seeded_rng;

pub struct RenderContext {
    pub frame_index: u32,
    pub total_frames: u32,
    pub time_seconds: f32,
    pub normalized_time: f32,
    pub seed: u64,
}

impl RenderContext {
    
    pub fn new(frame_index: u32, total_frames: u32, fps: f32, seed: u64) -> Self {
        let time_seconds = frame_index as f32 / fps;
        let normalized_time = if total_frames <= 1 {
            0.0
        } else {
            frame_index as f32 / (total_frames - 1) as f32
        };

        Self {
            frame_index,
            total_frames,
            time_seconds,
            normalized_time,
            seed,
        }
    }

    pub fn rng(&self) -> ChaCha8Rng {
        seeded_rng(self.seed)
    }
}

#[cfg(test)]
mod tests{
    use rand::Rng;

    use super::*;

    #[test]
    fn test_new() {
        let ctx = RenderContext::new(0, 120, 24.0, 42);
        assert_eq!(ctx.frame_index, 0);
        assert_eq!(ctx.total_frames, 120);
        assert_eq!(ctx.time_seconds, 0.0);
        assert_eq!(ctx.normalized_time, 0.0);
        assert_eq!(ctx.seed, 42);
    }

    #[test]
    fn last_frame_is_one(){
        let ctx = RenderContext::new(119, 120, 24.0, 42);
        assert!((ctx.normalized_time - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn first_frame_normalized_time(){
        let ctx = RenderContext::new(0, 120, 24.0, 42);
        assert!((ctx.normalized_time - 0.0).abs() < f32::EPSILON);
        assert!((ctx.time_seconds - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn middle_frame_normalized_time() {
        let ctx = RenderContext::new(60, 120, 24.0, 42);
        assert!((ctx.normalized_time - 0.504).abs() < 0.01);
        assert!((ctx.time_seconds - 2.5).abs() < f32::EPSILON);
    }

    #[test]
    fn consecutive_draws_from_one_seedproduces_different_output() {
        let ctx = RenderContext::new(0, 120, 24.0, 42);
        let mut rng = ctx.rng();
        let first = rng.random::<f32>();
        let second = rng.random::<f32>();
        assert_ne!(first, second);
    }

    
}