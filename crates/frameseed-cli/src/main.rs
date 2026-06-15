use frameseed_core::{RenderContext, welcome_message, Frame, Rgba, frame_path};
use std::{error::Error, path::Path};

fn main() -> Result<(), Box<dyn Error>> {
    println!("{}", welcome_message());

    let fps = 24.0;
    let duration = 5.0;
    let total_frames = (duration * fps) as u32;
    let seed = 42;

    std::fs::create_dir_all("output/sequence")?;

    for i in 0..total_frames {
        let ctx = RenderContext::new(i, total_frames, fps, seed);
        let mut frame = Frame::new(256, 64);
        
        let gray = (ctx.normalized_time * 255.0) as u8;
        frame.fill_solid(Rgba::new(gray, gray, gray, 255));
        

        let file_path = frame_path(Path::new("output/sequence"), i + 1);
        frame.save_png(&file_path)?;
        println!("wrote to {}", file_path.display());
    }

    println!("Done. Rendered {} frames to output/sequence", total_frames);

    Ok(())
}
