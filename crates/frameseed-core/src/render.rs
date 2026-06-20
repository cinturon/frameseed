use crate::config::{ConfigError, RenderConfig};
use crate::{
    effects_from_config, frame_path, scene_from_config, Frame, RenderContext, Rgba,
};
use std::fmt::{Display, Formatter};
use std::path::Path;

#[derive(Debug)]
pub enum RenderError {
    Config(ConfigError),
    Io(std::io::Error),
    Image(image::ImageError),
}

impl Display for RenderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RenderError::Config(e) => write!(f, "{e}"),
            RenderError::Io(e) => write!(f, "IO error: {e}"),
            RenderError::Image(e) => write!(f, "Image error: {e}"),
        }
    }
}

impl std::error::Error for RenderError {}

impl From<ConfigError> for RenderError {
    fn from(value: ConfigError) -> Self {
        RenderError::Config(value)
    }
}

impl From<std::io::Error> for RenderError {
    fn from(value: std::io::Error) -> Self {
        RenderError::Io(value)
    }
}

impl From<image::ImageError> for RenderError {
    fn from(value: image::ImageError) -> Self {
        RenderError::Image(value)
    }
}

/// Render every frame of `config` into numbered PNGs under `output_dir`.
pub fn render_sequence<F>(
    config: &RenderConfig,
    output_dir: &Path,
    mut on_progress: F,
) -> Result<(), RenderError>
where
    F: FnMut(u32, u32),
{
    config.validate()?;
    std::fs::create_dir_all(output_dir)?;

    let scene = scene_from_config(&config.scene)?;
    let mut effects = effects_from_config(&config.effects);
    let total_frame_count = config.total_frames();
    let mut frame = Frame::new(config.width, config.height);

    for i in 0..total_frame_count {
        frame.clear(Rgba::black());
        let ctx = RenderContext::new(i, total_frame_count, config.fps, config.seed);
        scene.render(&mut frame, &ctx);
        for effect in &mut effects {
            effect.apply(&mut frame, &ctx);
        }
        frame.save_png(&frame_path(output_dir, i + 1))?;
        on_progress(i + 1, total_frame_count);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{EffectsConfig, SceneConfig};
    use crate::scenes::{
        ConwayParams, FlowFieldParams, GradientParams, MandelbrotParams, NoiseCloudsParams,
        ParticleParams, SdfShapeParams, VoronoiParams,
    };
    use std::sync::{Arc, Mutex};

    fn test_scene_config() -> SceneConfig {
        SceneConfig {
            name: "gradient".into(),
            gradient: GradientParams::default(),
            noise_clouds: NoiseCloudsParams::default(),
            conway: ConwayParams::default(),
            particles: ParticleParams::default(),
            flow_field: FlowFieldParams::default(),
            sdf_shapes: SdfShapeParams::default(),
            mandelbrot: MandelbrotParams::default(),
            voronoi: VoronoiParams::default(),
        }
    }

    #[test]
    fn render_sequence_writes_expected_frame_count() {
        let dir = std::env::temp_dir().join("frameseed_render_sequence_test");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();

        let config = RenderConfig::new(
            16,
            16,
            24.0,
            0.1,
            42,
            test_scene_config(),
            EffectsConfig::default(),
        );

        let progress = Arc::new(Mutex::new(Vec::new()));
        let progress_clone = progress.clone();
        render_sequence(&config, &dir, |current, total| {
            progress_clone.lock().unwrap().push((current, total));
        })
        .unwrap();

        let expected = config.total_frames();
        assert_eq!(progress.lock().unwrap().len() as u32, expected);
        for i in 1..=expected {
            assert!(frame_path(&dir, i).exists());
        }

        let _ = std::fs::remove_dir_all(&dir);
    }
}
