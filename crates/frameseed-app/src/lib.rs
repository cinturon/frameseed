use frameseed_core::{
    config_to_toml, effects_from_config, frame_to_base64, load_preset, scene_from_config, Frame,
    RenderConfig, RenderContext, Rgba,
};
use frameseed_encoder::{export_video, ExportError, ExportFormat};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Emitter, State};
use tauri_plugin_dialog::DialogExt;

#[derive(Deserialize)]
struct PreviewRequest {
    config: RenderConfig,
    frame_index: Option<u32>,
}

#[derive(Deserialize)]
struct ExportRenderRequest {
    config: RenderConfig,
    output_format: String,
    /// Target video bitrate for MP4 (e.g. "5M", "10M"). `None` lets ffmpeg choose.
    bitrate: Option<String>,
}

#[derive(Clone, Serialize)]
struct RenderProgressEvent {
    phase: String,
    current: u32,
    total: u32,
}

#[derive(Clone, Serialize)]
struct RenderCompleteEvent {
    output_path: String,
    job_dir: String,
}

#[derive(Clone, Serialize)]
struct RenderErrorEvent {
    message: String,
}

#[derive(Clone, Serialize)]
struct ExportCancelled;

fn user_message(error: impl std::fmt::Display) -> String {
    error.to_string()
}

fn export_error_message(error: ExportError) -> String {
    error.to_string()
}

struct RenderState {
    running: Arc<Mutex<bool>>,
}

impl RenderState {
    fn new() -> Self {
        Self {
            running: Arc::new(Mutex::new(false)),
        }
    }
}

fn render_job_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../output/renders")
}

fn next_job_dir(base: &Path) -> PathBuf {
    let stamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or(0);
    base.join(format!("render_{stamp}"))
}

fn finish_render(state: &RenderState, running: bool) {
    if let Ok(mut guard) = state.running.lock() {
        *guard = running;
    }
}

fn export_dialog(app: &AppHandle, format: ExportFormat) -> Result<Option<PathBuf>, String> {
    let (title, filter_name, extension, default_name) = match format {
        ExportFormat::Mp4 => ("Export MP4", "MP4 Video", "mp4", "frameseed.mp4"),
        ExportFormat::Gif => ("Export GIF", "GIF Animation", "gif", "frameseed.gif"),
        ExportFormat::WebM => ("Export WebM", "WebM Video", "webm", "frameseed.webm"),
    };

    let selection = app
        .dialog()
        .file()
        .set_title(title)
        .set_file_name(default_name)
        .add_filter(filter_name, &[extension])
        .blocking_save_file();

    Ok(selection.map(|path| path.into_path().map_err(|error| error.to_string())).transpose()?)
}

fn run_export_job(
    app: AppHandle,
    state: RenderState,
    config: RenderConfig,
    format: ExportFormat,
    output_path: PathBuf,
    bitrate: Option<String>,
) {
    std::thread::spawn(move || {
        let result = (|| -> Result<PathBuf, String> {
            config.validate().map_err(user_message)?;

            let base_dir = render_job_dir();
            std::fs::create_dir_all(&base_dir).map_err(|error| error.to_string())?;
            let job_dir = next_job_dir(&base_dir);
            std::fs::create_dir_all(&job_dir).map_err(|error| error.to_string())?;

            export_video(
                &config,
                &job_dir,
                &output_path,
                format,
                bitrate.as_deref(),
                |phase, current, total| {
                    let _ = app.emit(
                        "render-progress",
                        RenderProgressEvent {
                            phase: phase.to_string(),
                            current,
                            total,
                        },
                    );
                },
            )
            .map_err(export_error_message)
        })();

        match result {
            Ok(saved_path) => {
                let job_dir = saved_path
                    .parent()
                    .map(|path| path.display().to_string())
                    .unwrap_or_default();
                let _ = app.emit(
                    "render-complete",
                    RenderCompleteEvent {
                        output_path: saved_path.display().to_string(),
                        job_dir,
                    },
                );
            }
            Err(message) => {
                let _ = app.emit("render-error", RenderErrorEvent { message });
            }
        }

        finish_render(&state, false);
    });
}

fn begin_export(
    app: AppHandle,
    state: &RenderState,
    config: RenderConfig,
    format: ExportFormat,
    output_path: PathBuf,
    bitrate: Option<String>,
) -> Result<(), String> {
    {
        let mut running = state
            .running
            .lock()
            .map_err(|_| "Render queue lock poisoned".to_string())?;
        if *running {
            return Err("A render is already in progress".into());
        }
        *running = true;
    }

    run_export_job(
        app,
        RenderState {
            running: state.running.clone(),
        },
        config,
        format,
        output_path,
        bitrate,
    );
    Ok(())
}

#[tauri::command]
async fn export_render(
    app: AppHandle,
    state: State<'_, RenderState>,
    request: ExportRenderRequest,
) -> Result<(), String> {
    request.config.validate().map_err(user_message)?;
    let format = ExportFormat::parse(&request.output_format).map_err(user_message)?;

    // blocking_save_file dispatches to the main thread internally; running it
    // inside spawn_blocking keeps the main thread free to handle the dialog.
    let app_for_dialog = app.clone();
    let maybe_path = tauri::async_runtime::spawn_blocking(move || {
        export_dialog(&app_for_dialog, format)
    })
    .await
    .map_err(|e| e.to_string())??;

    let Some(output_path) = maybe_path else {
        let _ = app.emit("export-cancelled", ExportCancelled);
        return Ok(());
    };

    begin_export(app, &state, request.config, format, output_path, request.bitrate)
}

