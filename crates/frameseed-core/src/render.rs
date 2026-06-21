use crate::config::{BeatPulseConfig, ConfigError, RenderConfig, SpeedKeyframe};
use crate::{Frame, RenderContext, Rgba, effects_from_config, frame_path, scene_from_config};
use rayon::prelude::*;
use std::fmt::{Display, Formatter};
use std::path::Path;
use std::sync::atomic::{AtomicU32, Ordering};

// ── Time-warp helpers ────────────────────────────────────────────────────────

/// Precompute an effective `time_seconds` for every frame, integrating
/// speed-warp keyframes and beat-pulse contributions.  Returns a plain Vec
/// (one entry per frame) so the parallel render workers can index it.
pub fn compute_warped_times(config: &RenderConfig) -> Vec<f32> {
    let total = config.total_frames() as usize;
    if total == 0 {
        return vec![];
    }

    let has_kf = !config.speed_keyframes.is_empty();
    let has_beats = config
        .beat_pulse
        .as_ref()
        .map(|b| !b.beat_times.is_empty())
        .unwrap_or(false);

    // Fast path: no warping — standard linear times
    if !has_kf && !has_beats {
        return (0..total).map(|i| i as f32 / config.fps).collect();
    }

    let dt = 1.0 / config.fps;
    let mut times = Vec::with_capacity(total);
    let mut accumulated = 0.0f32;

    for i in 0..total {
        times.push(accumulated);
        let t_real = i as f32 * dt;
        let norm = i as f32 / total as f32;

        let kf_mult = if has_kf {
            lerp_keyframes(&config.speed_keyframes, norm)
        } else {
            1.0
        };

        let beat_add = if let Some(ref bp) = config.beat_pulse {
            beat_speed_boost(bp, t_real)
        } else {
            0.0
        };

        accumulated += dt * kf_mult * (1.0 + beat_add);
    }
    times
}

fn lerp_keyframes(kf: &[SpeedKeyframe], norm: f32) -> f32 {
    if kf.is_empty() {
        return 1.0;
    }
    if norm <= kf[0].time {
        return kf[0].speed;
    }
    for w in kf.windows(2) {
        if norm <= w[1].time {
            let t = (norm - w[0].time) / (w[1].time - w[0].time);
            return w[0].speed * (1.0 - t) + w[1].speed * t;
        }
    }
    kf.last().unwrap().speed
}

fn beat_speed_boost(bp: &BeatPulseConfig, t_real: f32) -> f32 {
    let raw: f32 = bp
        .beat_times
        .iter()
        .filter(|&&bt| bt <= t_real)
        .map(|&bt| bp.strength * (-bp.decay * (t_real - bt)).exp())
        .sum();
    raw.min(bp.strength * 3.0)
}

fn audio_pulse(config: &RenderConfig, time_seconds: f32) -> f32 {
    config
        .beat_pulse
        .as_ref()
        .map(|bp| beat_speed_boost(bp, time_seconds))
        .unwrap_or(0.0)
}

pub fn config_for_context(config: &RenderConfig, context: &RenderContext) -> RenderConfig {
    if config.audio_mappings.is_empty() {
        return config.clone();
    }

    let pulse = audio_pulse(config, context.time_seconds);
    if pulse <= 0.0 {
        return config.clone();
    }

    let mut mapped = config.clone();
    for mapping in &config.audio_mappings {
        let amount = mapping.amount * pulse;
        match mapping.target.as_str() {
            "speed" => multiply_scene_speeds(&mut mapped, 1.0 + amount),
            "bloom_intensity" => {
                let bloom = mapped.effects.bloom.get_or_insert_with(Default::default);
                bloom.intensity = (bloom.intensity + amount).clamp(0.0, 4.0);
            }
            "chromatic_offset" => {
                let chromatic = mapped
                    .effects
                    .chromatic_aberration
                    .get_or_insert_with(Default::default);
                chromatic.offset = (chromatic.offset + amount * 4.0).clamp(0.0, 32.0);
            }
            "oscilloscope_glow" => {
                mapped.scene.oscilloscope.glow =
                    (mapped.scene.oscilloscope.glow + amount).clamp(0.1, 6.0);
            }
            "vignette_strength" => {
                let vignette = mapped.effects.vignette.get_or_insert_with(Default::default);
                vignette.strength = (vignette.strength + amount * 0.35).clamp(0.0, 1.0);
            }
            _ => {}
        }
    }
    mapped
}

