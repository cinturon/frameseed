use frameseed_core::{
    effects_from_config, frame_to_base64, load_preset, scene_from_config, Frame, RenderConfig,
    RenderContext, Rgba,
};
use frameseed_encoder::{export_video, ExportFormat};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Emitter, State};

#[derive(Deserialize)]
struct PreviewRequest {
    config: RenderConfig,
    frame_index: Option<u32>,
}

#[derive(Deserialize)]
struct QueueRenderRequest {
    config: RenderConfig,
    output_format: String,
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

fn next_job_dir(base: &PathBuf) -> PathBuf {
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

fn run_export_job(app: AppHandle, state: RenderState, config: RenderConfig, format: String) {
    std::thread::spawn(move || {
        let result = (|| -> Result<PathBuf, String> {
            config.validate().map_err(|error| error.to_string())?;
            let export_format = ExportFormat::parse(&format).map_err(|error| error.to_string())?;

            let base_dir = render_job_dir();
            std::fs::create_dir_all(&base_dir).map_err(|error| error.to_string())?;
            let job_dir = next_job_dir(&base_dir);
            std::fs::create_dir_all(&job_dir).map_err(|error| error.to_string())?;

            let output_path = export_video(&config, &job_dir, export_format, |phase, current, total| {
                let _ = app.emit(
                    "render-progress",
                    RenderProgressEvent {
                        phase: phase.to_string(),
                        current,
                        total,
                    },
                );
            })
            .map_err(|error| error.to_string())?;

            Ok(output_path)
        })();

        match result {
            Ok(output_path) => {
                let job_dir = output_path
                    .parent()
                    .map(|path| path.display().to_string())
                    .unwrap_or_default();
                let _ = app.emit(
                    "render-complete",
                    RenderCompleteEvent {
                        output_path: output_path.display().to_string(),
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

#[tauri::command]
fn queue_render(
    app: AppHandle,
    state: State<'_, RenderState>,
    request: QueueRenderRequest,
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
        request.config,
        request.output_format,
    );
    Ok(())
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
    load_preset(&preset).map_err(|e| e.to_string())
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
    request.config.validate().map_err(|e| e.to_string())?;

    let config = request.config;

    let frame_index = request.frame_index.unwrap_or(0);
    let total_frames = config.total_frames();

    let scene = scene_from_config(&config.scene).map_err(|e| e.to_string())?;
    let mut effects = effects_from_config(&config.effects);

    let mut frame = Frame::new(config.width, config.height);
    frame.clear(Rgba::black());

    let context = RenderContext::new(frame_index, total_frames, config.fps, config.seed);

    scene.render(&mut frame, &context);
    for effect in &mut effects {
        effect.apply(&mut frame, &context);
    }

    let base64 = frame_to_base64(&frame).map_err(|e| e.to_string())?;
    Ok(base64)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            welcome,
            list_gallery,
            preset_config,
            preview_frame,
            queue_render,
            render_queue_running
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
    fn gallery_lists_twelve_presets_with_slugs() {
        let items = list_gallery();
        assert_eq!(items.len(), 12);
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
