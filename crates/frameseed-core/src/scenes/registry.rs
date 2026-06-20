use crate::config::{ConfigError, SceneConfig};
use crate::scenes::palette_from_name;
use crate::{GradientScene, Scene};
use crate::scenes::NoiseCloudsScene;
use crate::scenes::ConwayScene;
use crate::scenes::ParticlesScene;
use crate::scenes::FlowFieldScene;
use crate::scenes::SdfShapeScene;
use crate::scenes::MandelbrotScene;
use crate::scenes::VoronoiScene;
use crate::scenes::PlasmaScene;
use crate::scenes::LissajousScene;

pub const KNOWN_SCENES: &[&str] = &[
    "gradient",
    "noise_clouds",
    "conway",
    "particles",
    "flow_field",
    "sdf_shapes",
    "mandelbrot",
    "voronoi",
    "plasma",
    "lissajous",
];

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
        "particles" => {
            Ok(Box::new(ParticlesScene::new(scene.particles.count, scene.particles.speed, scene.particles.kind.clone(), scene.particles.fps)))
        },
        "flow_field" => {
            Ok(Box::new(FlowFieldScene::new(scene.flow_field.count, scene.flow_field.speed, scene.flow_field.scale, scene.flow_field.fps)))
        },
        "sdf_shapes" => {
            Ok(Box::new(SdfShapeScene::new(scene.sdf_shapes.circle_radius, scene.sdf_shapes.box_half_width, scene.sdf_shapes.box_half_height, scene.sdf_shapes.speed)))
        },
        "mandelbrot" => {
            Ok(Box::new(MandelbrotScene::new(scene.mandelbrot.max_iter, scene.mandelbrot.center_re, scene.mandelbrot.center_im, scene.mandelbrot.initial_view_width, scene.mandelbrot.zoom_speed)))
        },
        "voronoi" => {
            Ok(Box::new(VoronoiScene::new(scene.voronoi.seed_count, scene.voronoi.speed, scene.voronoi.edge_width)))
        },
        "plasma" => {
            Ok(Box::new(PlasmaScene::new(scene.plasma.speed, scene.plasma.scale)))
        },
        "lissajous" => {
            let p = &scene.lissajous;
            Ok(Box::new(LissajousScene::new(p.a, p.b, p.delta, p.speed, p.thickness, p.trail_frames)))
        },
        _ => Err(ConfigError::Invalid(format!(
            "Unknown scene '{}'. Run `frameseed list-scenes` to see available scenes: {}",
            scene.name,
            KNOWN_SCENES.join(", ")
        ))),
    }
}