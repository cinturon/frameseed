mod commands;
use clap::Parser;
use commands::Cli;
use commands::Commands;
use commands::OutputFormat;
use commands::PresetsCommands;
use frameseed_core::gallery_entries;
use frameseed_core::list_presets;
use frameseed_core::load_from_path;
use frameseed_core::preset_path;
use frameseed_core::save_preset;
use frameseed_encoder::create_contact_sheet;
use frameseed_encoder::{export_video, ExportFormat};
use std::error::Error;
use std::path::Path;
use std::time::Instant;

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Render {
            config,
            preset,
            output_format,
            contact_sheet,
            contact_sheet_step,
            contact_sheet_cols,
        } => {
            let config_path = match (config, preset) {
                (Some(path), None) => path,
                (None, Some(name)) => preset_path(&name),
                _ => return Err("Provide either --config or --preset".into()),
            };
            render(
                &config_path,
                output_format,
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
    }

    Ok(())
}

fn render(
    config_path: &Path,
    output_format: OutputFormat,
    contact_sheet: bool,
    contact_sheet_step: u32,
    contact_sheet_cols: u32,
) -> Result<(), Box<dyn Error>> {
    let config = load_from_path(config_path)?;
    let export_format = match output_format {
        OutputFormat::Mp4 => ExportFormat::Mp4,
        OutputFormat::Gif => ExportFormat::Gif,
    };

    let job_dir = Path::new("output");
    let total_start = Instant::now();

    let output_path = export_video(&config, job_dir, export_format, |phase, current, total| {
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
