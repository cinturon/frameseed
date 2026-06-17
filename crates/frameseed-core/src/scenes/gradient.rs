use crate::{Frame, RenderContext, Rgba, Scene};

pub struct GradientScene {
    pub start_color: Rgba,
    pub end_color: Rgba,
}

impl GradientScene {
    pub fn new(start_color: Rgba, end_color: Rgba) -> Self {
        Self {
            start_color,
            end_color,
        }
    }
}

impl Scene for GradientScene {
    fn name(&self) -> &str{
        "gradient"
    }

    fn render(&self, frame: &mut Frame, context: &RenderContext){
        frame.fill_sliding_horizontal_gradient(
            self.start_color,
            self.end_color,
            context.normalized_time
        )
    }
}