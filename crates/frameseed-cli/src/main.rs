use frameseed_core::{Frame, Rgba, welcome_message};
use std::{error::Error, path::Path};

fn main() -> Result<(), Box<dyn Error>>{
    println!("{}", welcome_message());

    let mut frame = Frame::new(64, 64);
    let deadbeef = Rgba::new(222, 173, 191, 255);
    for y in 0..frame.height {
        for x in 0..frame.width {
            frame.set_pixel(x, y, deadbeef);
        }
    }

    std::fs::create_dir_all("output")?;
    frame.save_png(Path::new("output/first.png"))?;
    println!("wrote to output/first.png");

    Ok(())
}
