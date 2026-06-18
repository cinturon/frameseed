use crate::Scene;    
use crate::GradientScene;
use crate::config::ConfigError;
use crate::config::SceneConfig;
use crate::scenes::palette_from_name;

pub fn scene_from_config(scene: &SceneConfig) -> Result<Box<dyn Scene>, ConfigError> {
    match scene.name.as_str() {
        "gradient" => {
            let (start_color, end_color) = palette_from_name(&scene.gradient.palette);
            Ok(Box::new(GradientScene::new(start_color, end_color, scene.gradient.speed)))
        },
        _ => Err(ConfigError::Invalid(format!("Unknown scene: {}", scene.name))),
    }
}