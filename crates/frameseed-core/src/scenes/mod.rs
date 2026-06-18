mod gradient;
pub use gradient::{GradientScene, GradientParams, palette_from_name};

mod noise_clouds;
pub use noise_clouds::{NoiseCloudsScene, NoiseCloudsParams};

mod registry;
pub use registry::scene_from_config;
