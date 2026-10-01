use crate::config::{ConfigError, RenderConfig};
use crate::load_from_path;
use std::fs::create_dir_all;
use std::path::PathBuf;

/// Repo-root `presets/` directory, anchored from `frameseed-core`'s manifest path.
pub fn presets_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../presets")
}

pub fn preset_path(name: &str) -> PathBuf {
    presets_dir().join(format!("{name}.toml"))
}

pub fn load_preset(name: &str) -> Result<RenderConfig, ConfigError> {
    let path = preset_path(name);
    if !path.exists() {
        return Err(ConfigError::Invalid(format!(
            "Preset '{name}' not found. Run `frameseed list-presets` to see available presets."
        )));
    }
    load_from_path(&path)
}

pub fn list_presets() -> Result<Vec<String>, ConfigError> {
    let mut names = Vec::new();
    for entry in std::fs::read_dir(presets_dir())? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().is_some_and(|ext| ext == "toml") {
            if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                names.push(stem.to_string());
            }
        }
    }
    names.sort();
    Ok(names)
}

pub fn save_preset(name: &str, config: &RenderConfig) -> Result<(), ConfigError> {
    create_dir_all(presets_dir())?;
    let path = preset_path(name);
    let contents =
        toml::to_string_pretty(config).map_err(|e| ConfigError::Invalid(e.to_string()))?;
    std::fs::write(path, contents)?;
    Ok(())
}
