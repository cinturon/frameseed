use frameseed_core::{render_sequence, RenderConfig, RenderError};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};

use crate::{encode_gif, encode_png_sequence, FfmpegError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportFormat {
    Mp4,
    Gif,
}

impl ExportFormat {
    pub fn parse(value: &str) -> Result<Self, ExportError> {
        match value.to_lowercase().as_str() {
            "mp4" => Ok(Self::Mp4),
            "gif" => Ok(Self::Gif),
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
                "Unsupported export format '{value}'. Use `mp4` or `gif`."
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

pub fn export_video<F>(
    config: &RenderConfig,
    job_dir: &Path,
    output_path: &Path,
    format: ExportFormat,
    on_progress: F,
) -> Result<PathBuf, ExportError>
where
    F: FnMut(&str, u32, u32) + Send,
{
    let sequence_dir = job_dir.join("sequence");
    std::fs::create_dir_all(job_dir)?;

    let on_progress = std::sync::Mutex::new(on_progress);
    render_sequence(config, &sequence_dir, |current, total| {
        if let Ok(mut cb) = on_progress.lock() {
            cb("rendering", current, total);
        }
    })?;
    let mut on_progress = on_progress.into_inner().unwrap();

    on_progress("encoding", config.total_frames(), config.total_frames());

    if let Some(parent) = output_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let frame_count = config.total_frames();
    let pattern = sequence_dir.join("frame_%06d.png");

    let encode_result = match format {
        ExportFormat::Mp4 => encode_png_sequence(
            &pattern,
            output_path,
            config.fps,
            1,
            frame_count,
        ),
        ExportFormat::Gif => encode_gif(
            &pattern,
            output_path,
            config.fps,
            1,
            frame_count,
        ),
    };

    encode_result?;
    Ok(output_path.to_path_buf())
}