fn multiply_scene_speeds(config: &mut RenderConfig, multiplier: f32) {
    config.scene.gradient.speed *= multiplier;
    config.scene.noise_clouds.speed *= multiplier;
    config.scene.particles.speed *= multiplier;
    config.scene.flow_field.speed *= multiplier;
    config.scene.sdf_shapes.speed *= multiplier;
    config.scene.mandelbrot.zoom_speed *= multiplier;
    config.scene.voronoi.speed *= multiplier;
    config.scene.plasma.speed *= multiplier;
    config.scene.lissajous.speed *= multiplier;
    config.scene.sine_wave.speed *= multiplier;
    config.scene.starfield.speed *= multiplier;
    config.scene.tunnel.speed *= multiplier;
    config.scene.kaleidoscope.speed *= multiplier;
    config.scene.metaballs.speed *= multiplier;
    config.scene.oscilloscope.speed *= multiplier;
    config.scene.blend.speed *= multiplier;
}

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
    let warped = compute_warped_times(config);

    (0..total)
        .into_par_iter()
        .map(|i| -> Result<(), RenderError> {
            let mut frame = Frame::new(config.width, config.height);
            frame.clear(Rgba::black());
            let mut ctx = RenderContext::new(i, total, config.fps, config.seed);
            ctx.time_seconds = warped[i as usize];
            let frame_config = config_for_context(config, &ctx);
            let scene = scene_from_config(&frame_config.scene).map_err(RenderError::Config)?;
            let mut effects = effects_from_config(&frame_config.effects);
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

    let _has_time_warp = !config.speed_keyframes.is_empty()
        || config
            .beat_pulse
            .as_ref()
            .map(|b| !b.beat_times.is_empty())
            .unwrap_or(false);

    // Try GPU path first — skip if time warp is active (GPU path doesn't support it yet).
    #[cfg(feature = "gpu")]
    if !_has_time_warp {
        if let Some(gpu_buffers) = crate::gpu::try_gpu_render_all(config, &on_progress) {
            return Ok(gpu_buffers);
        }
    }

    let total = config.total_frames();
    let completed = AtomicU32::new(0);
    let warped = compute_warped_times(config);

    let mut buffers: Vec<Vec<u8>> = (0..total).map(|_| Vec::new()).collect();

    buffers
        .par_iter_mut()
        .enumerate()
        .try_for_each(|(i, buf)| -> Result<(), RenderError> {
            let mut frame = Frame::new(config.width, config.height);
            frame.clear(Rgba::black());
            let mut ctx = RenderContext::new(i as u32, total, config.fps, config.seed);
            ctx.time_seconds = warped[i];
            let frame_config = config_for_context(config, &ctx);
            let scene = scene_from_config(&frame_config.scene).map_err(RenderError::Config)?;
            let mut effects = effects_from_config(&frame_config.effects);
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

    let total_frames = config.total_frames().max(1);
    let index = frame_index.min(total_frames.saturating_sub(1));
    let warped = compute_warped_times(config);

    let mut frame = Frame::new(config.width, config.height);
    frame.clear(Rgba::black());
    let mut ctx = RenderContext::new(index, total_frames, config.fps, config.seed);
    if let Some(&t) = warped.get(index as usize) {
        ctx.time_seconds = t;
    }
    let frame_config = config_for_context(config, &ctx);
    let scene = scene_from_config(&frame_config.scene)?;
    let mut effects = effects_from_config(&frame_config.effects);
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
        ConwayParams, FlowFieldParams, GradientParams, KaleidoscopeParams, LissajousParams,
        MandelbrotParams, MetaballsParams, NoiseCloudsParams, OscilloscopeParams, ParticleParams,
        PlasmaParams, SdfShapeParams, SineWaveParams, StarfieldParams, TunnelParams, VoronoiParams,
    };
    use crate::{AudioMapping, BeatPulseConfig};
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
            kaleidoscope: KaleidoscopeParams::default(),
            metaballs: MetaballsParams::default(),
            oscilloscope: OscilloscopeParams::default(),
            blend: crate::scenes::BlendParams::default(),
        }
    }

    #[test]
    fn audio_mapping_modulates_bloom_intensity() {
        let mut config = RenderConfig::new(
            16,
            16,
            24.0,
            1.0,
            42,
            test_scene_config(),
            EffectsConfig::default(),
        );
        config.beat_pulse = Some(BeatPulseConfig {
            beat_times: vec![0.0],
            strength: 1.0,
            decay: 1.0,
        });
        config.audio_mappings = vec![AudioMapping {
            target: "bloom_intensity".into(),
            amount: 1.0,
        }];

        let context = RenderContext::new(0, 24, 24.0, 42);
        let mapped = config_for_context(&config, &context);
        assert!(mapped.effects.bloom.unwrap().intensity > 0.8);
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
