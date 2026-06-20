use crate::Effect;
use crate::config::EffectsConfig;
use crate::effects::InvertEffect;
use crate::effects::{
    BoxBlurEffect, MotionBlurEffect, OrderedDitherEffect, PaletteQuantizationEffect,
    PixelationEffect, VhsCrtEffect,
};

pub fn effects_from_config(effects: &EffectsConfig) -> Vec<Box<dyn Effect>> {
    let mut pipeline: Vec<Box<dyn Effect>> = Vec::new();

    if let Some(motion_blur) = &effects.motion_blur {
        pipeline.push(Box::new(MotionBlurEffect::new(motion_blur.strength)));
    }
    if effects.invert {
        pipeline.push(Box::new(InvertEffect));
    }
    if let Some(pixelation) = &effects.pixelation
        && pixelation.block_size > 1
    {
        pipeline.push(Box::new(PixelationEffect::new(pixelation.block_size)));
    }
    if let Some(dither) = &effects.dither {
        pipeline.push(Box::new(OrderedDitherEffect::new(dither.spread)));
    }
    if let Some(palette) = &effects.palette {
        pipeline.push(Box::new(PaletteQuantizationEffect::from_name(
            &palette.name,
        )));
    }

    if let Some(vhs_crt) = &effects.vhs_crt {
        pipeline.push(Box::new(VhsCrtEffect::new(
            vhs_crt.scanlines_strength,
            vhs_crt.chromatic_offset,
            vhs_crt.noise_amount,
            vhs_crt.warp_amount,
        )));
    }
    if let Some(blur) = &effects.blur {
        pipeline.push(Box::new(BoxBlurEffect::new(blur.radius)));
    }
    pipeline
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::effects::DitherParams;
    use crate::effects::MotionBlurParams;
    use crate::effects::PaletteQuantizationParams;
    use crate::effects::PixelationParams;
    use crate::effects::VhsCrtParams;

    #[test]
    fn test_effects_from_config() {
        let effects = EffectsConfig::default();
        let effects = effects_from_config(&effects);
        assert_eq!(effects.len(), 0);
    }

    #[test]
    fn test_effects_from_config_with_invert() {
        let effects = EffectsConfig {
            invert: true,
            pixelation: None,
            palette: None,
            dither: None,
            motion_blur: None,
            vhs_crt: None,
            blur: None,
        };
        let effects = effects_from_config(&effects);
        assert_eq!(effects.len(), 1);
        assert_eq!(effects[0].name(), "invert");
    }

    #[test]
    fn test_effects_from_config_with_pixelation() {
        let effects = EffectsConfig {
            invert: false,
            pixelation: Some(PixelationParams { block_size: 8 }),
            palette: None,
            dither: None,
            motion_blur: None,
            vhs_crt: None,
            blur: None,
        };
        let effects = effects_from_config(&effects);
        assert_eq!(effects.len(), 1);
        assert_eq!(effects[0].name(), "pixelation");
    }

    #[test]
    fn test_effects_from_config_with_palette() {
        let effects = EffectsConfig {
            invert: false,
            pixelation: None,
            palette: Some(PaletteQuantizationParams {
                name: "cga16".to_string(),
            }),
            dither: None,
            motion_blur: None,
            vhs_crt: None,
            blur: None,
        };
        let effects = effects_from_config(&effects);
        assert_eq!(effects.len(), 1);
        assert_eq!(effects[0].name(), "palette");
    }

    #[test]
    fn test_effects_from_config_with_dither() {
        let effects = EffectsConfig {
            invert: false,
            pixelation: None,
            palette: None,
            dither: Some(DitherParams { spread: 48.0 }),
            motion_blur: None,
            vhs_crt: None,
            blur: None,
        };
        let effects = effects_from_config(&effects);
        assert_eq!(effects.len(), 1);
        assert_eq!(effects[0].name(), "dither");
    }

    #[test]
    fn test_effects_from_config_with_motion_blur() {
        let effects = EffectsConfig {
            invert: false,
            pixelation: None,
            palette: None,
            dither: None,
            motion_blur: Some(MotionBlurParams { strength: 0.5 }),
            vhs_crt: None,
            blur: None,
        };
        let effects = effects_from_config(&effects);
        assert_eq!(effects.len(), 1);
        assert_eq!(effects[0].name(), "motion_blur");
    }

    #[test]
    fn test_effects_from_config_with_vhs_crt() {
        let effects = EffectsConfig {
            invert: false,
            pixelation: None,
            palette: None,
            dither: None,
            motion_blur: None,
            vhs_crt: Some(VhsCrtParams {
                scanlines_strength: 0.5,
                chromatic_offset: 2.0,
                noise_amount: 0.08,
                warp_amount: 2.5,
            }),
            blur: None,
        };
        let effects = effects_from_config(&effects);
        assert_eq!(effects.len(), 1);
        assert_eq!(effects[0].name(), "vhs_crt");
    }
}
