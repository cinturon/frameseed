use crate::Scene;    
use crate::GradientScene;
use crate::Rgba;
use crate::config::ConfigError;

pub fn scene_from_name(name: &str) -> Result<Box<dyn Scene>, ConfigError> {
    match name {
        "gradient" => Ok(Box::new(GradientScene::new(Rgba::black(), Rgba::white()))),
        _ => Err(ConfigError::Invalid(format!("Unknown scene: {}", name))),
    }
}