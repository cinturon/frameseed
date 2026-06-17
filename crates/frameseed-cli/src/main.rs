use frameseed_core::{RenderContext, welcome_message, Frame, Rgba, frame_path, RenderConfig, load_from_path};
use std::{error::Error, path::Path};

fn main() -> Result<(), Box<dyn Error>> {
    println!("{}", welcome_message());

    std::fs::create_dir_all("output/sequence")?;


    let bada55 = Rgba::new(189, 165, 85, 255);

    let config_file = load_from_path(Path::new("./examples/sine_wave.toml"))?;

    let config = RenderConfig::new(config_file.width, config_file.height, config_file.fps, config_file.duration, config_file.seed, config_file.scene);

    for i in 0..config.total_frames() {
        let ctx = RenderContext::new(i, config.total_frames(), config.fps, config.seed);
        let mut frame = Frame::new(config.width, config.height);
        frame.fill_sine_wave(bada55, ctx.normalized_time, 10.0);
        frame.save_png(&frame_path(Path::new("output/sequence"), i + 1))?;
    }

    Ok(())
}
