use std::error::Error;
use std::path::Path;
use std::process::Command;

pub fn ffmpeg_exists() -> Result<(), Box<dyn Error>> {
    let status = Command::new("ffmpeg").arg("-version").output()?;
    if !status.status.success() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "FFmpeg not found. Install with: brew install ffmpeg or apt install ffmpeg",
        )));
    }
    Ok(())
}

pub fn encode_png_sequence(
    input_pattern: &Path,
    output_file: &Path,
    fps: f32,
    start_frame: u32,
    frame_count: u32,
) -> Result<(), Box<dyn Error>> {
    ffmpeg_exists()?;

    let status = Command::new("ffmpeg")
        .arg("-y")
        .arg("-framerate")
        .arg(fps.to_string())
        .arg("-start_number")
        .arg(start_frame.to_string())
        .arg("-i")
        .arg(input_pattern)
        .arg("-frames:v")
        .arg(frame_count.to_string())
        .arg("-c:v")
        .arg("libx264")
        .arg("-pix_fmt")
        .arg("yuv420p")
        .arg(output_file)
        .status()?;

    if !status.success() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to encode PNG sequence",
        )));
    }
    Ok(())
}

pub fn encode_gif(
    input_pattern: &Path,
    output_file: &Path,
    fps: f32,
    start_frame: u32,
    frame_count: u32,
) -> Result<(), Box<dyn Error>> {
    ffmpeg_exists()?;

    let status = Command::new("ffmpeg")
        .arg("-y")
        .arg("-framerate")
        .arg(fps.to_string())
        .arg("-start_number")
        .arg(start_frame.to_string())
        .arg("-i")
        .arg(input_pattern)
        .arg("-frames:v")
        .arg(frame_count.to_string())
        .arg("-vf")
        .arg(format!(
            "fps={},split[s0][s1];[s0]palettegen[p];[s1][p]paletteuse",
            fps
        ))
        .arg("-loop")
        .arg("0")
        .arg(output_file)
        .status()?;

    if !status.success() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to encode GIF",
        )));
    }
    Ok(())
}