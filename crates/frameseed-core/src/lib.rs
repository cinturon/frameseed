#[cfg(feature = "gpu")]
pub(crate) mod gpu;

mod color;
pub use color::{Rgba, lerp_rgba};

mod frame;
pub use frame::Frame;

mod context;
pub use context::RenderContext;

mod rng;
pub use rng::seeded_rng;

mod noise;
pub use noise::value_noise_2d;

mod scenes;
pub use scenes::{
    GradientParams, GradientScene, KNOWN_PALETTES, KNOWN_SCENES, MandelbrotScene,
    palette_from_name, scene_from_config,
};

mod config;
pub use config::{
    AudioMapping, BeatPulseConfig, EffectsConfig, RenderConfig, SceneConfig, SpeedKeyframe,
    load_from_path,
};

mod audio;
pub use audio::{AudioAnalysis, analyze_wav};

mod effects;
pub use effects::{InvertEffect, effects_from_config};

mod presets;
pub use presets::{list_presets, load_preset, preset_path, save_preset};

mod gallery;
pub use gallery::{GalleryEntry, gallery_entries};

mod base64;
pub use base64::frame_to_base64;

mod render;
pub use render::{
    RenderError, compute_warped_times, config_for_context, render_frames_parallel,
    render_preview_frame, render_sequence,
};

use std::path::Path;

pub trait Scene {
    fn name(&self) -> &str;
    fn render(&self, frame: &mut Frame, context: &RenderContext);
}

pub trait Effect {
    fn name(&self) -> &str;
    fn apply(&mut self, frame: &mut Frame, context: &RenderContext);
}

/// Return a loop-perfect duration for the current scene, rounded to an exact
/// frame boundary.  Works analytically for speed-driven scenes; returns
/// the original duration for stateful/random scenes.
pub fn suggest_loop_duration(config: &RenderConfig) -> f32 {
    let speed = match config.scene.name.as_str() {
        "gradient" => config.scene.gradient.speed,
        "plasma" => config.scene.plasma.speed,
        "tunnel" => config.scene.tunnel.speed,
        "sine_wave" => config.scene.sine_wave.speed,
        "noise_clouds" => config.scene.noise_clouds.speed,
        "sdf_shapes" => config.scene.sdf_shapes.speed,
        "mandelbrot" => config.scene.mandelbrot.zoom_speed,
        "voronoi" => config.scene.voronoi.speed,
        "lissajous" => config.scene.lissajous.speed,
        "starfield" => config.scene.starfield.speed,
        "kaleidoscope" => config.scene.kaleidoscope.speed,
        "metaballs" => config.scene.metaballs.speed,
        "oscilloscope" => config.scene.oscilloscope.speed,
        _ => return config.duration,
    };
    let speed = speed.max(0.001);
    let cycle = 1.0 / speed; // one full animation cycle
    let n = (config.duration / cycle).round().max(1.0) as u32;
    let raw = n as f32 * cycle;
    // round to exact frame count so last frame = first frame
    ((raw * config.fps).round().max(1.0)) / config.fps
}

pub fn config_to_toml(config: &RenderConfig) -> Result<String, String> {
    toml::to_string_pretty(config).map_err(|e| e.to_string())
}

pub fn welcome_message() -> &'static str {
    "Welcome to Frameseed."
}

pub fn frame_path(output_dir: &Path, frame_number: u32) -> std::path::PathBuf {
    output_dir.join(format!("frame_{:06}.png", frame_number))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_welcome_message() {
        assert_eq!(welcome_message(), "Welcome to Frameseed.");
    }
}
