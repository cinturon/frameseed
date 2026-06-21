use crate::config::{ConfigError, SceneConfig};
use crate::scenes::ConwayScene;
use crate::scenes::FlowFieldScene;
use crate::scenes::KaleidoscopeScene;
use crate::scenes::LissajousScene;
use crate::scenes::MandelbrotScene;
use crate::scenes::MetaballsScene;
use crate::scenes::NoiseCloudsScene;
use crate::scenes::OscilloscopeScene;
use crate::scenes::ParticlesScene;
use crate::scenes::PlasmaScene;
use crate::scenes::SdfShapeScene;
use crate::scenes::SineWaveScene;
use crate::scenes::StarfieldScene;
use crate::scenes::TunnelScene;
use crate::scenes::VoronoiScene;
use crate::scenes::palette_from_name;
use crate::scenes::{BlendScene, ConfigScene};
use crate::{GradientScene, Scene, load_preset};

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
    "sine_wave",
    "starfield",
    "tunnel",
    "kaleidoscope",
    "metaballs",
    "oscilloscope",
    "blend",
];

pub fn scene_from_config(scene: &SceneConfig) -> Result<Box<dyn Scene>, ConfigError> {
    match scene.name.as_str() {
        "gradient" => {
            let (palette_start, palette_end) = palette_from_name(&scene.gradient.palette);
            let start_color = scene
                .gradient
                .start_color
                .as_deref()
                .and_then(crate::Rgba::from_hex)
                .unwrap_or(palette_start);
            let end_color = scene
                .gradient
                .end_color
                .as_deref()
                .and_then(crate::Rgba::from_hex)
                .unwrap_or(palette_end);
            Ok(Box::new(GradientScene::new(
                start_color,
                end_color,
                scene.gradient.speed,
                scene.gradient.direction.clone(),
            )))
        }
        "noise_clouds" => Ok(Box::new(NoiseCloudsScene::new(
            scene.noise_clouds.speed,
            scene.noise_clouds.scale,
            scene.noise_clouds.colored,
        ))),
        "conway" => Ok(Box::new(ConwayScene::new(
            scene.conway.cell_size,
            scene.conway.density,
        ))),
        "particles" => Ok(Box::new(ParticlesScene::new(
            scene.particles.count,
            scene.particles.speed,
            scene.particles.kind.clone(),
            scene.particles.fps,
        ))),
        "flow_field" => Ok(Box::new(FlowFieldScene::new(
            scene.flow_field.count,
            scene.flow_field.speed,
            scene.flow_field.scale,
            scene.flow_field.fps,
        ))),
        "sdf_shapes" => Ok(Box::new(SdfShapeScene::new(
            scene.sdf_shapes.circle_radius,
            scene.sdf_shapes.box_half_width,
            scene.sdf_shapes.box_half_height,
            scene.sdf_shapes.speed,
        ))),
        "mandelbrot" => Ok(Box::new(MandelbrotScene::new(
            scene.mandelbrot.max_iter,
            scene.mandelbrot.center_re,
            scene.mandelbrot.center_im,
            scene.mandelbrot.initial_view_width,
            scene.mandelbrot.zoom_speed,
        ))),
        "voronoi" => Ok(Box::new(VoronoiScene::new(
            scene.voronoi.seed_count,
            scene.voronoi.speed,
            scene.voronoi.edge_width,
        ))),
        "plasma" => Ok(Box::new(PlasmaScene::new(
            scene.plasma.speed,
            scene.plasma.scale,
        ))),
        "lissajous" => {
            let p = &scene.lissajous;
            Ok(Box::new(LissajousScene::new(
                p.a,
                p.b,
                p.delta,
                p.speed,
                p.thickness,
                p.trail_frames,
            )))
        }
        "sine_wave" => Ok(Box::new(SineWaveScene::new(scene.sine_wave.speed))),
        "starfield" => Ok(Box::new(StarfieldScene::new(
            scene.starfield.count,
            scene.starfield.speed,
        ))),
        "tunnel" => Ok(Box::new(TunnelScene::new(
            scene.tunnel.speed,
            scene.tunnel.rings,
        ))),
        "kaleidoscope" => Ok(Box::new(KaleidoscopeScene::new(
            scene.kaleidoscope.segments,
            scene.kaleidoscope.speed,
            scene.kaleidoscope.zoom,
        ))),
        "metaballs" => Ok(Box::new(MetaballsScene::new(
            scene.metaballs.count,
            scene.metaballs.speed,
            scene.metaballs.threshold,
        ))),
        "oscilloscope" => Ok(Box::new(OscilloscopeScene::new(
            scene.oscilloscope.frequency,
            scene.oscilloscope.amplitude,
            scene.oscilloscope.speed,
            scene.oscilloscope.glow,
        ))),
        "blend" => {
            if scene.blend.scene_a == "blend" || scene.blend.scene_b == "blend" {
                return Err(ConfigError::Invalid(
                    "Blend scene cannot reference itself.".into(),
                ));
            }
            let scene_a =
                blend_source(scene, scene.blend.preset_a.as_deref(), &scene.blend.scene_a)?;
            let scene_b =
                blend_source(scene, scene.blend.preset_b.as_deref(), &scene.blend.scene_b)?;
            Ok(Box::new(BlendScene::new(
                scene_a,
                scene_b,
                scene.blend.speed,
            )))
        }
        _ => Err(ConfigError::Invalid(format!(
            "Unknown scene '{}'. Run `frameseed list-scenes` to see available scenes: {}",
            scene.name,
            KNOWN_SCENES.join(", ")
        ))),
    }
}

fn blend_source(
    scene: &SceneConfig,
    preset: Option<&str>,
    fallback_scene: &str,
) -> Result<Box<dyn Scene>, ConfigError> {
    if let Some(preset) = preset.filter(|value| !value.trim().is_empty()) {
        let config = load_preset(preset)?;
        return Ok(Box::new(ConfigScene::new(config)));
    }

    let mut cfg = scene.clone();
    cfg.name = fallback_scene.to_string();
    scene_from_config(&cfg)
}
