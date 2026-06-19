mod invert;
pub use invert::InvertEffect;

mod pixelation;
pub use pixelation::{PixelationEffect, PixelationParams};

mod palette;
pub use palette::{PaletteQuantizationEffect, PaletteQuantizationParams};

mod registry;
pub use registry::effects_from_config;