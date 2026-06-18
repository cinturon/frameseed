use crate::{Frame, RenderContext, Rgba, Scene};
use serde::Deserialize;

pub struct GradientScene {
    pub start_color: Rgba,
    pub end_color: Rgba,
    pub speed: f32,
}

impl GradientScene {
    pub fn new(start_color: Rgba, end_color: Rgba, speed: f32) -> Self {
        Self {
            start_color,
            end_color,
            speed,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct GradientParams{
    #[serde(default = "default_speed")]
    pub speed: f32,
    #[serde(default)]
    pub palette: String,
}

impl Default for GradientParams {
    fn default() -> Self {
        Self {
            speed: 1.0,
            palette: String::new(),
        }
    }
}

fn default_speed() -> f32 {
    1.0
}

impl Scene for GradientScene {
    fn name(&self) -> &str{
        "gradient"
    }

    fn render(&self, frame: &mut Frame, context: &RenderContext){
        let offset = context.normalized_time * self.speed;
        frame.fill_sliding_horizontal_gradient(
            self.start_color,
            self.end_color,
            offset,
        )
    }
}

pub fn palette_from_name(name: &str) -> (Rgba, Rgba) {
    match name {
        "sunset" => (Rgba::new(255, 94, 77, 255), Rgba::new(255, 200, 87, 255)),
        _ => (Rgba::black(), Rgba::white()),
    }
}