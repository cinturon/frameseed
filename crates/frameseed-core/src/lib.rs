mod color;
pub use color::{lerp_rgba, Rgba};

mod frame;
pub use frame::Frame;

mod context;
pub use context::RenderContext;

mod rng;
pub use rng::seeded_rng;

mod config;
pub use config::{RenderConfig, load_from_path};

mod scenes;
pub use scenes::GradientScene;

use std::path::Path;

pub trait Scene {
    fn name(&self) -> &str;
    fn render(&self, frame: &mut Frame, context: &RenderContext);
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
