use crate::config::EffectsConfig;
use crate::effects::InvertEffect;
use crate::{Effect};
use crate::effects::{PixelationEffect};

pub fn effects_from_config(effects: &EffectsConfig) -> Vec<Box<dyn Effect>> {
    let mut pipeline: Vec<Box<dyn Effect>> = Vec::new();
    if effects.invert {
        pipeline.push(Box::new(InvertEffect));
    }
    if let Some(pixelation) = &effects.pixelation && pixelation.block_size > 1 {
        pipeline.push(Box::new(PixelationEffect::new(pixelation.block_size)));
    }

    pipeline
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::effects::PixelationParams;

    #[test]
    fn test_effects_from_config() {
        let effects = EffectsConfig::default();
        let effects = effects_from_config(&effects);
        assert_eq!(effects.len(), 0);
    }

    #[test]
    fn test_effects_from_config_with_invert() {
        let effects = EffectsConfig { invert: true, pixelation: None };
        let effects = effects_from_config(&effects);
        assert_eq!(effects.len(), 1);
        assert_eq!(effects[0].name(), "invert");
    }

    #[test]
    fn test_effects_from_config_with_pixelation() {
        let effects = EffectsConfig { invert: false, pixelation: Some(PixelationParams { block_size: 8 }) };
        let effects = effects_from_config(&effects);
        assert_eq!(effects.len(), 1);
        assert_eq!(effects[0].name(), "pixelation");
    }
}