use frameseed_core::{RenderContext, welcome_message, Frame, Rgba, frame_path, RenderConfig};
use std::{error::Error, path::Path};

fn main() -> Result<(), Box<dyn Error>> {
    println!("{}", welcome_message());

    std::fs::create_dir_all("output/sequence")?;


    let bada55 = Rgba::new(189, 165, 85, 255);

    let config = RenderConfig::new(256, 64, 24.0, 5.0, 42, "sine_wave");

    for i in 0..config.total_frames() {
        let ctx = RenderContext::new(i, config.total_frames(), config.fps, config.seed);
        let mut frame = Frame::new(config.width, config.height);
        frame.fill_sine_wave(bada55, ctx.normalized_time, 10.0);
        frame.save_png(&frame_path(Path::new("output/sequence"), i + 1))?;
    }

    Ok(())
}
