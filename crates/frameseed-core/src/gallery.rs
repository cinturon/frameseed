
pub struct GalleryEntry{
    pub slug: &'static str,
    pub title: &'static str,
    pub description: &'static str,
}

pub fn gallery_entries() -> &'static [GalleryEntry] {
    &[
        GalleryEntry {
            slug: "gradient",
            title: "Gradient",
            description: "A gradient scene",
        },
        GalleryEntry {
            slug: "noise_clouds",
            title: "Noise Clouds",
            description: "A noise clouds scene",
        },
        GalleryEntry {
            slug: "nebula",
            title: "Particle Nebula",
            description: "high count, motion blur, dark bg",
        },
        GalleryEntry {
            slug: "conway",
            title: "Cellular Automata",
            description: "conway's game of life",
        },
        GalleryEntry {
            slug: "flow_field",
            title: "Flow Field",
            description: "A flow field scene",
        },
        GalleryEntry {
            slug: "sdf_shapes",
            title: "SDF Shapes",
            description: "A sdf shapes scene",
        },
        GalleryEntry {
            slug: "fractal_zoom",
            title: "Fractal Zoom",
            description: "A mandelbrot scene",
        },
        GalleryEntry {
            slug: "voronoi",
            title: "Voronoi",
            description: "A voronoi scene",
        },
        GalleryEntry {
            slug: "snow",
            title: "Snow",
            description: "A snow scene",
        },
        GalleryEntry {  
            slug: "rain",
            title: "Rain",
            description: "A rain scene",
        },
        GalleryEntry {
            slug: "vhs_gradient",
            title: "VHS Grid",
            description: "A VHS grid scene",
        },
        GalleryEntry {
            slug: "pixeled_sunset",
            title: "Pixelated Sunset",
            description: "A pixelated sunset scene",
        },
        GalleryEntry {
            slug: "capstone",
            title: "Capstone — Drift",
            description: "45s flow-field short with blur, palette, and VHS",
        },
        GalleryEntry {
            slug: "plasma",
            title: "Plasma",
            description: "classic demoscene plasma using layered sine waves",
        },
        GalleryEntry {
            slug: "lissajous",
            title: "Lissajous",
            description: "animated Lissajous curves with phosphor trail",
        },
        GalleryEntry {
            slug: "starfield",
            title: "Starfield",
            description: "3D starfield with depth-based brightness and size",
        },
        GalleryEntry {
            slug: "plasma_neon",
            title: "Plasma Neon",
            description: "high-contrast plasma with boosted saturation",
        },
        GalleryEntry {
            slug: "deep_space",
            title: "Deep Space",
            description: "dense starfield at high speed with darkened contrast",
        },
        GalleryEntry {
            slug: "ocean_gradient",
            title: "Ocean Gradient",
            description: "diagonal ocean palette gradient with slow drift",
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{effects_from_config, load_from_path, scene_from_config};
    use std::path::PathBuf;

    fn repo_preset_path(slug: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../presets")
            .join(format!("{slug}.toml"))
    }

    #[test]
    fn gallery_presets_load_and_resolve() {
        for entry in gallery_entries() {
            let path = repo_preset_path(entry.slug);
            assert!(
                path.exists(),
                "missing preset file for gallery slug '{}'",
                entry.slug
            );
            let config = load_from_path(&path)
                .unwrap_or_else(|e| panic!("failed to load preset '{}': {e}", entry.slug));
            scene_from_config(&config.scene)
                .unwrap_or_else(|e| panic!("failed to resolve scene for '{}': {e}", entry.slug));
            let _effects = effects_from_config(&config.effects);
        }
    }
}