mod color;
pub use color::{lerp_rgba, Rgba};

mod frame;
pub use frame::Frame;

mod context;
pub use context::RenderContext;

mod rng;
pub use rng::seeded_rng;

mod noise;
pub use noise::value_noise_2d;

mod scenes;
pub use scenes::{scene_from_config, GradientParams, GradientScene, MandelbrotScene, palette_from_name, KNOWN_PALETTES};

mod config;
pub use config::{RenderConfig, load_from_path, EffectsConfig, SceneConfig};

mod effects;
pub use effects::{effects_from_config, InvertEffect};

mod presets;
pub use presets::{list_presets, load_preset, preset_path, save_preset};

mod gallery;
pub use gallery::{gallery_entries, GalleryEntry};

mod base64;
pub use base64::frame_to_base64;

mod render;
pub use render::{render_preview_frame, render_sequence, RenderError};


use std::path::Path;

pub trait Scene {
    fn name(&self) -> &str;
    fn render(&self, frame: &mut Frame, context: &RenderContext);
}

pub trait Effect {
    fn name(&self) -> &str;
    fn apply(&mut self, frame: &mut Frame, context: &RenderContext);
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
