use crate::{
    Frame, RenderConfig, RenderContext, Rgba, Scene, effects_from_config, scene_from_config,
};
use serde::{Deserialize, Serialize};

pub struct BlendScene {
    scene_a: Box<dyn Scene>,
    scene_b: Box<dyn Scene>,
    speed: f32,
}

impl BlendScene {
    pub fn new(scene_a: Box<dyn Scene>, scene_b: Box<dyn Scene>, speed: f32) -> Self {
        Self {
            scene_a,
            scene_b,
            speed,
        }
    }
}

pub struct ConfigScene {
    config: RenderConfig,
}

impl ConfigScene {
    pub fn new(config: RenderConfig) -> Self {
        Self { config }
    }
}

impl Scene for ConfigScene {
    fn name(&self) -> &str {
        "config"
    }

    fn render(&self, frame: &mut Frame, context: &RenderContext) {
        let Ok(scene) = scene_from_config(&self.config.scene) else {
            return;
        };
        let mut effects = effects_from_config(&self.config.effects);
        scene.render(frame, context);
        for effect in &mut effects {
            effect.apply(frame, context);
        }
    }
}

impl Scene for BlendScene {
    fn name(&self) -> &str {
        "blend"
    }

    fn render(&self, frame: &mut Frame, context: &RenderContext) {
        let t = if context.total_frames <= 1 {
            0.0
        } else {
            context.frame_index as f32 / (context.total_frames - 1) as f32
        };
        let ratio = ((t * std::f32::consts::TAU * self.speed).sin() + 1.0) / 2.0;

        let mut frame_a = Frame::new(frame.width, frame.height);
        let mut frame_b = Frame::new(frame.width, frame.height);
        frame_a.clear(Rgba::black());
        frame_b.clear(Rgba::black());

        self.scene_a.render(&mut frame_a, context);
        self.scene_b.render(&mut frame_b, context);

        for y in 0..frame.height {
            for x in 0..frame.width {
                let a = frame_a.get_pixel(x, y).unwrap_or(Rgba::black());
                let b = frame_b.get_pixel(x, y).unwrap_or(Rgba::black());
                frame.set_pixel(
                    x,
                    y,
                    Rgba::new(
                        lerp(a.r, b.r, ratio),
                        lerp(a.g, b.g, ratio),
                        lerp(a.b, b.b, ratio),
                        255,
                    ),
                );
            }
        }
    }
}

fn lerp(a: u8, b: u8, t: f32) -> u8 {
    ((a as f32) * (1.0 - t) + (b as f32) * t).round() as u8
}

fn default_blend_scene_a() -> String {
    "gradient".to_string()
}
fn default_blend_scene_b() -> String {
    "plasma".to_string()
}
fn default_blend_speed() -> f32 {
    1.0
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlendParams {
    #[serde(default = "default_blend_scene_a")]
    pub scene_a: String,
    #[serde(default = "default_blend_scene_b")]
    pub scene_b: String,
    #[serde(default)]
    pub preset_a: Option<String>,
    #[serde(default)]
    pub preset_b: Option<String>,
    #[serde(default = "default_blend_speed")]
    pub speed: f32,
}

impl Default for BlendParams {
    fn default() -> Self {
        Self {
            scene_a: default_blend_scene_a(),
            scene_b: default_blend_scene_b(),
            preset_a: None,
            preset_b: None,
            speed: default_blend_speed(),
        }
    }
}
