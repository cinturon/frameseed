pub struct RenderConfig {
    pub width: u32,
    pub height: u32,
    pub fps: f32,
    pub duration: f32,
    pub seed: u64,
    pub scene: String,
}

impl RenderConfig {
    pub fn new(
        width: u32,
        height: u32,
        fps: f32,
        duration: f32,
        seed: u64,
        scene: impl Into<String>,
    ) -> Self {
        Self {
            width,
            height,
            fps,
            duration,
            seed,
            scene: scene.into(),
        }
    }

    pub fn total_frames(&self) -> u32 {
        (self.duration * self.fps).ceil() as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let config = RenderConfig::new(1920, 1080, 24.0, 5.0, 42, "gradient");
        assert_eq!(config.width, 1920);
        assert_eq!(config.height, 1080);
        assert_eq!(config.fps, 24.0);
        assert_eq!(config.duration, 5.0);
        assert_eq!(config.seed, 42);
        assert_eq!(config.scene, "gradient");
    }

    #[test]
    fn test_total_frames() {
        let config = RenderConfig::new(1920, 1080, 24.0, 5.0, 42, "gradient");
        assert_eq!(config.total_frames(), 120);
    }
}
