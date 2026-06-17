mod commands;
use clap::Parser;
use commands::Cli;
use commands::Commands;
use frameseed_core::Frame;
use frameseed_core::RenderContext;
use frameseed_core::Rgba;
use frameseed_core::frame_path;
use frameseed_core::load_from_path;
use std::error::Error;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Render { config } => {
            render(&config)?;
        }
    }

    Ok(())
}

fn render(config: &Path) -> Result<(), Box<dyn Error>> {
    let config = load_from_path(config)?;

    std::fs::create_dir_all("output/sequence")?;

    let bada55 = Rgba::new(189, 165, 85, 255);

    for i in 0..config.total_frames() {
        let ctx = RenderContext::new(i, config.total_frames(), config.fps, config.seed);
        let mut frame = Frame::new(config.width, config.height);
        frame.fill_sine_wave(bada55, ctx.normalized_time, 10.0);
        frame.save_png(&frame_path(Path::new("output/sequence"), i + 1))?;
    }

    Ok(())
}
