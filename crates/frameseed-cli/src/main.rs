mod commands;
use clap::Parser;
use commands::Cli;
use commands::Commands;
use commands::OutputFormat;
use commands::PresetsCommands;
use frameseed_core::Frame;
use frameseed_core::RenderContext;
use frameseed_core::Rgba;
use frameseed_core::effects_from_config;
use frameseed_core::frame_path;
use frameseed_core::list_presets;
use frameseed_core::load_from_path;
use frameseed_core::preset_path;
use frameseed_core::save_preset;
use frameseed_core::scene_from_config;
use frameseed_encoder::create_contact_sheet;
use frameseed_encoder::{encode_gif, encode_png_sequence};
use std::error::Error;
use std::path::Path;

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

    std::fs::create_dir_all("output/sequence")?;

    let scene = scene_from_config(&config.scene)?;
    let mut effects = effects_from_config(&config.effects);

    let total_start = std::time::Instant::now();
    let total_frame_count = config.total_frames();

    let mut frame = Frame::new(config.width, config.height);

    for i in 0..total_frame_count {
        let frame_start = std::time::Instant::now();
        frame.clear(Rgba::black());

        let ctx = RenderContext::new(i, config.total_frames(), config.fps, config.seed);
        scene.render(&mut frame, &ctx);

        for effect in &mut effects {
            effect.apply(&mut frame, &ctx);
        }

        frame.save_png(&frame_path(Path::new("output/sequence"), i + 1))?;
        let frame_duration = frame_start.elapsed().as_secs_f64() * 1000.0;
        eprintln!(
            "Frame {}/{}: {:.2} ms",
            i + 1,
            total_frame_count,
            frame_duration
        );
    }
    let total_duration = total_start.elapsed().as_secs_f64();
    let avg_frame_duration = total_duration * 1000.0 / total_frame_count as f64;
    eprintln!("------------------------------------------");
    eprintln!("Scene: {}", config.scene.name);
    eprintln!("Total frames: {}", total_frame_count);
    eprintln!("Total time: {:.2} seconds", total_duration);
    eprintln!(
        "Average: {:.1} ms/frame ({:.1} fps)",
        avg_frame_duration,
        total_frame_count as f64 / total_duration
    );
    eprintln!("------------------------------------------");

    let pattern = Path::new("output/sequence/frame_%06d.png");
    let frame_count = config.total_frames();

    match output_format {
        OutputFormat::Mp4 => encode_png_sequence(
            pattern,
            Path::new("output/video.mp4"),
            config.fps,
            1,
            frame_count,
        ),
        OutputFormat::Gif => encode_gif(
            pattern,
            Path::new("output/animation.gif"),
            config.fps,
            1,
            frame_count,
        ),
    }?;

    if contact_sheet {
        create_contact_sheet(
            Path::new("output/sequence"),
            Path::new("output/contact_sheet.png"),
            frame_count,
            config.fps,
            contact_sheet_step,
            contact_sheet_cols,
            128,
        )?;
        eprintln!(
            "Contact sheet created at {}",
            Path::new("output/contact_sheet.png").display()
        );
    }

    Ok(())
}
