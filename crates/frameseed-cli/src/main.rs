mod commands;
use clap::Parser;
use commands::Cli;
use commands::Commands;
use frameseed_core::Frame;
use frameseed_core::scene_from_config;
use frameseed_core::effects_from_config;
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
    let mut effects = effects_from_config(&config.effects);

    let total_start = std::time::Instant::now();
    let total_frame_count = config.total_frames();
    for i in 0..total_frame_count {
        let frame_start = std::time::Instant::now();
    
        let ctx = RenderContext::new(i, config.total_frames(), config.fps, config.seed);
        let mut frame = Frame::new(config.width, config.height);
        scene.render(&mut frame, &ctx);
        
        for effect in &mut effects {
            effect.apply(&mut frame, &ctx);
        }

        frame.save_png(&frame_path(Path::new("output/sequence"), i + 1))?;
        let frame_duration = frame_start.elapsed().as_secs_f64() * 1000.0;
        eprintln!("Frame {}/{}: {:.2} ms", i + 1, total_frame_count, frame_duration);
    }
    let total_duration = total_start.elapsed().as_secs_f64();
    let avg_frame_duration = total_duration * 1000.0 / total_frame_count as f64;
    eprintln!("------------------------------------------");
    eprintln!("Scene: {}", config.scene.name);
    eprintln!("Total frames: {}", total_frame_count);
    eprintln!("Total time: {:.2} seconds", total_duration); 
    eprintln!("Average: {:.1} ms/frame ({:.1} fps)", avg_frame_duration, total_frame_count as f64 / total_duration);
    eprintln!("------------------------------------------");
    Ok(())
}
