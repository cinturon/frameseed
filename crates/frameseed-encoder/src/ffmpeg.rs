use std::error::Error;
use std::fmt::{Display, Formatter};
use std::path::Path;
use std::process::Command;

#[derive(Debug)]
pub enum FfmpegError {
    NotInstalled,
    EncodeFailed { kind: &'static str },
    Io(std::io::Error),
}

impl Display for FfmpegError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FfmpegError::NotInstalled => write!(
                f,
                "FFmpeg is not installed or not on your PATH. Install it with `brew install ffmpeg` (macOS) or `apt install ffmpeg` (Linux), then try again."
            ),
            FfmpegError::EncodeFailed { kind } => write!(
                f,
                "FFmpeg failed while encoding the {kind}. Check that frame PNGs exist in the sequence folder and rerun the export."
            ),
            FfmpegError::Io(error) => write!(f, "Could not run FFmpeg: {error}"),
        }
    }
}

impl Error for FfmpegError {}

pub fn ffmpeg_exists() -> Result<(), FfmpegError> {
    let output = Command::new("ffmpeg").arg("-version").output();
    match output {
        Ok(status) if status.status.success() => Ok(()),
        Ok(_) | Err(_) => Err(FfmpegError::NotInstalled),
    }
}

pub fn encode_png_sequence(
    input_pattern: &Path,
    output_file: &Path,
    fps: f32,
    start_frame: u32,
    frame_count: u32,
) -> Result<(), FfmpegError> {
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
        .status()
        .map_err(FfmpegError::Io)?;

    if !status.success() {
        return Err(FfmpegError::EncodeFailed { kind: "MP4 video" });
    }
    Ok(())
}

pub fn encode_gif(
    input_pattern: &Path,
    output_file: &Path,
    fps: f32,
    start_frame: u32,
    frame_count: u32,
) -> Result<(), FfmpegError> {
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
        .status()
        .map_err(FfmpegError::Io)?;

    if !status.success() {
        return Err(FfmpegError::EncodeFailed {
            kind: "GIF animation",
        });
    }
    Ok(())
}
