use frameseed_core::{RenderContext, welcome_message, Frame, Rgba, frame_path};
use std::{error::Error, path::Path};
use frameseed_encoder::encode_png_sequence;

fn main() -> Result<(), Box<dyn Error>> {
    println!("{}", welcome_message());

    let fps = 24.0;
    let duration = 5.0;
    let total_frames = (duration * fps) as u32;
    let seed = 42;

    std::fs::create_dir_all("output/sequence")?;


    let deadbeef = Rgba::new(222, 173, 191, 255);

    for i in 0..total_frames {
        let ctx = RenderContext::new(i, total_frames, fps, seed);
        let mut frame = Frame::new(256, 64);
        
        frame.fill_sine_wave(deadbeef, ctx.normalized_time, 10.0);
        

        let file_path = frame_path(Path::new("output/sequence"), i + 1);
        frame.save_png(&file_path)?;
    }

    println!("Done. Rendered {} frames to output/sequence", total_frames);

    let sequence_dir = Path::new("output/sequence");
    let input_pattern = sequence_dir.join("frame_%06d.png");
    let output_path = Path::new("output/video.mp4");

    encode_png_sequence(&input_pattern, output_path, fps, 1, total_frames)?;

    println!("Done. Encoded {} frames to output/video.mp4", total_frames);

    Ok(())
}
