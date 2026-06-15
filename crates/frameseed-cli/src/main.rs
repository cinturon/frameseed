use frameseed_core::{Frame, Rgba, welcome_message, lerp_rgba};
use std::{error::Error, path::Path};

fn main() -> Result<(), Box<dyn Error>> {
    println!("{}", welcome_message());

    let mut frame = Frame::new(256, 64);

    let deadbeef = Rgba::new(222, 173, 191, 255);
    let bada55 = Rgba::new(186, 218, 85, 255);

    for y in 0..frame.height {
        for x in 0..frame.width {
            let t = x as f32 / (frame.width - 1) as f32;
            let color = lerp_rgba(deadbeef, bada55, t);
            frame.set_pixel(x, y, color);
        }
    }

    std::fs::create_dir_all("output")?;
    frame.save_png(Path::new("output/horizontal_gradient.png"))?;
    println!("wrote to output/horizontal_gradient.png");

    Ok(())
}
