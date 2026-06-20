mod commands;
use clap::Parser;
use commands::Cli;
use commands::Commands;
use commands::OutputFormat;
use commands::PresetsCommands;
use frameseed_core::gallery_entries;
use frameseed_core::list_presets;
use frameseed_core::load_from_path;
use frameseed_core::load_preset;
use frameseed_core::preset_path;
use frameseed_core::render_preview_frame;
use frameseed_core::save_preset;
use frameseed_core::scene_from_config;
use frameseed_core::{EffectsConfig, RenderConfig, SceneConfig};
use frameseed_core::KNOWN_PALETTES;
use frameseed_encoder::create_contact_sheet;
use frameseed_encoder::{export_video, ExportFormat};
use std::error::Error;
use std::path::{Path, PathBuf};
use std::time::Instant;

fn parse_seed(s: &str) -> Result<u64, Box<dyn Error>> {
    if s.eq_ignore_ascii_case("random") {
        let seed = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.subsec_nanos() as u64 ^ d.as_secs())
            .unwrap_or(42)
            | 1;
        eprintln!("Seed: {seed}");
        Ok(seed)
    } else {
        let n: u64 = s.parse().map_err(|_| format!("Invalid seed '{s}': expected a positive integer or 'random'"))?;
        Ok(n)
    }
}

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {err}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Render {
            config,
            preset,
            seed,
            output_format,
            output,
            contact_sheet,
            contact_sheet_step,
            contact_sheet_cols,
        } => {
            let config_path = match (config, preset) {
                (Some(path), None) => path,
                (None, Some(name)) => preset_path(&name),
                _ => {
                    return Err(
                        "Provide either --config <path.toml> or --preset <name>. Run `frameseed list-presets` to see presets.".into(),
                    );
                }
            };
            let seed_override = seed.as_deref().map(parse_seed).transpose()?;
            render(
                &config_path,
                seed_override,
                output_format,
                output,
                contact_sheet,
                contact_sheet_step,
                contact_sheet_cols,
            )?;
        }
        Commands::Presets { command } => match command {
            PresetsCommands::List => {
                for name in list_presets()? {
                    println!("{name}");
                }
            }
            PresetsCommands::Save { name, config } => {
                let cfg = load_from_path(&config)?;
                save_preset(&name, &cfg)?;
                eprintln!("Preset saved: {name}");
            }
        },
        Commands::ListScenes => {
            for entry in gallery_entries() {
                println!("{:<20} {}", entry.title, entry.description);
                println!("  preset: {}", entry.slug);
            }
        }
        Commands::ListPresets => {
            for name in list_presets()? {
                println!("{name}");
            }
        }
        Commands::ListPalettes => {
            for name in KNOWN_PALETTES {
                println!("{name}");
            }
        }
        Commands::Preview {
            scene,
            preset,
            config,
            seed,
            output,
            width,
            height,
            frame_index,
        } => {
            let seed_u64 = parse_seed(&seed)?;
            let render_config = match (scene.as_deref(), preset.as_deref(), config.as_deref()) {
                (Some(scene_name), None, None) => {
                    preview_config_from_scene(scene_name, seed_u64, width, height)?
                }
                (None, Some(preset_name), None) => {
                    let mut cfg = load_preset(preset_name)?;
                    cfg.seed = seed_u64;
                    cfg
                }
                (None, None, Some(path)) => {
                    let mut cfg = load_from_path(path)?;
                    cfg.seed = seed_u64;
                    cfg
                }
                _ => {
                    return Err(
                        "Provide exactly one of --scene, --preset, or --config for preview.".into(),
                    );
                }
            };

            render_preview_frame(&render_config, frame_index, &output)?;
            eprintln!("Preview saved to {}", output.display());
        }
    }

    Ok(())
}

fn preview_config_from_scene(
    scene: &str,
    seed: u64,
    width: u32,
    height: u32,
) -> Result<RenderConfig, Box<dyn Error>> {
    let config = RenderConfig::new(
        width,
        height,
        24.0,
        1.0,
        seed,
        SceneConfig::with_name(scene),
        EffectsConfig::default(),
    );
    config.validate()?;
    scene_from_config(&config.scene)?;
    Ok(config)
}

fn render(
    config_path: &Path,
    seed_override: Option<u64>,
    output_format: OutputFormat,
    output_override: Option<PathBuf>,
    contact_sheet: bool,
    contact_sheet_step: u32,
    contact_sheet_cols: u32,
) -> Result<(), Box<dyn Error>> {
    let mut config = load_from_path(config_path)?;
    if let Some(seed) = seed_override {
        config.seed = seed;
    }
    let export_format = match output_format {
        OutputFormat::Mp4 => ExportFormat::Mp4,
        OutputFormat::Gif => ExportFormat::Gif,
    };

    let job_dir = Path::new("output");
    let total_start = Instant::now();

    let output_path = output_override.unwrap_or_else(|| match export_format {
        ExportFormat::Mp4 => job_dir.join("video.mp4"),
        ExportFormat::Gif => job_dir.join("animation.gif"),
    });

    let output_path = export_video(&config, job_dir, &output_path, export_format, |phase, current, total| {
        if phase == "rendering" {
            eprintln!("Frame {current}/{total}");
        } else if phase == "encoding" {
            eprintln!("Encoding...");
        }
    })?;

    let total_duration = total_start.elapsed().as_secs_f64();
    let frame_count = config.total_frames();
    eprintln!("------------------------------------------");
    eprintln!("Scene: {}", config.scene.name);
    eprintln!("Total frames: {frame_count}");
    eprintln!("Output: {}", output_path.display());
    eprintln!("Total time: {total_duration:.2} seconds");
    eprintln!("------------------------------------------");

    if contact_sheet {
        create_contact_sheet(
            &job_dir.join("sequence"),
            &job_dir.join("contact_sheet.png"),
            frame_count,
            config.fps,
            contact_sheet_step,
            contact_sheet_cols,
            128,
        )?;
        eprintln!(
            "Contact sheet created at {}",
            job_dir.join("contact_sheet.png").display()
        );
    }

    Ok(())
}
