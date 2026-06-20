use serde::{Deserialize, Serialize};
use crate::scenes::{GradientParams, NoiseCloudsParams, ConwayParams, ParticleParams, FlowFieldParams, SdfShapeParams, MandelbrotParams, VoronoiParams, PlasmaParams, LissajousParams, SineWaveParams};
use crate::effects::{PixelationParams, PaletteQuantizationParams, DitherParams, MotionBlurParams, VhsCrtParams, BlurParams};
use std::{error::Error, fmt::Display, path::Path};

#[derive(Debug)]
pub enum ConfigError {
    Io(std::io::Error),
    Parse(toml::de::Error),
    Invalid(String),
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::Io(e) if e.kind() == std::io::ErrorKind::NotFound => {
                write!(
                    f,
                    "Config file not found ({}). Check the path or run `frameseed list-presets`.",
                    e
                )
            }
            ConfigError::Io(e) => write!(f, "Could not read config file: {e}"),
            ConfigError::Parse(e) => write!(
                f,
                "Invalid TOML in config file: {e}. Check field names, types, and `[scene]` settings."
            ),
            ConfigError::Invalid(message) => write!(f, "{message}"),
        }
    }
}

impl Error for ConfigError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ConfigError::Io(e) => Some(e),
            ConfigError::Parse(e) => Some(e),
            ConfigError::Invalid(_) => None,
        }
    }
}

impl From<std::io::Error> for ConfigError {
    fn from(e: std::io::Error) -> Self {
        ConfigError::Io(e)
    }
}

impl From<toml::de::Error> for ConfigError {
    fn from(e: toml::de::Error) -> Self {
        ConfigError::Parse(e)
    }
}

impl From<&str> for ConfigError {
    fn from(e: &str) -> Self {
        ConfigError::Invalid(e.to_string())
    }
}
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct RenderConfig {
    pub width: u32,
    pub height: u32,
    pub fps: f32,
    pub duration: f32,
    pub seed: u64,
    pub scene: SceneConfig,
    #[serde(default)]
    pub effects: EffectsConfig,
}

impl RenderConfig {
    pub fn new(
        width: u32,
        height: u32,
        fps: f32,
        duration: f32,
        seed: u64,
        scene: SceneConfig,
        effects: EffectsConfig,
    ) -> Self {
        Self {
            width,
            height,
            fps,
            duration,
            seed,
            scene,
            effects,
        }
    }

    pub fn total_frames(&self) -> u32 {
        (self.duration * self.fps).ceil() as u32
    }

    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.width == 0 {
            return Err(ConfigError::Invalid(
                "Width must be greater than 0. Set `width = 640` (or another positive value) in your config.".into(),
            ));
        }
        if self.height == 0 {
            return Err(ConfigError::Invalid(
                "Height must be greater than 0. Set `height = 360` (or another positive value) in your config.".into(),
            ));
        }
        if self.fps <= 0.0 {
            return Err(ConfigError::Invalid(
                "FPS must be greater than 0. Set `fps = 24.0` (or another positive value) in your config.".into(),
            ));
        }
        if self.duration <= 0.0 {
            return Err(ConfigError::Invalid(
                "Duration must be greater than 0. Set `duration = 5.0` for a five-second clip.".into(),
            ));
        }
        if self.seed == 0 {
            return Err(ConfigError::Invalid(
                "Seed must be greater than 0. Use any positive integer, e.g. `seed = 42`.".into(),
            ));
        }
        if self.scene.name.is_empty() {
            return Err(ConfigError::Invalid(
                "Scene name is missing. Add `[scene]` with `name = \"gradient\"` (or another scene from `frameseed list-scenes`).".into(),
            ));
        }
        let known = crate::scenes::KNOWN_SCENES;
        if !known.contains(&self.scene.name.as_str()) {
            return Err(ConfigError::Invalid(format!(
                "Unknown scene '{}'. Available scenes: {}. Run `frameseed list-scenes` for details.",
                self.scene.name,
                known.join(", ")
            )));
        }
        Ok(())
    }
}

