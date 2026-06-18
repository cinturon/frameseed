mod commands;
use clap::Parser;
use commands::Cli;
use commands::Commands;
use frameseed_core::Frame;
use frameseed_core::scene_from_config;
use frameseed_core::RenderContext;
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

    let scene = scene_from_config(&config.scene)?;

    for i in 0..config.total_frames() {
        let ctx = RenderContext::new(i, config.total_frames(), config.fps, config.seed);
        let mut frame = Frame::new(config.width, config.height);
        scene.render(&mut frame, &ctx);
        frame.save_png(&frame_path(Path::new("output/sequence"), i + 1))?;
    }

    Ok(())
}
