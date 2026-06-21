use crate::Frame;
use image::{RgbaImage};
use std::io::Cursor;
use base64::{engine::general_purpose::STANDARD, Engine};

fn frame_to_png_bytes(frame: &Frame) -> Result<Vec<u8>, image::ImageError> {
    let image = RgbaImage::from_raw(frame.width, frame.height, frame.as_raw_rgba().to_vec())
        .expect("pixel buffer size mismatch");
    let mut cursor = Cursor::new(Vec::new());
    image.write_to(&mut cursor, image::ImageFormat::Png)?;
    Ok(cursor.into_inner())
}

pub fn frame_to_base64(frame: &Frame) -> Result<String, image::ImageError>{
    let bytes = frame_to_png_bytes(frame)?;
    Ok(STANDARD.encode(bytes))
}