impl SceneConfig {
    pub fn with_name(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            gradient: GradientParams::default(),
            noise_clouds: NoiseCloudsParams::default(),
            conway: ConwayParams::default(),
            particles: ParticleParams::default(),
            flow_field: FlowFieldParams::default(),
            sdf_shapes: SdfShapeParams::default(),
            mandelbrot: MandelbrotParams::default(),
            voronoi: VoronoiParams::default(),
            plasma: PlasmaParams::default(),
            lissajous: LissajousParams::default(),
            sine_wave: SineWaveParams::default(),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct SceneConfig {
    pub name: String,
    #[serde(default)]
    pub gradient: GradientParams,
    #[serde(default)]
    pub noise_clouds: NoiseCloudsParams,
    #[serde(default)]
    pub conway: ConwayParams,
    #[serde(default)]
    pub particles: ParticleParams,
    #[serde(default)]
    pub flow_field: FlowFieldParams,
    #[serde(default)]
    pub sdf_shapes: SdfShapeParams,
    #[serde(default)]
    pub mandelbrot: MandelbrotParams,
    #[serde(default)]
    pub voronoi: VoronoiParams,
    #[serde(default)]
    pub plasma: PlasmaParams,
    #[serde(default)]
    pub lissajous: LissajousParams,
    #[serde(default)]
    pub sine_wave: SineWaveParams,
}

#[derive(Debug, Deserialize, Clone, Default, Serialize)]
pub struct EffectsConfig {
    #[serde(default)]
    pub invert: bool,
    #[serde(default)]
    pub pixelation: Option<PixelationParams>,
    #[serde(default)]
    pub palette: Option<PaletteQuantizationParams>,
    #[serde(default)]
    pub dither: Option<DitherParams>,
    #[serde(default)]
    pub motion_blur: Option<MotionBlurParams>,
    #[serde(default)]
    pub vhs_crt: Option<VhsCrtParams>,
    #[serde(default)]
    pub blur: Option<BlurParams>,
}

pub fn load_from_path(path: &Path) -> Result<RenderConfig, ConfigError> {
    let contents = std::fs::read_to_string(path)?;
    let config: RenderConfig = toml::from_str(&contents)?;

    config.validate()?;
    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let config = RenderConfig::new(
            1920,
            1080,
            24.0,
            5.0,
            42,
            SceneConfig {
                name: "gradient".to_string(),
                gradient: GradientParams::default(),
                noise_clouds: NoiseCloudsParams::default(),
                conway: ConwayParams::default(),
                particles: ParticleParams::default(),
                flow_field: FlowFieldParams::default(),
                sdf_shapes: SdfShapeParams::default(),
                mandelbrot: MandelbrotParams::default(),
                voronoi: VoronoiParams::default(),
                plasma: PlasmaParams::default(),
            lissajous: LissajousParams::default(),
            sine_wave: SineWaveParams::default(),
            },
            EffectsConfig::default(),
        );
        assert_eq!(config.width, 1920);
        assert_eq!(config.height, 1080);
        assert_eq!(config.fps, 24.0);
        assert_eq!(config.duration, 5.0);
        assert_eq!(config.seed, 42);
        assert_eq!(config.scene.name, "gradient");
    }

    #[test]
    fn test_total_frames() {
        let config = RenderConfig::new(
            1920,
            1080,
            24.0,
            5.0,
            42,
            SceneConfig {
                name: "gradient".to_string(),
                gradient: GradientParams::default(),
                noise_clouds: NoiseCloudsParams::default(),
                conway: ConwayParams::default(),
                particles: ParticleParams::default(),
                flow_field: FlowFieldParams::default(),
                sdf_shapes: SdfShapeParams::default(),
                mandelbrot: MandelbrotParams::default(),
                voronoi: VoronoiParams::default(),
                plasma: PlasmaParams::default(),
            lissajous: LissajousParams::default(),
            sine_wave: SineWaveParams::default(),
            },
            EffectsConfig::default(),
        );
        assert_eq!(config.total_frames(), 120);
    }

    #[test]
    fn loads_valid_toml() {
        let toml_content = r#"
        width = 256
        height = 64
        fps = 24.0
        duration = 5.0
        seed = 42
        
        [scene]
        name = "gradient"

        [scene.gradient]
        speed = 1.0
        palette = "sunset"
        "#;
        let config: RenderConfig = toml::from_str(toml_content).unwrap();
        config.validate().unwrap();
        assert_eq!(config.width, 256);
        assert_eq!(config.height, 64);
        assert_eq!(config.fps, 24.0);
        assert_eq!(config.duration, 5.0);
        assert_eq!(config.seed, 42);
        assert_eq!(config.scene.name, "gradient");
    }

    #[test]
    fn invalid_zero_width_returns_error() {
        let toml_content = r#"
        width = 0
        height = 64
        fps = 24.0
        duration = 5.0
        seed = 42
        
        [scene]
        name = "gradient"

        [scene.gradient]
        speed = 1.0
        palette = "sunset"
        "#;
        let config: RenderConfig = toml::from_str(toml_content).unwrap();
        assert!(config.validate().is_err());
    }

    #[test]
    fn invalid_zero_height_returns_error() {
        let toml_content = r#"
        width = 256
        height = 0
        fps = 24.0
        duration = 5.0
        seed = 42
        
        [scene]
        name = "gradient"

        [scene.gradient]
        speed = 1.0
        palette = "sunset"
        "#;
        let config: RenderConfig = toml::from_str(toml_content).unwrap();
        assert!(config.validate().is_err());
    }

    #[test]
    fn invalid_zero_fps_returns_error() {
        let toml_content = r#"
        width = 256
        height = 64
        fps = 0.0
        duration = 5.0
        seed = 42
        
        [scene]
        name = "gradient"

        [scene.gradient]
        speed = 1.0
        palette = "sunset"
        "#;
        let config: RenderConfig = toml::from_str(toml_content).unwrap();
        assert!(config.validate().is_err());
    }

    #[test]
    fn invalid_zero_duration_returns_error() {
        let toml_content = r#"
        width = 256
        height = 64
        fps = 24.0
        duration = 0.0
        seed = 42
        
        [scene]
        name = "gradient"

        [scene.gradient]
        speed = 1.0
        palette = "sunset"
        "#;
        let config: RenderConfig = toml::from_str(toml_content).unwrap();
        assert!(config.validate().is_err());
    }

    #[test]
    fn invalid_zero_seed_returns_error() {
        let toml_content = r#"
        width = 256
        height = 64
        fps = 24.0
        duration = 5.0
        seed = 0
        
        [scene]
        name = "gradient"

        [scene.gradient]
        speed = 1.0
        palette = "sunset"
        "#;
        let config: RenderConfig = toml::from_str(toml_content).unwrap();
        assert!(config.validate().is_err());
    }

    #[test]
    fn invalid_empty_scene_returns_error() {
        let toml_content = r#"
        width = 256
        height = 64
        fps = 24.0
        duration = 5.0
        seed = 42

        [scene]
        name = ""

        [scene.gradient]
        speed = 1.0
        palette = "sunset"
        "#;
        let config: RenderConfig = toml::from_str(toml_content).unwrap();
        assert!(config.validate().is_err());
    }

    #[test]
    fn invalid_non_existent_file_returns_error() {
        let config = load_from_path(Path::new("../../examples/nonexistent.toml"));
        assert!(config.is_err());
    }

    #[test]
    fn valid_config_loads_from_file() {
        let config = load_from_path(Path::new("../../examples/sine_wave.toml")).unwrap();
        assert!(config.validate().is_ok());
    }
}