#[tauri::command]
fn render_queue_running(state: State<'_, RenderState>) -> Result<bool, String> {
    let running = state
        .running
        .lock()
        .map_err(|_| "Render queue lock poisoned".to_string())?;
    Ok(*running)
}

#[tauri::command]
fn welcome() -> String {
    frameseed_core::welcome_message().to_string()
}

#[derive(Serialize)]
struct GalleryItem {
    slug: String,
    title: String,
}

#[tauri::command]
fn preset_config(preset: String) -> Result<RenderConfig, String> {
    load_preset(&preset).map_err(user_message)
}

#[tauri::command]
fn list_gallery() -> Vec<GalleryItem> {
    frameseed_core::gallery_entries()
        .iter()
        .map(|entry| GalleryItem {
            slug: entry.slug.to_string(),
            title: entry.title.to_string(),
        })
        .collect()
}

#[tauri::command]
fn preview_frame(request: PreviewRequest) -> Result<String, String> {
    request.config.validate().map_err(user_message)?;

    let config = request.config;

    let frame_index = request.frame_index.unwrap_or(0);
    let total_frames = config.total_frames();

    let scene = scene_from_config(&config.scene).map_err(user_message)?;
    let mut effects = effects_from_config(&config.effects);

    let mut frame = Frame::new(config.width, config.height);
    frame.clear(Rgba::black());

    let context = RenderContext::new(frame_index, total_frames, config.fps, config.seed);

    scene.render(&mut frame, &context);
    for effect in &mut effects {
        effect.apply(&mut frame, &context);
    }

    let base64 = frame_to_base64(&frame).map_err(user_message)?;
    Ok(base64)
}

#[tauri::command]
fn render_animation_preview(request: PreviewRequest) -> Result<Vec<String>, String> {
    request.config.validate().map_err(user_message)?;
    let config = request.config;
    let total = config.total_frames();
    let count = 30_u32.min(total);
    let scene = scene_from_config(&config.scene).map_err(user_message)?;
    let mut effects = effects_from_config(&config.effects);
    let mut frames = Vec::with_capacity(count as usize);
    for i in 0..count {
        let frame_idx = if count <= 1 {
            0
        } else {
            (i as f32 / (count - 1) as f32 * (total - 1) as f32).round() as u32
        };
        let mut frame = Frame::new(config.width, config.height);
        frame.clear(Rgba::black());
        let ctx = RenderContext::new(frame_idx, total, config.fps, config.seed);
        scene.render(&mut frame, &ctx);
        for effect in &mut effects {
            effect.apply(&mut frame, &ctx);
        }
        frames.push(frame_to_base64(&frame).map_err(user_message)?);
    }
    Ok(frames)
}

#[tauri::command]
fn export_config_to_toml(config: RenderConfig) -> Result<String, String> {
    config_to_toml(&config)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            welcome,
            list_gallery,
            preset_config,
            preview_frame,
            export_render,
            render_queue_running,
            render_animation_preview,
            export_config_to_toml
        ])
        .manage(RenderState::new())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn preview_with_preset(preset: &str, seed: u64) -> String {
        let mut config = load_preset(preset).expect("load preset");
        config.seed = seed;
        preview_frame(PreviewRequest {
            config,
            frame_index: None,
        })
        .expect("preview")
    }

    #[test]
    fn gallery_lists_all_presets_with_slugs() {
        let items = list_gallery();
        assert!(!items.is_empty());
        assert_eq!(items[0].slug, "gradient");
        assert_eq!(items[0].title, "Gradient");
    }

    #[test]
    fn preset_config_loads_gallery_preset() {
        let config = preset_config("noise_clouds".into()).expect("preset config");
        assert_eq!(config.scene.name, "noise_clouds");
        assert_eq!(config.seed, 42);
    }

    #[test]
    fn preview_frame_changes_with_width_and_preset() {
        let mut narrow = load_preset("gradient").expect("load preset");
        narrow.width = 128;
        let small = preview_frame(PreviewRequest {
            config: narrow,
            frame_index: None,
        })
        .expect("small preview");

        let mut wide = load_preset("gradient").expect("load preset");
        wide.width = 256;
        let large = preview_frame(PreviewRequest {
            config: wide,
            frame_index: None,
        })
        .expect("large preview");

        let noise = preview_with_preset("noise_clouds", 42);

        assert!(!small.is_empty());
        assert_ne!(small, large, "width change should change preview");
        assert_ne!(small, noise, "preset change should change preview");
    }

    #[test]
    fn preview_frame_changes_with_seed() {
        let seed_42 = preview_with_preset("conway", 42);
        let seed_99 = preview_with_preset("conway", 99);
        assert_ne!(seed_42, seed_99, "seed change should change preview");
    }
}
