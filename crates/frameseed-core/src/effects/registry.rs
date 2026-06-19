use crate::config::EffectsConfig;
use crate::effects::InvertEffect;
use crate::{Effect};

pub fn effects_from_config(effects: &EffectsConfig) -> Vec<Box<dyn Effect>> {
    let mut pipeline: Vec<Box<dyn Effect>> = Vec::new();
    if effects.invert {
        pipeline.push(Box::new(InvertEffect));
    }
    pipeline
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_effects_from_config() {
        let effects = EffectsConfig::default();
        let effects = effects_from_config(&effects);
        assert_eq!(effects.len(), 0);
    }

    #[test]
    fn test_effects_from_config_with_invert() {
        let effects = EffectsConfig { invert: true };
        let effects = effects_from_config(&effects);
        assert_eq!(effects.len(), 1);
        assert_eq!(effects[0].name(), "invert");
    }
}