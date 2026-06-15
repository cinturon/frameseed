use frameseed_core::{RenderContext, welcome_message, Frame, Rgba, frame_path};
use std::{error::Error, path::Path};

fn main() -> Result<(), Box<dyn Error>> {
    println!("{}", welcome_message());

    let fps = 24.0;
    let duration = 5.0;
    let total_frames = (duration * fps) as u32;
    let seed = 42;

    std::fs::create_dir_all("output/sequence")?;


    let deadbeef = Rgba::new(222, 173, 191, 255);
    let bada55 = Rgba::new(186, 218, 85, 255);

    for i in 0..total_frames {
        let ctx = RenderContext::new(i, total_frames, fps, seed);
        let mut frame = Frame::new(256, 64);
        
        frame.fill_sliding_horizontal_gradient(deadbeef, bada55, ctx.normalized_time);
        

        let file_path = frame_path(Path::new("output/sequence"), i + 1);
        frame.save_png(&file_path)?;
    }

    println!("Done. Rendered {} frames to output/sequence", total_frames);

    Ok(())
}
