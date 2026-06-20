mod invert;
pub use invert::InvertEffect;

mod blur;
pub use blur::{BoxBlurEffect, BlurParams};

mod pixelation;
pub use pixelation::{PixelationEffect, PixelationParams};

mod palette;
pub use palette::{PaletteQuantizationEffect, PaletteQuantizationParams};

mod dither;
pub use dither::{OrderedDitherEffect, DitherParams};

mod motion_blur;
pub use motion_blur::{MotionBlurEffect, MotionBlurParams};

mod vhs_crt;
pub use vhs_crt::{VhsCrtEffect, VhsCrtParams};

mod registry;
pub use registry::effects_from_config;