use crate::Frame;
use image::{RgbaImage};
use std::io::Cursor;
use base64::{engine::general_purpose::STANDARD, Engine};

fn frame_to_png_bytes(frame: &Frame) -> Result<Vec<u8>, image::ImageError>{
    let mut image = RgbaImage::new(frame.width, frame.height);

    for y in 0..frame.height {
        for x in 0..frame.width {
            if let Some(pixel) = frame.get_pixel(x, y){
                image.put_pixel(x, y, image::Rgba([pixel.r, pixel.g, pixel.b, pixel.a]));
            }
        }
    }
    let mut cursor = Cursor::new(Vec::new());
    image.write_to(&mut cursor, image::ImageFormat::Png)?;
    Ok(cursor.into_inner())
}

pub fn frame_to_base64(frame: &Frame) -> Result<String, image::ImageError>{
    let bytes = frame_to_png_bytes(frame)?;
    Ok(STANDARD.encode(bytes))
}