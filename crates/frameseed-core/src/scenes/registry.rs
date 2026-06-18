use crate::config::{ConfigError, SceneConfig};
use crate::scenes::palette_from_name;
use crate::{GradientScene, Scene};
use crate::scenes::NoiseCloudsScene;
use crate::scenes::ConwayScene;

pub fn scene_from_config(scene: &SceneConfig) -> Result<Box<dyn Scene>, ConfigError> {
    match scene.name.as_str() {
        "gradient" => {
            let (start_color, end_color) = palette_from_name(&scene.gradient.palette);
            Ok(Box::new(GradientScene::new(start_color, end_color, scene.gradient.speed)))
        },
        "noise_clouds" => {
            Ok(Box::new(NoiseCloudsScene::new(scene.noise_clouds.speed, scene.noise_clouds.scale)))
        },
        "conway" => {
            Ok(Box::new(ConwayScene::new(scene.conway.cell_size, scene.conway.density)))
        },
        _ => Err(ConfigError::Invalid(format!("Unknown scene: {}", scene.name))),
    }
}