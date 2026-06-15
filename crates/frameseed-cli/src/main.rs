use frameseed_core::{Frame, Rgba, welcome_message};
use std::{error::Error, path::Path};

fn main() -> Result<(), Box<dyn Error>> {
    println!("{}", welcome_message());

    let deadbeef = Rgba::new(222, 173, 191, 255);
    let bada55 = Rgba::new(186, 218, 85, 255);

    let mut h = Frame::new(256, 64);
    h.fill_horizontal_gradient(deadbeef, bada55);
    h.save_png(Path::new("output/horizontal_gradient.png"))?;
    let mut v = Frame::new(256, 256);
    v.fill_vertical_gradient(Rgba::black(), Rgba::white());
    v.save_png(Path::new("output/vertical_gradient.png"))?;
    let mut r = Frame::new(256, 256);
    r.fill_radial_gradient(bada55, deadbeef);
    r.save_png(Path::new("output/radial_gradient.png"))?;

    Ok(())
}
