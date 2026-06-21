use frameseed_core::{render_frames_parallel, RenderConfig, RenderError};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use crate::{FfmpegError, ffmpeg_exists};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportFormat {
    Mp4,
    Gif,
    WebM,
}

impl ExportFormat {
    pub fn parse(value: &str) -> Result<Self, ExportError> {
        match value.to_lowercase().as_str() {
            "mp4" => Ok(Self::Mp4),
            "gif" => Ok(Self::Gif),
            "webm" => Ok(Self::WebM),
            _ => Err(ExportError::InvalidFormat(value.to_string())),
        }
    }
}

#[derive(Debug)]
pub enum ExportError {
    Render(RenderError),
    Io(std::io::Error),
    InvalidFormat(String),
    Ffmpeg(FfmpegError),
}

impl Display for ExportError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ExportError::Render(e) => write!(f, "{e}"),
            ExportError::Io(e) => write!(f, "Could not write export files: {e}"),
            ExportError::InvalidFormat(value) => write!(
                f,
                "Unsupported export format '{value}'. Use `mp4`, `gif`, or `webm`."
            ),
            ExportError::Ffmpeg(error) => write!(f, "{error}"),
        }
    }
}

impl Error for ExportError {}

impl From<RenderError> for ExportError {
    fn from(value: RenderError) -> Self {
        ExportError::Render(value)
    }
}

impl From<std::io::Error> for ExportError {
    fn from(value: std::io::Error) -> Self {
        ExportError::Io(value)
    }
}

impl From<FfmpegError> for ExportError {
    fn from(value: FfmpegError) -> Self {
        ExportError::Ffmpeg(value)
    }
}

/// Export a rendered video by piping raw RGBA frames directly to ffmpeg stdin.
///
/// This avoids writing a PNG sequence to disk and eliminates the PNG
/// compression/decompression round-trip, making exports significantly faster.
/// All frames are rendered in parallel and held in memory; for very long
/// high-resolution clips this can be substantial (width × height × 4 × frames).
pub fn export_video<F>(
    config: &RenderConfig,
    _job_dir: &Path,
    output_path: &Path,
    format: ExportFormat,
    bitrate: Option<&str>,
    on_progress: F,
) -> Result<PathBuf, ExportError>
where
    F: FnMut(&str, u32, u32) + Send,
{
    ffmpeg_exists()?;

    if let Some(parent) = output_path.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)?;
        }
    }

    let on_progress = std::sync::Mutex::new(on_progress);

    // Render all frames in parallel into in-memory RGBA buffers.
    let buffers = render_frames_parallel(config, |current, total| {
        if let Ok(mut cb) = on_progress.lock() {
            cb("rendering", current, total);
        }
    })?;

    let total = config.total_frames();

    // Pipe raw RGBA frames to ffmpeg stdin.
    let size_arg = format!("{}x{}", config.width, config.height);
    let fps_str = config.fps.to_string();

    let mut cmd = Command::new("ffmpeg");
    cmd.arg("-y")
        .arg("-f").arg("rawvideo")
        .arg("-pixel_format").arg("rgba")
        .arg("-video_size").arg(&size_arg)
        .arg("-framerate").arg(&fps_str)
        .arg("-i").arg("pipe:0");

    match format {
        ExportFormat::Mp4 => {
            cmd.arg("-c:v").arg("libx264").arg("-pix_fmt").arg("yuv420p");
            if let Some(br) = bitrate {
                cmd.arg("-b:v").arg(br);
            }
        }
        ExportFormat::WebM => {
            cmd.arg("-c:v").arg("libvpx-vp9").arg("-pix_fmt").arg("yuv420p");
            if let Some(br) = bitrate {
                cmd.arg("-b:v").arg(br);
            }
        }
        ExportFormat::Gif => {
            let vf = format!(
                "fps={},split[s0][s1];[s0]palettegen[p];[s1][p]paletteuse",
                config.fps
            );
            cmd.arg("-vf").arg(vf).arg("-loop").arg("0");
        }
    }

    cmd.arg(output_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null());

    let mut child = cmd.spawn().map_err(FfmpegError::Io)?;
    let mut stdin = child.stdin.take().expect("stdin is piped");

    for (i, buf) in buffers.iter().enumerate() {
        stdin.write_all(buf).map_err(ExportError::Io)?;
        if let Ok(mut cb) = on_progress.lock() {
            cb("encoding", (i + 1) as u32, total);
        }
    }
    drop(stdin);

    let status = child.wait().map_err(FfmpegError::Io)?;
    if !status.success() {
        let kind = match format {
            ExportFormat::Mp4 => "MP4 video",
            ExportFormat::Gif => "GIF animation",
            ExportFormat::WebM => "WebM video",
        };
        return Err(FfmpegError::EncodeFailed { kind }.into());
    }

    Ok(output_path.to_path_buf())
}
