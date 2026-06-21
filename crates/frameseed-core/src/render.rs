use crate::config::{ConfigError, RenderConfig};
use crate::{
    effects_from_config, frame_path, scene_from_config, Frame, RenderContext, Rgba,
};
use rayon::prelude::*;
use std::sync::atomic::{AtomicU32, Ordering};
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
///
/// Frames are rendered in parallel across all available CPU cores. Each
/// parallel worker constructs its own scene and effects from the config so
/// there is no shared mutable state between threads.
pub fn render_sequence<F>(
    config: &RenderConfig,
    output_dir: &Path,
    on_progress: F,
) -> Result<(), RenderError>
where
    F: Fn(u32, u32) + Send + Sync,
{
    config.validate()?;
    std::fs::create_dir_all(output_dir)?;

    let total = config.total_frames();
    let completed = AtomicU32::new(0);

    (0..total)
        .into_par_iter()
        .map(|i| -> Result<(), RenderError> {
            let scene = scene_from_config(&config.scene).map_err(RenderError::Config)?;
            let mut effects = effects_from_config(&config.effects);

            let mut frame = Frame::new(config.width, config.height);
            frame.clear(Rgba::black());
            let ctx = RenderContext::new(i, total, config.fps, config.seed);
            scene.render(&mut frame, &ctx);
            for effect in &mut effects {
                effect.apply(&mut frame, &ctx);
            }
            frame.save_png(&frame_path(output_dir, i + 1))?;

            let done = completed.fetch_add(1, Ordering::Relaxed) + 1;
            on_progress(done, total);
            Ok(())
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(())
}

/// Render all frames in parallel and return them as raw RGBA byte buffers in
/// frame order, ready to stream to an encoder via stdin.
///
/// When the `gpu` feature is enabled and the scene has a compute shader, frames
/// are rendered sequentially on the GPU (each GPU frame is far faster than a
/// CPU frame).  Otherwise the existing CPU rayon path is used.
///
/// Memory: `width × height × 4 × total_frames` bytes are held at once.
pub fn render_frames_parallel<F>(
    config: &RenderConfig,
    on_progress: F,
) -> Result<Vec<Vec<u8>>, RenderError>
where
    F: Fn(u32, u32) + Send + Sync,
{
    config.validate()?;

    // Try GPU path first (no-op if feature disabled or scene unsupported).
    #[cfg(feature = "gpu")]
    if let Some(gpu_buffers) = crate::gpu::try_gpu_render_all(config, &on_progress) {
        return Ok(gpu_buffers);
    }

    let total = config.total_frames();
    let completed = AtomicU32::new(0);

    let mut buffers: Vec<Vec<u8>> = (0..total).map(|_| Vec::new()).collect();

    buffers
        .par_iter_mut()
        .enumerate()
        .try_for_each(|(i, buf)| -> Result<(), RenderError> {
            let scene = scene_from_config(&config.scene).map_err(RenderError::Config)?;
            let mut effects = effects_from_config(&config.effects);
            let mut frame = Frame::new(config.width, config.height);
            frame.clear(Rgba::black());
            let ctx = RenderContext::new(i as u32, total, config.fps, config.seed);
            scene.render(&mut frame, &ctx);
            for effect in &mut effects {
                effect.apply(&mut frame, &ctx);
            }
            *buf = frame.as_raw_rgba().to_vec();
            let done = completed.fetch_add(1, Ordering::Relaxed) + 1;
            on_progress(done, total);
            Ok(())
        })?;

    Ok(buffers)
}

/// Render a single preview frame to a PNG file.
pub fn render_preview_frame(
    config: &RenderConfig,
    frame_index: u32,
    output_path: &Path,
) -> Result<(), RenderError> {
    config.validate()?;

    let scene = scene_from_config(&config.scene)?;
    let mut effects = effects_from_config(&config.effects);
    let total_frames = config.total_frames().max(1);
    let index = frame_index.min(total_frames.saturating_sub(1));

    let mut frame = Frame::new(config.width, config.height);
    frame.clear(Rgba::black());
    let ctx = RenderContext::new(index, total_frames, config.fps, config.seed);
    scene.render(&mut frame, &ctx);
    for effect in &mut effects {
        effect.apply(&mut frame, &ctx);
    }

    if let Some(parent) = output_path.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)?;
        }
    }
    frame.save_png(output_path)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{EffectsConfig, SceneConfig};
    use crate::scenes::{
        ConwayParams, FlowFieldParams, GradientParams, LissajousParams, MandelbrotParams,
        NoiseCloudsParams, ParticleParams, PlasmaParams, SdfShapeParams, SineWaveParams,
        StarfieldParams, TunnelParams, VoronoiParams,
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
            plasma: PlasmaParams::default(),
            lissajous: LissajousParams::default(),
            sine_wave: SineWaveParams::default(),
            starfield: StarfieldParams::default(),
            tunnel: TunnelParams::default(),
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
